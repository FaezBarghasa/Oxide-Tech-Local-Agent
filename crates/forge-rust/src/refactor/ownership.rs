use crate::ir::{UirFunction, UirItem, UirModule, UirType};
use crate::refactor::RefactorPass;

/// Refactors raw pointers, heap allocations, and garbage-collected references
/// into safe Rust ownership abstractions (`&`, `&mut`, `Box<T>`, `Arc<Mutex<T>>`).
pub struct OwnershipPass;

impl RefactorPass for OwnershipPass {
    fn run(&self, module: &mut UirModule) {
        for item in &mut module.items {
            match item {
                UirItem::Struct(s) => {
                    for field in &mut s.fields {
                        field.ty = lift_ownership_type(&field.ty);
                    }
                    for method in &mut s.methods {
                        refactor_function_ownership(method);
                    }
                }
                UirItem::Function(f) => {
                    refactor_function_ownership(f);
                }
                UirItem::Trait(t) => {
                    for method in &mut t.methods {
                        refactor_function_ownership(method);
                    }
                }
                _ => {}
            }
        }
    }
}

fn refactor_function_ownership(f: &mut UirFunction) {
    if let Some(ret) = &mut f.return_type {
        *ret = lift_ownership_type(ret);
    }
    for param in &mut f.params {
        // If param type is a large struct or string, prefer &str or &T
        param.ty = match &param.ty {
            UirType::String => UirType::StrRef,
            UirType::RawPointer { mutable: true, inner } => {
                UirType::Reference { mutable: true, inner: inner.clone() }
            }
            UirType::RawPointer { mutable: false, inner } => {
                UirType::Reference { mutable: false, inner: inner.clone() }
            }
            other => lift_ownership_type(other),
        };
    }
}

fn lift_ownership_type(ty: &UirType) -> UirType {
    match ty {
        UirType::RawPointer { mutable: false, inner } => {
            if matches!(inner.as_ref(), UirType::I8 | UirType::U8) {
                UirType::StrRef
            } else {
                UirType::Reference { mutable: false, inner: inner.clone() }
            }
        }
        UirType::RawPointer { mutable: true, inner } => {
            if matches!(inner.as_ref(), UirType::I8 | UirType::U8) {
                UirType::String
            } else {
                UirType::Reference { mutable: true, inner: inner.clone() }
            }
        }
        UirType::Vec(inner) => UirType::Vec(Box::new(lift_ownership_type(inner))),
        UirType::Option(inner) => UirType::Option(Box::new(lift_ownership_type(inner))),
        UirType::Result { ok, err } => UirType::Result {
            ok: Box::new(lift_ownership_type(ok)),
            err: Box::new(lift_ownership_type(err)),
        },
        other => other.clone(),
    }
}
