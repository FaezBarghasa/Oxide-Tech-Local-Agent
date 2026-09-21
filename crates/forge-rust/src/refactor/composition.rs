use crate::ir::{UirItem, UirModule};
use crate::refactor::RefactorPass;

/// Converts OOP class inheritance patterns and methods into idiomatic Rust struct `impl` blocks.
pub struct CompositionPass;

impl RefactorPass for CompositionPass {
    fn run(&self, module: &mut UirModule) {
        let mut candidates = Vec::new();

        for (idx, item) in module.items.iter().enumerate() {
            if let UirItem::Function(f) = item {
                let target_struct = f.struct_target.clone().or_else(|| {
                    if f.is_method {
                        f.name
                            .find('_')
                            .map(|under_idx| f.name[..under_idx].to_string())
                    } else {
                        None
                    }
                });

                if let Some(target) = target_struct {
                    let method_name = if f.struct_target.is_some() {
                        f.name.clone()
                    } else if let Some(under_idx) = f.name.find('_') {
                        f.name[under_idx + 1..].to_string()
                    } else {
                        f.name.clone()
                    };

                    let mut func = f.clone();
                    func.name = method_name;
                    candidates.push((idx, target, func));
                }
            }
        }

        let mut attached_indices = Vec::new();
        for (idx, target, func) in candidates {
            for item in &mut module.items {
                if let UirItem::Struct(s) = item
                    && s.name.eq_ignore_ascii_case(&target)
                {
                    s.methods.push(func.clone());
                    attached_indices.push(idx);
                    break;
                }
            }
        }

        // Remove only successfully attached functions from top-level module
        attached_indices.sort_unstable();
        attached_indices.dedup();
        for idx in attached_indices.into_iter().rev() {
            module.items.remove(idx);
        }
    }
}
