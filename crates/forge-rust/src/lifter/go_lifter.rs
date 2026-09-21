use crate::detector::SourceLanguage;
use crate::ir::{
    UirField, UirFunction, UirItem, UirModule, UirParam, UirSelfKind, UirStmt, UirStruct, UirTrait,
    UirType,
};
use crate::lifter::{LanguageLifter, LifterError};
use regex::Regex;

pub struct GoLifter;

impl LanguageLifter for GoLifter {
    fn language(&self) -> SourceLanguage {
        SourceLanguage::Go
    }

    fn lift(&self, source: &str, module_name: &str) -> Result<UirModule, LifterError> {
        let mut module = UirModule::new(module_name);

        // 1. Extract Go Structs & Interfaces
        // type StructName struct { Field Type }
        let type_re =
            Regex::new(r"(?s)type\s+(\w+)\s+(struct|interface)\s*\{([^}]*)\}").map_err(|e| {
                LifterError::ParseError {
                    language: SourceLanguage::Go,
                    details: e.to_string(),
                }
            })?;

        for cap in type_re.captures_iter(source) {
            let name = cap.get(1).map(|m| m.as_str()).unwrap_or("Anonymous");
            let kind = cap.get(2).map(|m| m.as_str()).unwrap_or("struct");
            let body = cap.get(3).map(|m| m.as_str()).unwrap_or("");

            if kind == "struct" {
                let mut fields = Vec::new();
                for line in body.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with("//") {
                        continue;
                    }
                    let parts: Vec<&str> = trimmed.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let field_name = parts[0];
                        let field_ty_str = parts[1];
                        fields.push(UirField {
                            name: field_name.to_string(),
                            ty: map_go_type(field_ty_str),
                            is_pub: field_name.chars().next().is_some_and(|c| c.is_uppercase()),
                            doc: None,
                        });
                    }
                }

                module.items.push(UirItem::Struct(UirStruct {
                    name: name.to_string(),
                    doc: None,
                    is_pub: name.chars().next().is_some_and(|c| c.is_uppercase()),
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
            } else {
                // Interface
                let mut trait_methods = Vec::new();
                for line in body.lines() {
                    let trimmed = line.trim();
                    if trimmed.is_empty() || trimmed.starts_with("//") {
                        continue;
                    }
                    if let Some(paren_idx) = trimmed.find('(') {
                        let fn_name = trimmed[..paren_idx].trim();
                        let rest = trimmed[paren_idx..].trim();
                        let ret_ty = if let Some(close_idx) = rest.rfind(')') {
                            let ret_part = rest[close_idx + 1..].trim();
                            if ret_part.is_empty() {
                                None
                            } else {
                                Some(map_go_type(ret_part))
                            }
                        } else {
                            None
                        };

                        trait_methods.push(UirFunction {
                            name: fn_name.to_string(),
                            doc: None,
                            is_pub: true,
                            is_async: false,
                            is_unsafe: false,
                            is_method: true,
                            struct_target: None,
                            self_kind: Some(UirSelfKind::Ref),
                            params: Vec::new(),
                            return_type: ret_ty,
                            body: Vec::new(),
                        });
                    }
                }

                module.items.push(UirItem::Trait(UirTrait {
                    name: name.to_string(),
                    doc: None,
                    is_pub: name.chars().next().is_some_and(|c| c.is_uppercase()),
                    methods: trait_methods,
                }));
            }
        }

        // 2. Extract Go Functions: func (recv *Receiver) FuncName(arg1 Type) (RetType, error) { ... }
        let fn_re =
            Regex::new(r"(?s)func\s+(?:\(([^)]+)\)\s+)?(\w+)\s*\(([^)]*)\)\s*([^{]*)\{([^}]*)\}")
                .map_err(|e| LifterError::ParseError {
                language: SourceLanguage::Go,
                details: e.to_string(),
            })?;

        for cap in fn_re.captures_iter(source) {
            let recv_str = cap.get(1).map(|m| m.as_str());
            let fn_name = cap.get(2).map(|m| m.as_str()).unwrap_or("unknown");
            let args_str = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let ret_str = cap.get(4).map(|m| m.as_str().trim()).unwrap_or("");
            let body_str = cap.get(5).map(|m| m.as_str()).unwrap_or("");

            let mut is_method = false;
            let mut self_kind = None;
            let mut struct_target = None;

            if let Some(recv) = recv_str {
                is_method = true;
                self_kind = if recv.contains('*') {
                    Some(UirSelfKind::MutRef)
                } else {
                    Some(UirSelfKind::Ref)
                };
                let parts: Vec<&str> = recv.split_whitespace().collect();
                if parts.len() >= 2 {
                    struct_target = Some(parts[1].trim_start_matches('*').to_string());
                } else if parts.len() == 1 {
                    struct_target = Some(parts[0].trim_start_matches('*').to_string());
                }
            }

            let mut params = Vec::new();
            for arg in args_str.split(',') {
                let trimmed = arg.trim();
                if trimmed.is_empty() {
                    continue;
                }
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let param_name = parts[0];
                    let param_ty = map_go_type(parts[1]);
                    params.push(UirParam {
                        name: param_name.to_string(),
                        ty: param_ty,
                    });
                }
            }

            // Convert Go error returns: (Type, error) -> Result<Type, anyhow::Error>
            let return_type = if ret_str.is_empty() {
                None
            } else if ret_str == "error" {
                Some(UirType::Result {
                    ok: Box::new(UirType::Void),
                    err: Box::new(UirType::Custom("anyhow::Error".into())),
                })
            } else if ret_str.contains("error") {
                let cleaned = ret_str.trim_start_matches('(').trim_end_matches(')');
                let types: Vec<&str> = cleaned.split(',').collect();
                if types.len() == 2 && types[1].trim() == "error" {
                    Some(UirType::Result {
                        ok: Box::new(map_go_type(types[0].trim())),
                        err: Box::new(UirType::Custom("anyhow::Error".into())),
                    })
                } else {
                    Some(map_go_type(cleaned))
                }
            } else {
                Some(map_go_type(ret_str))
            };

            let mut has_goroutine = false;
            if body_str.contains("go ") {
                has_goroutine = true;
                module.required_dependencies.push("tokio".into());
            }

            module.items.push(UirItem::Function(UirFunction {
                name: fn_name.to_string(),
                doc: None,
                is_pub: fn_name.chars().next().is_some_and(|c| c.is_uppercase()),
                is_async: has_goroutine,
                is_unsafe: false,
                is_method,
                struct_target,
                self_kind,
                params,
                return_type,
                body: vec![UirStmt::Raw("// Lifted Go function body".to_string())],
            }));
        }

        Ok(module)
    }
}

