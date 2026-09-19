use thiserror::Error;

#[derive(Debug, Error)]
pub enum VerificationError {
    #[error("Rust syntax error: {message}\nEmitted code:\n{code}")]
    SyntaxError {
        message: String,
        code: String,
    },
}

/// Verifier that validates emitted Rust code against `syn` parser.
pub struct RustVerifier;

impl RustVerifier {
    pub fn verify(code: &str) -> Result<(), VerificationError> {
        syn::parse_file(code).map_err(|e| VerificationError::SyntaxError {
            message: e.to_string(),
            code: code.to_string(),
        })?;
        Ok(())
    }
}
