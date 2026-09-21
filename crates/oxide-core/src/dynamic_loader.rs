use crate::ffi_boundary::call_ffi_safe;
use crate::OxideError;
use libloading::{Library, Symbol};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type SkillFn = unsafe extern "C" fn(*const u8, usize, *mut u8, usize) -> i32;

/// Dynamic native shared library (`.so`) skill loader with memory residency management.
pub struct DynamicSkillLoader {
    loaded_libraries: Arc<RwLock<HashMap<String, Arc<Library>>>>,
}

impl Default for DynamicSkillLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl DynamicSkillLoader {
    pub fn new() -> Self {
        Self {
            loaded_libraries: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Load a native shared library into memory.
    pub async fn load_library<P: AsRef<Path>>(
        &self,
        name: &str,
        path: P,
    ) -> Result<(), OxideError> {
        let path_ref = path.as_ref();
        let lib = unsafe {
            Library::new(path_ref).map_err(|e| {
                OxideError::FFI(format!(
                    "Failed to load dynamic library '{}' from {}: {}",
                    name,
                    path_ref.display(),
                    e
                ))
            })?
        };

        let mut lock = self.loaded_libraries.write().await;
        lock.insert(name.to_string(), Arc::new(lib));
        tracing::info!(target: "oxide_skills", "Loaded dynamic native library '{}'", name);
        Ok(())
    }

    /// Execute a symbol from a loaded library behind a panic-isolated FFI boundary.
    pub async fn call_symbol(
        &self,
        lib_name: &str,
        symbol_name: &str,
        input_data: &[u8],
        output_buffer: &mut [u8],
    ) -> Result<i32, OxideError> {
        let lib = {
            let lock = self.loaded_libraries.read().await;
            lock.get(lib_name).cloned().ok_or_else(|| {
                OxideError::FFI(format!("Library '{}' is not loaded", lib_name))
            })?
        };

        let sym_name_bytes = symbol_name.as_bytes();

        call_ffi_safe(symbol_name, move || {
            let func: Symbol<SkillFn> = unsafe {
                lib.get(sym_name_bytes).map_err(|e| {
                    OxideError::FFI(format!("Symbol '{}' not found: {}", symbol_name, e))
                })?
            };

            let ret = unsafe {
                func(
                    input_data.as_ptr(),
                    input_data.len(),
                    output_buffer.as_mut_ptr(),
                    output_buffer.len(),
                )
            };

            Ok(ret)
        })?
    }

    /// Total active loaded dynamic libraries.
    pub async fn loaded_count(&self) -> usize {
        let lock = self.loaded_libraries.read().await;
        lock.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dynamic_loader_init() {
        let loader = DynamicSkillLoader::new();
        assert_eq!(loader.loaded_count().await, 0);

        // Attempting to call non-existent library returns typed FFI error
        let mut out = [0u8; 16];
        let res = loader.call_symbol("nonexistent", "run", b"test", &mut out).await;
        assert!(res.is_err());
    }
}
