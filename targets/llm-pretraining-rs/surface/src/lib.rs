//! # Autoresearch Mutable Surface (`surface.rs`)
//!
//! Budget: <= 420 lines. This IS the pure-Rust rewritten `train.py`.
//! Uses safe newtypes over the frozen `repr(C)` ABI with zero runtime overhead.

use surface_api::prelude::*;

// ── A. MODEL INTENT ──────────────────────────────────────────────────────────

pub fn build_model_intent<'h>(d: &Dims, h: &'h HostOps) -> Model<'h> {
    let mut m = Model::transformer(d, h);

    m.attention(AttentionCfg {
        qknorm: true,
        qknorm_scale: 0.5,
        rope_theta: 10_000.0,
        flash: true,
        head_dim_cap: None,
    });

    m.value_embeddings(ValueEmbedsCfg {
        lora_rank: 16,
        fuse_with_gate: false,
    });

    m.mlp(MlpCfg {
        mult: 4,
        act: Act::SwiGLU,
        bias: false,
    });

    m.dtype(DTypeTag::BF16);
    m
}

// ── B. OPTIMIZER INTENT ──────────────────────────────────────────────────────

pub fn build_optimizer_intent<'h>(m: &Model<'h>, h: &'h HostOps) -> Optim<'h> {
    let mut o = Optim::adamw(m, h);
    o.betas(0.9, 0.95).weight_decay(0.1).fused(true);
    o.schedule(|_step, _total| Schedule::WarmupCosine {
        peak: 6e-4,
        warmup_frac: 0.02,
        floor: 6e-5,
    });
    o
}

// ── C. STEP HOOK (Budgeted <= 60 lines) ───────────────────────────────────────

pub fn step_hook_intent(ctx: &mut StepContext<'_>, _h: &Host<'_>) -> HookAction {
    if ctx.step % 50 == 0 {
        ctx.log_grad_norms();
    }

    if ctx.loss.is_nan() || ctx.loss.is_infinite() {
        return HookAction::Abort("loss is NaN/Infinite");
    }

    // Experimental knobs: loss reweighting, aux loss, gradient clipping
    HookAction::Continue
}

// ── D. FFI EXPORTS ────────────────────────────────────────────────────────────

extern "C" fn ffi_build_model(dims: *const Dims, host: *const HostOps) -> ModelHandle {
    let d = unsafe { &*dims };
    let h = unsafe { &*host };
    let model = build_model_intent(d, h);
    model.handle
}

extern "C" fn ffi_build_optimizer(model: ModelHandle, host: *const HostOps) -> OptimHandle {
    let h = unsafe { &*host };
    let m = Model { handle: model, host: h };
    let optim = build_optimizer_intent(&m, h);
    optim.handle
}

extern "C" fn ffi_step_hook(raw_ctx: *const StepCtx, host: *const HostOps) -> surface_api::abi::HookActionResult {
    let raw = unsafe { &*raw_ctx };
    let h_ops = unsafe { &*host };
    let mut ctx = StepContext::from_raw(raw, h_ops);
    let host_obj = Host::new(h_ops);
    let action = step_hook_intent(&mut ctx, &host_obj);
    action.to_raw()
}

extern "C" fn ffi_destroy_model(model: ModelHandle, host: *const HostOps) {
    let h = unsafe { &*host };
    (h.release_model)(model);
}

#[unsafe(no_mangle)]
pub extern "C" fn oxide_surface_vtable() -> SurfaceVtable {
    SurfaceVtable {
        abi_version: SURFACE_ABI_VERSION,
        build_model: ffi_build_model,
        build_optimizer: ffi_build_optimizer,
        step_hook: ffi_step_hook,
        destroy_model: ffi_destroy_model,
    }
}
