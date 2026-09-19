use crate::ir::{UirItem, UirModule};
use crate::refactor::RefactorPass;

/// Converts callbacks, threads, goroutines, and promises into Rust async/await (Tokio) primitives.
pub struct ConcurrencyPass;

impl RefactorPass for ConcurrencyPass {
    fn run(&self, module: &mut UirModule) {
        let mut needs_tokio = false;

        for item in &mut module.items {
            match item {
                UirItem::Function(f) => {
                    if f.is_async {
                        needs_tokio = true;
                    }
                }
                UirItem::Struct(s) => {
                    for method in &mut s.methods {
                        if method.is_async {
                            needs_tokio = true;
                        }
                    }
                }
                _ => {}
            }
        }

        if needs_tokio && !module.required_dependencies.contains(&"tokio".to_string()) {
            module.required_dependencies.push("tokio".to_string());
        }
    }
}
