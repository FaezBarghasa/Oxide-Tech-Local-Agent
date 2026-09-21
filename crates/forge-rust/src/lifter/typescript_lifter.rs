use crate::detector::SourceLanguage;
use crate::ir::{
    UirField, UirFunction, UirItem, UirModule, UirParam, UirSelfKind, UirStmt, UirStruct, UirTrait,
    UirType,
};
use crate::lifter::{LanguageLifter, LifterError};
use regex::Regex;

pub struct TypeScriptLifter;

impl LanguageLifter for TypeScriptLifter {
    fn language(&self) -> SourceLanguage {
        SourceLanguage::TypeScript
    }

    fn lift(&self, source: &str, module_name: &str) -> Result<UirModule, LifterError> {
        let mut module = UirModule::new(module_name);

        // 1. Extract Interfaces: interface Foo { bar: string; baz?: number; }
        let interface_re = Regex::new(r"(?s)interface\s+(\w+)\s*\{([^}]*)\}").map_err(|e| {
            LifterError::ParseError {
                language: SourceLanguage::TypeScript,
                details: e.to_string(),
            }
        })?;

        for cap in interface_re.captures_iter(source) {
            let iface_name = cap
                .get(1)
                .map(|m| m.as_str())
                .unwrap_or("AnonymousInterface");
            let body = cap.get(2).map(|m| m.as_str()).unwrap_or("");

            let mut fields = Vec::new();
            let mut trait_methods = Vec::new();

            for line in body.lines() {
                let trimmed = line.trim().trim_end_matches(';').trim_end_matches(',');
                if trimmed.is_empty() || trimmed.starts_with("//") {
                    continue;
                }

                // Check method signature in interface: foo(a: string): number;
                if let Some(paren_idx) = trimmed.find('(')
                    && let Some(colon_idx) = trimmed.rfind(':')
                    && colon_idx > paren_idx
                {
                    let fn_name = trimmed[..paren_idx].trim();
                    let ret_str = trimmed[colon_idx + 1..].trim();
                    trait_methods.push(UirFunction {
                        name: fn_name.to_string(),
                        doc: None,
                        is_pub: true,
                        is_async: ret_str.starts_with("Promise<"),
                        is_unsafe: false,
                        is_method: true,
                        struct_target: None,
                        self_kind: Some(UirSelfKind::Ref),
                        params: Vec::new(),
                        return_type: Some(map_ts_type(ret_str)),
                        body: Vec::new(),
                    });
                    continue;
                }

                // Field: name: type or name?: type
                let parts: Vec<&str> = trimmed.split(':').collect();
                if parts.len() >= 2 {
                    let field_name_raw = parts[0].trim();
                    let is_optional = field_name_raw.ends_with('?');
                    let field_name = field_name_raw.trim_end_matches('?');
                    let field_ty_str = parts[1..].join(":");
                    let mut ty = map_ts_type(field_ty_str.trim());
                    if is_optional {
                        ty = UirType::Option(Box::new(ty));
                    }

                    fields.push(UirField {
                        name: field_name.to_string(),
                        ty,
                        is_pub: true,
                        doc: None,
                    });
                }
            }

            if !trait_methods.is_empty() && fields.is_empty() {
                module.items.push(UirItem::Trait(UirTrait {
                    name: iface_name.to_string(),
                    doc: None,
                    is_pub: true,
                    methods: trait_methods,
                }));
            } else {
                module.items.push(UirItem::Struct(UirStruct {
                    name: iface_name.to_string(),
                    doc: None,
                    is_pub: true,
                    fields,
                    methods: Vec::new(),
                    derives: vec![
                        "Debug".into(),
                        "Clone".into(),
                        "Serialize".into(),
                        "Deserialize".into(),
                    ],
                }));
                module.required_dependencies.push("serde".into());
            }
        }

        // 2. Extract Functions: export async function foo(x: number): Promise<string> { ... }
        let fn_re = Regex::new(r"(?s)(?:export\s+)?(?:async\s+)?function\s+(\w+)\s*\(([^)]*)\)(?:\s*:\s*([^{]+))?\s*\{([^}]*)\}")
            .map_err(|e| LifterError::ParseError { language: SourceLanguage::TypeScript, details: e.to_string() })?;

        for cap in fn_re.captures_iter(source) {
            let fn_name = cap.get(1).map(|m| m.as_str()).unwrap_or("unknown");
            let args_str = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let ret_str = cap.get(3).map(|m| m.as_str().trim());
            let body_str = cap.get(4).map(|m| m.as_str()).unwrap_or("");

            let is_async = source.contains(&format!("async function {}", fn_name))
                || ret_str.is_some_and(|r| r.starts_with("Promise<"));
            if is_async {
                module.required_dependencies.push("tokio".into());
            }

            let mut params = Vec::new();
            for arg in args_str.split(',') {
                let arg_trimmed = arg.trim();
                if arg_trimmed.is_empty() {
                    continue;
                }
                let parts: Vec<&str> = arg_trimmed.split(':').collect();
                let param_name = parts[0].trim().trim_end_matches('?');
                let param_ty = if parts.len() > 1 {
                    map_ts_type(parts[1].trim())
                } else {
                    UirType::String
                };
                params.push(UirParam {
                    name: param_name.to_string(),
                    ty: param_ty,
                });
            }

            let return_type = ret_str.map(map_ts_type);

            module.items.push(UirItem::Function(UirFunction {
                name: fn_name.to_string(),
                doc: None,
                is_pub: true,
                is_async,
                is_unsafe: false,
                is_method: false,
                struct_target: None,
                self_kind: None,
                params,
                return_type,
                body: vec![UirStmt::Raw(format!(
                    "// Lifted TS function: {}",
                    body_str.trim().lines().next().unwrap_or("")
                ))],
            }));
        }

        Ok(module)
    }
}

fn map_ts_type(ty: &str) -> UirType {
    let clean = ty.trim().trim_end_matches(';').trim();
    if clean.starts_with("Promise<") && clean.ends_with('>') {
        let inner = &clean[8..clean.len() - 1];
        return map_ts_type(inner);
    }
    if let Some(inner) = clean.strip_suffix("[]") {
        return UirType::Vec(Box::new(map_ts_type(inner)));
    }
    if clean.starts_with("Array<") && clean.ends_with('>') {
        let inner = &clean[6..clean.len() - 1];
        return UirType::Vec(Box::new(map_ts_type(inner)));
    }
    if clean.starts_with("Record<") && clean.ends_with('>') {
        let inner = &clean[7..clean.len() - 1];
        let kv: Vec<&str> = inner.split(',').collect();
        if kv.len() == 2 {
            return UirType::HashMap {
                key: Box::new(map_ts_type(kv[0].trim())),
                value: Box::new(map_ts_type(kv[1].trim())),
            };
        }
    }

    match clean {
        "number" => UirType::F64,
        "string" => UirType::String,
        "boolean" => UirType::Bool,
        "void" => UirType::Void,
        "any" | "unknown" => UirType::Custom("serde_json::Value".to_string()),
        "Uint8Array" => UirType::Vec(Box::new(UirType::U8)),
        other => UirType::Custom(other.to_string()),
    }
}