fn map_go_type(ty: &str) -> UirType {
    let clean = ty.trim();
    if let Some(inner) = clean.strip_prefix("[]") {
        return UirType::Vec(Box::new(map_go_type(inner)));
    }
    if let Some(inner) = clean.strip_prefix('*') {
        return UirType::Reference {
            mutable: true,
            inner: Box::new(map_go_type(inner)),
        };
    }
    if let Some(inner) = clean.strip_prefix("map[")
        && let Some(close_bracket) = inner.find(']')
    {
        let key = &inner[..close_bracket];
        let val = &inner[close_bracket + 1..];
        return UirType::HashMap {
            key: Box::new(map_go_type(key)),
            value: Box::new(map_go_type(val)),
        };
    }

    match clean {
        "int" => UirType::ISize,
        "int8" => UirType::I8,
        "int16" => UirType::I16,
        "int32" | "rune" => UirType::I32,
        "int64" => UirType::I64,
        "uint" => UirType::USize,
        "uint8" | "byte" => UirType::U8,
        "uint16" => UirType::U16,
        "uint32" => UirType::U32,
        "uint64" => UirType::U64,
        "float32" => UirType::F32,
        "float64" => UirType::F64,
        "string" => UirType::String,
        "bool" => UirType::Bool,
        other => UirType::Custom(other.to_string()),
    }
}
