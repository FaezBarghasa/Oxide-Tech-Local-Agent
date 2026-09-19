use crate::ir::{UirFunction, UirItem, UirModule, UirType};
use crate::refactor::RefactorPass;

/// Transforms nullable types, error integer codes (-1 / errno), and exceptions
/// into idiomatic Rust `Result<T, E>` and `Option<T>` types.
pub struct ErrorHandlingPass;

impl RefactorPass for ErrorHandlingPass {
    fn run(&self, module: &mut UirModule) {
        for item in &mut module.items {
            if let UirItem::Function(f) = item {
                refactor_function_errors(f, &mut module.required_dependencies);
            }
        }
    }
}

fn refactor_function_errors(f: &mut UirFunction, required_dependencies: &mut Vec<String>) {
    // If function name implies fallibility or returns an integer error code
    let name_lower = f.name.to_lowercase();
    let is_fallible_name = name_lower.starts_with("try_")
        || name_lower.starts_with("parse_")
        || name_lower.starts_with("read_")
        || name_lower.starts_with("write_")
        || name_lower.starts_with("open_")
        || name_lower.starts_with("connect_")
        || name_lower.starts_with("load_")
        || name_lower.starts_with("save_");

    if let Some(ret) = &f.return_type {
        match ret {
            UirType::I32 | UirType::I64 if is_fallible_name => {
                // Map C style int return error code to Result<(), anyhow::Error>
                f.return_type = Some(UirType::Result {
                    ok: Box::new(UirType::Void),
                    err: Box::new(UirType::Custom("anyhow::Error".into())),
                });
                if !required_dependencies.contains(&"anyhow".to_string()) {
                    required_dependencies.push("anyhow".to_string());
                }
            }
            UirType::Result { .. } if !required_dependencies.contains(&"anyhow".to_string()) => {
                required_dependencies.push("anyhow".to_string());
            }
            _ => {}
        }
    } else if is_fallible_name {
        f.return_type = Some(UirType::Result {
            ok: Box::new(UirType::Void),
            err: Box::new(UirType::Custom("anyhow::Error".into())),
        });
        if !required_dependencies.contains(&"anyhow".to_string()) {
            required_dependencies.push("anyhow".to_string());
        }
    }
}
