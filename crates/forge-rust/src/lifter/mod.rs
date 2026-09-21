use crate::detector::SourceLanguage;
use crate::ir::UirModule;
use thiserror::Error;

pub mod c_lifter;
pub mod generic_lifter;
pub mod go_lifter;
pub mod python_lifter;
pub mod typescript_lifter;

#[derive(Debug, Error)]
pub enum LifterError {
    #[error("Failed to parse {language:?}: {details}")]
    ParseError {
        language: SourceLanguage,
        details: String,
    },

    #[error("Unsupported language idiom in {language:?}: {details}")]
    UnsupportedIdiom {
        language: SourceLanguage,
        details: String,
    },
}

/// Trait for lifting foreign source text into a Universal Intermediate Representation (`UirModule`).
pub trait LanguageLifter: Send + Sync {
    fn language(&self) -> SourceLanguage;
    fn lift(&self, source: &str, module_name: &str) -> Result<UirModule, LifterError>;
}

/// Dispatches lifting based on detected source language.
pub struct LifterDispatcher;

impl LifterDispatcher {
    pub fn lift(
        source: &str,
        module_name: &str,
        lang: SourceLanguage,
    ) -> Result<UirModule, LifterError> {
        match lang {
            SourceLanguage::C | SourceLanguage::Cpp => c_lifter::CLifter.lift(source, module_name),
            SourceLanguage::Python => python_lifter::PythonLifter.lift(source, module_name),
            SourceLanguage::TypeScript | SourceLanguage::JavaScript => {
                typescript_lifter::TypeScriptLifter.lift(source, module_name)
            }
            SourceLanguage::Go => go_lifter::GoLifter.lift(source, module_name),
            _ => generic_lifter::GenericLifter.lift(source, module_name),
        }
    }
}
