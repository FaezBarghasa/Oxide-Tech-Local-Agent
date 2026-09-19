//! # Safe Zero-Cost Convenience Wrappers over `HostOps`
//!
//! Exposes an idiomatic safe API without direct raw pointer manipulation.

use crate::abi::*;

pub struct Host<'a> {
    pub ops: &'a HostOps,
}

impl<'a> Host<'a> {
    pub fn new(ops: &'a HostOps) -> Self {
        Self { ops }
    }

    pub fn log_scalar(&self, name: &str, val: f64) {
        (self.ops.log_scalar)(name.as_ptr(), name.len(), val);
    }
}

pub struct Model<'h> {
    pub handle: ModelHandle,
    pub host: &'h HostOps,
}

impl<'h> Model<'h> {
    pub fn transformer(dims: &Dims, h: &'h HostOps) -> Self {
        let handle = (h.create_model)(dims);
        Self { handle, host: h }
    }

    pub fn attention(&mut self, _cfg: AttentionCfg) -> &mut Self {
        // Configure attention properties on host
        self
    }

    pub fn value_embeddings(&mut self, _cfg: ValueEmbedsCfg) -> &mut Self {
        self
    }

    pub fn mlp(&mut self, _cfg: MlpCfg) -> &mut Self {
        self
    }

    pub fn dtype(&mut self, tag: DTypeTag) -> &mut Self {
        (self.host.set_dtype)(self.handle, tag);
        self
    }
}

#[derive(Debug, Clone)]
pub struct AttentionCfg {
    pub qknorm: bool,
    pub qknorm_scale: f64,
    pub rope_theta: f64,
    pub flash: bool,
    pub head_dim_cap: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct ValueEmbedsCfg {
    pub lora_rank: u32,
    pub fuse_with_gate: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Act {
    SwiGLU,
    GELU,
    SiLU,
}

#[derive(Debug, Clone)]
pub struct MlpCfg {
    pub mult: u32,
    pub act: Act,
    pub bias: bool,
}

pub struct Optim<'h> {
    pub handle: OptimHandle,
    pub host: &'h HostOps,
}

impl<'h> Optim<'h> {
    pub fn adamw(m: &Model<'h>, h: &'h HostOps) -> Self {
        let handle = (h.create_adamw)(m.handle, 0.9, 0.95, 0.1, true);
        Self { handle, host: h }
    }

    pub fn betas(&mut self, _beta1: f64, _beta2: f64) -> &mut Self {
        self
    }

    pub fn weight_decay(&mut self, _wd: f64) -> &mut Self {
        self
    }

    pub fn fused(&mut self, _fused: bool) -> &mut Self {
        self
    }

    pub fn schedule<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(u64, u64) -> Schedule,
    {
        match f(0, 1000) {
            Schedule::WarmupCosine { peak, warmup_frac, floor } => {
                (self.host.set_schedule_cosine)(self.handle, peak, warmup_frac, floor);
            }
            Schedule::Constant(lr) => {
                (self.host.set_schedule_cosine)(self.handle, lr, 0.0, lr);
            }
        }
        self
    }
}

pub enum Schedule {
    WarmupCosine {
        peak: f64,
        warmup_frac: f64,
        floor: f64,
    },
    Constant(f64),
}

pub struct StepContext<'h> {
    pub raw: &'h StepCtx,
    pub host: &'h HostOps,
    pub step: u64,
    pub total_steps: u64,
    pub loss: f64,
    pub lr: f64,
}

impl<'h> StepContext<'h> {
    pub fn from_raw(raw: &'h StepCtx, host: &'h HostOps) -> Self {
        Self {
            raw,
            host,
            step: raw.step,
            total_steps: raw.total_steps,
            loss: raw.loss,
            lr: raw.lr,
        }
    }

    pub fn log_grad_norms(&self) {
        (self.host.log_grad_norms)(self.raw.model);
    }
}

pub enum HookAction {
    Continue,
    Abort(&'static str),
    Skip,
}

impl HookAction {
    pub fn to_raw(&self) -> HookActionResult {
        match self {
            HookAction::Continue => HookActionResult {
                tag: HookActionTag::Continue,
                msg_ptr: std::ptr::null(),
                msg_len: 0,
            },
            HookAction::Abort(msg) => HookActionResult {
                tag: HookActionTag::Abort,
                msg_ptr: msg.as_ptr(),
                msg_len: msg.len(),
            },
            HookAction::Skip => HookActionResult {
                tag: HookActionTag::Skip,
                msg_ptr: std::ptr::null(),
                msg_len: 0,
            },
        }
    }
}
