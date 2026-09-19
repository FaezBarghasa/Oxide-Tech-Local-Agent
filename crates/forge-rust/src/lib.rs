pub mod detector;
pub mod emitter;
pub mod ir;
pub mod lifter;
pub mod refactor;
pub mod scaffold;
pub mod verifier;

use std::collections::HashMap;
use thiserror::Error;

pub use detector::{LanguageDetector, SourceLanguage};
pub use emitter::RustEmitter;
pub use ir::{UirField, UirFunction, UirItem, UirModule, UirParam, UirSelfKind, UirStmt, UirStruct, UirTrait, UirType};
pub use lifter::{LanguageLifter, LifterDispatcher, LifterError};
pub use refactor::{RefactorPass, RefactorPipeline};
pub use scaffold::ProjectScaffolder;
pub use verifier::{RustVerifier, VerificationError};

#[derive(Debug, Error)]
pub enum ForgeRustError {
    #[error("Lifter failure: {0}")]
    Lifter(#[from] LifterError),

    #[error("Verification failure: {0}")]
    Verification(#[from] VerificationError),
}

/// Configuration options for the polyglot-to-Rust refactoring process.
#[derive(Debug, Clone)]
pub struct RefactorConfig {
    pub module_name: String,
    pub language_hint: Option<SourceLanguage>,
    pub verify_syntax: bool,
    pub is_binary: bool,
}

impl Default for RefactorConfig {
    fn default() -> Self {
        Self {
            module_name: "refactored_module".to_string(),
            language_hint: None,
            verify_syntax: true,
            is_binary: false,
        }
    }
}

/// Result of refactoring polyglot source code to Rust.
#[derive(Debug, Clone)]
pub struct RefactorResult {
    pub source_language: SourceLanguage,
    pub uir: UirModule,
    pub rust_code: String,
    pub files: HashMap<String, String>,
}

/// Main entry point for `forge-rust` refactoring engine.
pub struct ForgeRust;

impl ForgeRust {
    /// Refactors foreign source code into idiomatic Rust.
    pub fn refactor(source: &str, config: RefactorConfig) -> Result<RefactorResult, ForgeRustError> {
        // 1. Detect language
        let lang = config.language_hint.unwrap_or_else(|| {
            LanguageDetector::detect(source, None)
        });

        // 2. Lift to UIR
        let mut uir = LifterDispatcher::lift(source, &config.module_name, lang)?;

        // 3. Execute Refactoring Pipeline
        let pipeline = RefactorPipeline::default();
        pipeline.execute(&mut uir);

        // 4. Emit Idiomatic Rust Source Code
        let rust_code = RustEmitter::emit(&uir);

        // 5. Syntax Verification (via syn)
        if config.verify_syntax {
            RustVerifier::verify(&rust_code)?;
        }

        // 6. Scaffold Cargo project files
        let files = ProjectScaffolder::scaffold(
            &config.module_name,
            &rust_code,
            &uir.required_dependencies,
            config.is_binary,
        );

        Ok(RefactorResult {
            source_language: lang,
            uir,
            rust_code,
            files,
        })
    }
}
