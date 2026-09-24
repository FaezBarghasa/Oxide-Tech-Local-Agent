//! # Mobile FFI & UniFFI Scaffolding
//!
//! Provides zero-overhead, thread-safe C-FFI and UniFFI bindings for embedding
//! the non-autoregressive `DecisionEngine` directly into iOS (Swift) and Android (Kotlin).

use crate::decision_engine::{
    DecisionDevice, DecisionEngine, DecisionInput, DecisionOutput, SpeculativeCascadeConfig,
};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Arc;

/// Safe high-level handle for mobile platforms
#[derive(Clone)]
pub struct MobileDecisionEngine {
    inner: Arc<DecisionEngine>,
}

impl MobileDecisionEngine {
    pub fn new(max_batch_size: u32, timeout_ms: u64) -> Self {
        let engine = DecisionEngine::new(DecisionDevice::Cpu, max_batch_size as usize, timeout_ms);
        Self {
            inner: Arc::new(engine),
        }
    }

    pub fn decide(
        &self,
        state: String,
        criteria: String,
        candidates: Vec<String>,
    ) -> Result<DecisionOutput, String> {
        let input = DecisionInput {
            state,
            criteria,
            candidates,
        };
        self.inner.decide(input)
    }

    pub fn decide_with_cascade(
        &self,
        state: String,
        criteria: String,
        candidates: Vec<String>,
        confidence_spread_threshold: f32,
        min_confidence_threshold: f32,
    ) -> Result<DecisionOutput, String> {
        let input = DecisionInput {
            state,
            criteria,
            candidates,
        };
        let cfg = SpeculativeCascadeConfig {
            confidence_spread_threshold,
            min_confidence_threshold,
            fallback_enabled: true,
        };
        self.inner.decide_with_cascade(input, cfg)
    }
}

// --- C-ABI Exported Functions for Swift / Kotlin Native JNI / FFI ---

/// Allocate and initialize a new `DecisionEngine` on the heap.
///
/// # Safety
/// Caller must free using `oxide_decision_engine_free`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxide_decision_engine_create(
    max_batch_size: u32,
    timeout_ms: u64,
) -> *mut DecisionEngine {
    let engine = DecisionEngine::new(DecisionDevice::Cpu, max_batch_size as usize, timeout_ms);
    Box::into_raw(Box::new(engine))
}

/// Free a `DecisionEngine` instance allocated by `oxide_decision_engine_create`.
///
/// # Safety
/// `engine_ptr` must be a valid pointer created by `oxide_decision_engine_create` or NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxide_decision_engine_free(engine_ptr: *mut DecisionEngine) {
    if !engine_ptr.is_null() {
        unsafe {
            drop(Box::from_raw(engine_ptr));
        }
    }
}

/// Execute a synchronous non-autoregressive decision pass from a JSON-serialized `DecisionInput`.
///
/// Returns a JSON-serialized `DecisionOutput` string.
///
/// # Safety
/// `engine_ptr` and `input_json_ptr` must be valid, non-null pointers.
/// Returned string must be freed with `oxide_decision_engine_free_string`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxide_decision_engine_decide_json(
    engine_ptr: *const DecisionEngine,
    input_json_ptr: *const c_char,
) -> *mut c_char {
    if engine_ptr.is_null() || input_json_ptr.is_null() {
        return std::ptr::null_mut();
    }

    let c_str = unsafe { CStr::from_ptr(input_json_ptr) };
    let json_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let input: DecisionInput = match serde_json::from_str(json_str) {
        Ok(inp) => inp,
        Err(_) => return std::ptr::null_mut(),
    };

    let engine = unsafe { &*engine_ptr };
    let output = match engine.decide(input) {
        Ok(out) => out,
        Err(_) => return std::ptr::null_mut(),
    };

    let output_json = match serde_json::to_string(&output) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    match CString::new(output_json) {
        Ok(c_out) => c_out.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Free a C-string allocated by `oxide_decision_engine_decide_json`.
///
/// # Safety
/// `str_ptr` must be a valid pointer allocated by Rust or NULL.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn oxide_decision_engine_free_string(str_ptr: *mut c_char) {
    if !str_ptr.is_null() {
        unsafe {
            drop(CString::from_raw(str_ptr));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mobile_decision_engine_wrapper() {
        let mobile = MobileDecisionEngine::new(8, 5);
        let out = mobile
            .decide(
                "sensor_reading_hot".into(),
                "thermal_state".into(),
                vec!["throttle".into(), "nominal".into()],
            )
            .expect("Decision should succeed");

        assert!(out.selected == "throttle" || out.selected == "nominal");
        assert!(out.confidence > 0.0);
    }

    #[test]
    fn test_c_ffi_json_bridge() {
        unsafe {
            let engine = oxide_decision_engine_create(4, 5);
            assert!(!engine.is_null());

            let input_json = r#"{
                "state": "low_battery_state",
                "criteria": "power_saving",
                "candidates": ["enable_eco", "maintain_performance"]
            }"#;

            let c_input = CString::new(input_json).unwrap();
            let c_output = oxide_decision_engine_decide_json(engine, c_input.as_ptr());
            assert!(!c_output.is_null());

            let out_str = CStr::from_ptr(c_output).to_str().unwrap();
            let output: DecisionOutput = serde_json::from_str(out_str).unwrap();
            assert!(output.selected == "enable_eco" || output.selected == "maintain_performance");

            oxide_decision_engine_free_string(c_output);
            oxide_decision_engine_free(engine);
        }
    }
}
