//! # Declarative Specifications & Hook Points
//!
//! Frozen types for the training harness.

use surface_api::abi::Dims;

#[derive(Debug, Clone)]
pub struct ModelSpec {
    pub dims: Dims,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct OptimSpec {
    pub lr: f64,
    pub beta1: f64,
    pub beta2: f64,
    pub weight_decay: f64,
}

#[derive(Debug, Clone)]
pub enum HookPoint {
    BeforeStep,
    AfterStep,
    OnNanLoss,
}
