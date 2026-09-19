//! # Dynamic Hot-Swap Loader with `catch_unwind` Fault Isolation
//!
//! Loads the candidate `cdylib`, verifies ABI version, and wraps all surface
//! callbacks in `catch_unwind` to prevent panics from terminating the harness.

use libloading::{Library, Symbol};
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;
use surface_api::abi::*;
use thiserror::Error;
use tracing::{error, info};

#[derive(Debug, Error)]
pub enum LoaderError {
    #[error("Failed to load cdylib library: {0}")]
    LoadFailed(#[from] libloading::Error),
    #[error("ABI Version mismatch: candidate v{candidate}, harness expects v{expected}")]
    AbiMismatch { candidate: u32, expected: u32 },
    #[error("Surface panicked during execution: {0}")]
    SurfacePanic(String),
}

pub struct SurfaceLoader {
    _lib: Library,
    pub vtable: SurfaceVtable,
}

impl SurfaceLoader {
    /// Load a candidate cdylib and verify ABI
    pub fn load(cdylib_path: &Path) -> Result<Self, LoaderError> {
        info!("Hot-swap loading candidate cdylib: {:?}", cdylib_path);
        let lib = unsafe { Library::new(cdylib_path)? };

        let vtable_fn: Symbol<extern "C" fn() -> SurfaceVtable> =
            unsafe { lib.get(b"oxide_surface_vtable")? };
        let vtable = vtable_fn();

        if vtable.abi_version != SURFACE_ABI_VERSION {
            return Err(LoaderError::AbiMismatch {
                candidate: vtable.abi_version,
                expected: SURFACE_ABI_VERSION,
            });
        }

        info!(
            "Successfully linked candidate surface cdylib (ABI v{})",
            vtable.abi_version
        );
        Ok(Self { _lib: lib, vtable })
    }

    /// Safely build model with `catch_unwind`
    pub fn safe_build_model(
        &self,
        dims: &Dims,
        host: &HostOps,
    ) -> Result<ModelHandle, LoaderError> {
        let result = catch_unwind(AssertUnwindSafe(|| {
            (self.vtable.build_model)(dims as *const Dims, host as *const HostOps)
        }));

        match result {
            Ok(handle) => Ok(handle),
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<&str>()
                    .map(|s| s.to_string())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "Unknown panic in build_model".to_string());
                error!("Surface build_model panicked: {}", msg);
                Err(LoaderError::SurfacePanic(msg))
            }
        }
    }

    /// Safely execute step hook with `catch_unwind`
    pub fn safe_step_hook(
        &self,
        ctx: &StepCtx,
        host: &HostOps,
    ) -> Result<HookActionResult, LoaderError> {
        let result = catch_unwind(AssertUnwindSafe(|| {
            (self.vtable.step_hook)(ctx as *const StepCtx, host as *const HostOps)
        }));

        match result {
            Ok(action) => Ok(action),
            Err(payload) => {
                let msg = payload
                    .downcast_ref::<&str>()
                    .map(|s| s.to_string())
                    .or_else(|| payload.downcast_ref::<String>().cloned())
                    .unwrap_or_else(|| "Unknown panic in step_hook".to_string());
                error!("Surface step_hook panicked: {}", msg);
                Err(LoaderError::SurfacePanic(msg))
            }
        }
    }
}
