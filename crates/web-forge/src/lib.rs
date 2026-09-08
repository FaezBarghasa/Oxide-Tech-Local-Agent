use std::time::Duration;
use thiserror::Error;

pub mod actionability;
pub mod codegen;
pub mod distiller;
pub mod network_interceptor;

pub use actionability::{
    ActionabilityConfig, ActionabilityEvaluator, BoundingBox, ElementLayoutState, HitTestResult,
    LayoutInspector,
};
pub use codegen::{MacroCodegen, WebAction, WebScript};
pub use distiller::{AXNode, AXTree, DomDistiller};
pub use network_interceptor::{
    InterceptDecision, InterceptedRequest, MockResponse, MockRule, NetworkInterceptor,
};

/// Errors encountered in the web automation, distillation, and interception pipeline.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum WebForgeError {
    #[error("Actionability check timed out: {0}")]
    ActionabilityTimeout(String),

    #[error("Element not found in DOM: {0}")]
    ElementNotFound(String),

    #[error("Element '{selector}' is occluded by '{hit_element}'")]
    ElementOccluded {
        selector: String,
        hit_element: String,
    },

    #[error("Element '{0}' is disabled")]
    ElementDisabled(String),

    #[error("Causal network interception error: {0}")]
    NetworkInterceptionError(String),

    #[error("Assertion failed: {0}")]
    AssertionFailed(String),

    #[error("Serialization / parsing error: {0}")]
    SerializationError(String),
}

/// Abstract harness trait for driving web interactions asynchronously.
#[allow(async_fn_in_trait)]
pub trait WebHarness {
    async fn navigate(&mut self, url: &str) -> Result<(), WebForgeError>;
    async fn click(&mut self, selector: &str) -> Result<(), WebForgeError>;
    async fn type_text(&mut self, selector: &str, text: &str) -> Result<(), WebForgeError>;
    async fn hover(&mut self, selector: &str) -> Result<(), WebForgeError>;
    async fn wait_for_selector(
        &mut self,
        selector: &str,
        timeout: Duration,
    ) -> Result<(), WebForgeError>;
    async fn press_key(&mut self, key: &str) -> Result<(), WebForgeError>;
    async fn assert_text(&mut self, selector: &str, expected: &str) -> Result<(), WebForgeError>;
}
