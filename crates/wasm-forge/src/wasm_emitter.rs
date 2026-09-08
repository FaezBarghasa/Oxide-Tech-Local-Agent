use thiserror::Error;
use wasmtime::{Config, Engine};

#[derive(Error, Debug)]
pub enum WasmError {
    #[error("WAT parsing error: {0}")]
    WatParse(String),
    #[error("Wasmtime compilation error: {0}")]
    Compilation(String),
    #[error("Execution error: {0}")]
    Execution(String),
}

pub struct WasmEngine {
    pub engine: Engine,
}

impl Default for WasmEngine {
    fn default() -> Self {
        Self::new().expect("Failed to create default WasmEngine")
    }
}

impl WasmEngine {
    pub fn new() -> Result<Self, WasmError> {
        let mut config = Config::new();
        config.consume_fuel(true);
        let engine = Engine::new(&config).map_err(|e| WasmError::Compilation(e.to_string()))?;
        Ok(Self { engine })
    }

    /// Compile WAT source text to binary WASM bytecode.
    pub fn compile_wat(&self, wat_source: &str) -> Result<Vec<u8>, WasmError> {
        wat::parse_str(wat_source).map_err(|e| WasmError::WatParse(e.to_string()))
    }
}

pub struct WasmModule {
    pub name: String,
    pub bytecode: Vec<u8>,
}

impl WasmModule {
    pub fn from_wat(name: impl Into<String>, wat_source: &str) -> Result<Self, WasmError> {
        let engine = WasmEngine::new()?;
        let bytecode = engine.compile_wat(wat_source)?;
        Ok(Self {
            name: name.into(),
            bytecode,
        })
    }
}
