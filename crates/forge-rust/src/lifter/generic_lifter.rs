use crate::detector::SourceLanguage;
use crate::ir::{UirFunction, UirItem, UirModule, UirParam, UirStmt, UirType};
use crate::lifter::{LanguageLifter, LifterError};
use regex::Regex;

pub struct GenericLifter;

impl LanguageLifter for GenericLifter {
    fn language(&self) -> SourceLanguage {
        SourceLanguage::Generic
    }

    fn lift(&self, source: &str, module_name: &str) -> Result<UirModule, LifterError> {
        let mut module = UirModule::new(module_name);

        // Extract generic function patterns: fn/function/def/sub/proc name(args)
        let fn_re = Regex::new(r"(?m)(?:function|def|func|fn|proc|sub)\s+(\w+)\s*\(([^)]*)\)")
            .map_err(|e| LifterError::ParseError {
                language: SourceLanguage::Generic,
                details: e.to_string(),
            })?;

        for cap in fn_re.captures_iter(source) {
            let fn_name = cap.get(1).map(|m| m.as_str()).unwrap_or("unknown");
            let args_str = cap.get(2).map(|m| m.as_str()).unwrap_or("");

            let mut params = Vec::new();
            for arg in args_str.split(',') {
                let trimmed = arg.trim();
                if trimmed.is_empty() {
                    continue;
                }
                params.push(UirParam {
                    name: trimmed.to_string(),
                    ty: UirType::String,
                });
            }

            module.items.push(UirItem::Function(UirFunction {
                name: fn_name.to_string(),
                doc: None,
                is_pub: true,
                is_async: false,
                is_unsafe: false,
                is_method: false,
                struct_target: None,
                self_kind: None,
                params,
                return_type: None,
                body: vec![UirStmt::Raw("// Generic lifted function body".to_string())],
            }));
        }

        if module.items.is_empty() {
            // Raw block preservation if no functions could be detected
            module.items.push(UirItem::RawBlock(format!(
                "// Preserved generic source\n/*\n{}\n*/",
                source.trim()
            )));
        }

        Ok(module)
    }
}
