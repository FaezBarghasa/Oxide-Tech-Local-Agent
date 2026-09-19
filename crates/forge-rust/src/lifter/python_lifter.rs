use crate::detector::SourceLanguage;
use crate::ir::{UirFunction, UirItem, UirModule, UirParam, UirSelfKind, UirStmt, UirStruct, UirType};
use crate::lifter::{LanguageLifter, LifterError};
use regex::Regex;

pub struct PythonLifter;

impl LanguageLifter for PythonLifter {
    fn language(&self) -> SourceLanguage {
        SourceLanguage::Python
    }

    fn lift(&self, source: &str, module_name: &str) -> Result<UirModule, LifterError> {
        let mut module = UirModule::new(module_name);

        // 1. Extract Python Classes
        let class_re = Regex::new(r"(?m)^class\s+(\w+)(?:\(([^)]*)\))?:")
            .map_err(|e| LifterError::ParseError { language: SourceLanguage::Python, details: e.to_string() })?;

        for cap in class_re.captures_iter(source) {
            let class_name = cap.get(1).map(|m| m.as_str()).unwrap_or("AnonymousClass");
            module.items.push(UirItem::Struct(UirStruct {
                name: class_name.to_string(),
                doc: None,
                is_pub: true,
                fields: Vec::new(),
                methods: Vec::new(),
                derives: vec!["Debug".into(), "Clone".into(), "Serialize".into(), "Deserialize".into()],
            }));
            module.required_dependencies.push("serde".into());
        }

        // 2. Extract Functions: def func_name(arg1: int, arg2: str) -> bool:
        let fn_re = Regex::new(r"(?m)^(?:async\s+)?def\s+(\w+)\s*\(([^)]*)\)(?:\s*->\s*([^:]+))?:")
            .map_err(|e| LifterError::ParseError { language: SourceLanguage::Python, details: e.to_string() })?;

        for cap in fn_re.captures_iter(source) {
            let fn_name = cap.get(1).map(|m| m.as_str()).unwrap_or("unknown");
            let args_str = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            let ret_str = cap.get(3).map(|m| m.as_str().trim());

            let is_async = source.contains(&format!("async def {}", fn_name));
            if is_async {
                module.required_dependencies.push("tokio".into());
            }

            let mut params = Vec::new();
            let mut self_kind = None;
            let mut is_method = false;

            for arg in args_str.split(',') {
                let arg_trimmed = arg.trim();
                if arg_trimmed.is_empty() {
                    continue;
                }
                if arg_trimmed == "self" {
                    is_method = true;
                    self_kind = Some(UirSelfKind::MutRef);
                    continue;
                }
                if arg_trimmed == "cls" {
                    continue;
                }

                let parts: Vec<&str> = arg_trimmed.split(':').collect();
                let param_name = parts[0].trim();
                let param_ty = if parts.len() > 1 {
                    map_py_type(parts[1].trim())
                } else {
                    UirType::String
                };

                params.push(UirParam {
                    name: param_name.to_string(),
                    ty: param_ty,
                });
            }

            let return_type = ret_str.map(map_py_type);

            module.items.push(UirItem::Function(UirFunction {
                name: fn_name.to_string(),
                doc: None,
                is_pub: true,
                is_async,
                is_unsafe: false,
                is_method,
                struct_target: None,
                self_kind,
                params,
                return_type,
                body: vec![UirStmt::Raw("// Lifted from Python body".to_string())],
            }));
        }

        Ok(module)
    }
}

fn map_py_type(ty: &str) -> UirType {
    let clean = ty.trim();
    if clean.starts_with("Optional[") && clean.ends_with(']') {
        let inner = &clean[9..clean.len() - 1];
        return UirType::Option(Box::new(map_py_type(inner)));
    }
    if (clean.starts_with("List[") || clean.starts_with("list[")) && clean.ends_with(']') {
        let inner = &clean[5..clean.len() - 1];
        return UirType::Vec(Box::new(map_py_type(inner)));
    }
    if (clean.starts_with("Dict[") || clean.starts_with("dict[")) && clean.ends_with(']') {
        let inner = &clean[5..clean.len() - 1];
        let kv: Vec<&str> = inner.split(',').collect();
        if kv.len() == 2 {
            return UirType::HashMap {
                key: Box::new(map_py_type(kv[0].trim())),
                value: Box::new(map_py_type(kv[1].trim())),
            };
        }
    }

    match clean {
        "int" => UirType::I64,
        "float" => UirType::F64,
        "str" => UirType::String,
        "bool" => UirType::Bool,
        "bytes" => UirType::Vec(Box::new(UirType::U8)),
        "None" => UirType::Void,
        other => UirType::Custom(other.to_string()),
    }
}
