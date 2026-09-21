use crate::OxideError;
use std::panic::{AssertUnwindSafe, catch_unwind};

/// Execute a foreign function or native operation behind a panic-isolated boundary fence.
pub fn call_ffi_safe<F, R>(operation_name: &str, f: F) -> Result<R, OxideError>
where
    F: FnOnce() -> R,
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(res) => Ok(res),
        Err(panic_payload) => {
            let msg = if let Some(s) = panic_payload.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = panic_payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "Unknown native panic".to_string()
            };

            tracing::error!(
                target: "oxide_ffi",
                "Panicked during FFI boundary execution of '{}': {}",
                operation_name,
                msg
            );

            Err(OxideError::FFI(format!(
                "Native boundary fault in '{}': {}",
                operation_name, msg
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_ffi_safe_success() {
        let res = call_ffi_safe("matrix_multiply", || 42 * 2);
        assert_eq!(res.unwrap(), 84);
    }

    #[test]
    fn test_call_ffi_safe_panic_isolation() {
        let res = call_ffi_safe("unstable_cuda_op", || {
            panic!("CUDA out of memory in native library");
        });
        assert!(res.is_err());
        match res.unwrap_err() {
            OxideError::FFI(msg) => {
                assert!(msg.contains("CUDA out of memory"));
            }
            other => panic!("Unexpected error type: {:?}", other),
        }
    }
}
