use crate::ir::UirModule;

pub mod composition;
pub mod concurrency;
pub mod error_handling;
pub mod naming;
pub mod ownership;

pub trait RefactorPass {
    fn run(&self, module: &mut UirModule);
}

/// Pipeline of refactoring passes converting polyglot UIR into idiomatic Rust.
pub struct RefactorPipeline {
    passes: Vec<Box<dyn RefactorPass>>,
}

impl Default for RefactorPipeline {
    fn default() -> Self {
        Self {
            passes: vec![
                Box::new(naming::NamingPass),
                Box::new(ownership::OwnershipPass),
                Box::new(error_handling::ErrorHandlingPass),
                Box::new(composition::CompositionPass),
                Box::new(concurrency::ConcurrencyPass),
            ],
        }
    }
}

impl RefactorPipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute(&self, module: &mut UirModule) {
        for pass in &self.passes {
            pass.run(module);
        }
    }
}
