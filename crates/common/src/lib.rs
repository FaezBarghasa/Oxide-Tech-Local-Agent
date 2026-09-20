pub mod config;
pub mod contracts;
pub mod error;

pub use config::AppConfig;
pub use contracts::*;
pub use error::{EiosError, Result};

uniffi::setup_scaffolding!();
