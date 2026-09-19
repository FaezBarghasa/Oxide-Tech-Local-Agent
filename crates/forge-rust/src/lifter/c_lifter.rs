use crate::detector::SourceLanguage;
use crate::ir::{UirField, UirFunction, UirItem, UirModule, UirParam, UirSelfKind, UirStmt, UirStruct, UirType};
use crate::lifter::{LanguageLifter, LifterError};
use regex::Regex;

pub struct CLifter;

impl LanguageLifter for CLifter {
    fn language(&self) -> SourceLanguage {
        SourceLanguage::C
    }

    fn lift(&self, source: &str, module_name: &str) -> Result<UirModule, LifterError> {
        let mut module = UirModule::new(module_name);
        
        // 1. Extract C/C++ Structs (e.g. typedef struct { ... } Name; or struct Name { ... };)
        let struct_re = Regex::new(r"(?s)(?:typedef\s+)?struct\s+(\w+)?\s*\{([^}]*)\}\s*(\w+)?;")
            .map_err(|e| LifterError::ParseError { language: SourceLanguage::C, details: e.to_string() })?;

        for cap in struct_re.captures_iter(source) {
            let name = cap.get(3).or_else(|| cap.get(1)).map(|m| m.as_str()).unwrap_or("AnonymousStruct");
            let body = cap.get(2).map(|m| m.as_str()).unwrap_or("");
            
            let mut fields = Vec::new();
            for line in body.lines() {
                let trimmed = line.trim().trim_end_matches(';');
                if trimmed.is_empty() || trimmed.starts_with("//") || trimmed.starts_with("/*") {
                    continue;
                }
                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if parts.len() >= 2 {
                    let field_name = parts[parts.len() - 1].trim_start_matches('*');
                    let field_type_str = parts[..parts.len() - 1].join(" ");
                    let is_ptr = parts[parts.len() - 1].starts_with('*') || field_type_str.ends_with('*');
                    
                    let ty = map_c_type(&field_type_str, is_ptr);
                    fields.push(UirField {
                        name: field_name.to_string(),
                        ty,
                        is_pub: true,
                        doc: None,
                    });
                }
            }

            module.items.push(UirItem::Struct(UirStruct {
                name: name.to_string(),
                doc: None,
                is_pub: true,
                fields,
                methods: Vec::new(),
                derives: vec!["Debug".into(), "Clone".into(), "PartialEq".into()],
            }));
        }

        // 2. Extract Functions: return_type func_name(args) { body }
        let fn_re = Regex::new(r"(?s)(?:static\s+)?([\w\*]+)\s+(\w+)\s*\(([^)]*)\)\s*\{([^}]*)\}")
            .map_err(|e| LifterError::ParseError { language: SourceLanguage::C, details: e.to_string() })?;

        for cap in fn_re.captures_iter(source) {
            let ret_ty_str = cap.get(1).map(|m| m.as_str()).unwrap_or("void");
            let fn_name = cap.get(2).map(|m| m.as_str()).unwrap_or("unknown");
            let args_str = cap.get(3).map(|m| m.as_str()).unwrap_or("");
            let body_str = cap.get(4).map(|m| m.as_str()).unwrap_or("");

            if fn_name == "struct" || fn_name == "typedef" || fn_name == "if" || fn_name == "while" || fn_name == "for" {
                continue;
            }

            let mut params = Vec::new();
            let mut self_kind = None;
            let mut is_method = false;
            let mut struct_target = None;

            if !args_str.trim().is_empty() && args_str.trim() != "void" {
                for arg in args_str.split(',') {
                    let arg_trimmed = arg.trim();
                    let parts: Vec<&str> = arg_trimmed.split_whitespace().collect();
                    if parts.len() >= 2 {
                        let param_name = parts[parts.len() - 1].trim_start_matches('*');
                        let param_ty_str = parts[..parts.len() - 1].join(" ");
                        let is_ptr = parts[parts.len() - 1].starts_with('*') || param_ty_str.ends_with('*');

                        if param_name == "self" || param_name == "this" {
                            is_method = true;
                            self_kind = if is_ptr { Some(UirSelfKind::MutRef) } else { Some(UirSelfKind::Ref) };
                            struct_target = Some(param_ty_str.trim_end_matches('*').trim().to_string());
                        } else {
                            params.push(UirParam {
                                name: param_name.to_string(),
                                ty: map_c_type(&param_ty_str, is_ptr),
                            });
                        }
                    }
                }
            }

            let ret_ty = if ret_ty_str == "void" {
                None
            } else {
                let is_ptr = ret_ty_str.contains('*');
                Some(map_c_type(ret_ty_str.trim_end_matches('*'), is_ptr))
            };

            let mut stmts = Vec::new();
            for line in body_str.lines() {
                let l = line.trim();
                if l.is_empty() || l.starts_with("//") {
                    continue;
                }
                stmts.push(UirStmt::Raw(l.trim_end_matches(';').to_string()));
            }

            module.items.push(UirItem::Function(UirFunction {
                name: fn_name.to_string(),
                doc: None,
                is_pub: true,
                is_async: false,
                is_unsafe: false,
                is_method,
                struct_target,
                self_kind,
                params,
                return_type: ret_ty,
                body: stmts,
            }));
        }

        Ok(module)
    }
}

fn map_c_type(ty: &str, is_ptr: bool) -> UirType {
    let clean = ty.trim().trim_end_matches('*').trim();
    let base = match clean {
        "void" => UirType::Void,
        "bool" | "_Bool" => UirType::Bool,
        "char" | "int8_t" => UirType::I8,
        "uint8_t" | "unsigned char" => UirType::U8,
        "short" | "int16_t" => UirType::I16,
        "uint16_t" | "unsigned short" => UirType::U16,
        "int" | "int32_t" | "long" => UirType::I32,
        "uint32_t" | "unsigned int" | "unsigned long" => UirType::U32,
        "int64_t" | "long long" => UirType::I64,
        "uint64_t" | "unsigned long long" | "size_t" => UirType::USize,
        "float" => UirType::F32,
        "double" => UirType::F64,
        "char*" => UirType::String,
        other => UirType::Custom(other.to_string()),
    };

    if is_ptr {
        match base {
            UirType::I8 | UirType::U8 => UirType::String,
            UirType::Void => UirType::RawPointer { mutable: true, inner: Box::new(UirType::U8) },
            other => UirType::Reference { mutable: true, inner: Box::new(other) },
        }
    } else {
        base
    }
}
