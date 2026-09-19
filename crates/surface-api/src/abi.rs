//! # `repr(C)` ABI Definition
//!
//! FROZEN. `repr(C)` ONLY. No candle. No generics. No heap allocations.
//! Host owns all tensor memory and candle types behind opaque handles.

pub const SURFACE_ABI_VERSION: u32 = 1;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelHandle(pub *mut ());

unsafe impl Send for ModelHandle {}
unsafe impl Sync for ModelHandle {}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TensorHandle(pub *mut ());

unsafe impl Send for TensorHandle {}
unsafe impl Sync for TensorHandle {}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OptimHandle(pub *mut ());

unsafe impl Send for OptimHandle {}
unsafe impl Sync for OptimHandle {}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DTypeTag {
    F32 = 0,
    F16 = 1,
    BF16 = 2,
    I64 = 3,
    I32 = 4,
    U8 = 5,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Dims {
    pub n_layer: u32,
    pub n_head: u32,
    pub n_embd: u32,
    pub n_ctx: u32,
    pub vocab: u32,
    pub dtype: DTypeTag,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StepCtx {
    pub step: u64,
    pub total_steps: u64,
    pub loss: f64,
    pub lr: f64,
    pub model: ModelHandle,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HookActionTag {
    Continue = 0,
    Abort = 1,
    Skip = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HookActionResult {
    pub tag: HookActionTag,
    pub msg_ptr: *const u8,
    pub msg_len: usize,
}

/// Host operations vtable provided to the mutable surface cdylib
#[repr(C)]
pub struct HostOps {
    pub create_model: extern "C" fn(*const Dims) -> ModelHandle,
    pub linear: extern "C" fn(TensorHandle, u32, u32, *const u8, usize, ModelHandle) -> TensorHandle,
    pub rmsnorm: extern "C" fn(TensorHandle, f64) -> TensorHandle,
    pub attention: extern "C" fn(TensorHandle, u32, bool, f64, bool) -> TensorHandle,
    pub mlp_swiglu: extern "C" fn(TensorHandle, u32, bool) -> TensorHandle,
    pub set_dtype: extern "C" fn(ModelHandle, DTypeTag) -> (),
    pub create_adamw: extern "C" fn(ModelHandle, f64, f64, f64, bool) -> OptimHandle,
    pub set_schedule_cosine: extern "C" fn(OptimHandle, f64, f64, f64) -> (),
    pub adamw_step: extern "C" fn(OptimHandle, f64) -> i32,
    pub log_scalar: extern "C" fn(*const u8, usize, f64) -> (),
    pub log_grad_norms: extern "C" fn(ModelHandle) -> (),
    pub release_tensor: extern "C" fn(TensorHandle) -> (),
    pub release_model: extern "C" fn(ModelHandle) -> (),
    pub release_optim: extern "C" fn(OptimHandle) -> (),
}

/// Vtable exported by the surface cdylib
#[repr(C)]
pub struct SurfaceVtable {
    pub abi_version: u32,
    pub build_model: extern "C" fn(*const Dims, *const HostOps) -> ModelHandle,
    pub build_optimizer: extern "C" fn(ModelHandle, *const HostOps) -> OptimHandle,
    pub step_hook: extern "C" fn(*const StepCtx, *const HostOps) -> HookActionResult,
    pub destroy_model: extern "C" fn(ModelHandle, *const HostOps) -> (),
}
