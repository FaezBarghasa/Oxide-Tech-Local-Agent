use crate::wasm_emitter::WasmEngine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystallizedTool {
    pub name: String,
    pub description: String,
    pub invocation_count: u64,
    pub wasm_bytes: Vec<u8>,
}

pub struct SkillCrystallizer {
    engine: WasmEngine,
    registry: Arc<RwLock<HashMap<String, CrystallizedTool>>>,
}

impl Default for SkillCrystallizer {
    fn default() -> Self {
        Self::new()
    }
}

impl SkillCrystallizer {
    pub fn new() -> Self {
        Self {
            engine: WasmEngine::default(),
            registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Crystallize a verified recurring tool sequence into an immutable WebAssembly module.
    pub async fn crystallize_from_wat(
        &self,
        name: &str,
        description: &str,
        wat_source: &str,
    ) -> Result<CrystallizedTool, String> {
        let bytecode = self
            .engine
            .compile_wat(wat_source)
            .map_err(|e| format!("Compilation failed: {e:?}"))?;

        let tool = CrystallizedTool {
            name: name.to_string(),
            description: description.to_string(),
            invocation_count: 0,
            wasm_bytes: bytecode,
        };

        let mut lock = self.registry.write().await;
        lock.insert(name.to_string(), tool.clone());

        Ok(tool)
    }

    pub async fn get_tool(&self, name: &str) -> Option<CrystallizedTool> {
        let lock = self.registry.read().await;
        lock.get(name).cloned()
    }
}
