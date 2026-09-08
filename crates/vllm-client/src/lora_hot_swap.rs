use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::info;

/// Manages dynamic discovery and hot-swapping of LoRA adapters at runtime.
pub struct LoraHotSwapManager {
    adapters: HashMap<String, PathBuf>,
    active_adapter: Option<String>,
}

impl Default for LoraHotSwapManager {
    fn default() -> Self {
        Self::new()
    }
}

impl LoraHotSwapManager {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
            active_adapter: None,
        }
    }

    pub fn register_adapter(&mut self, name: &str, path: PathBuf) {
        info!("Registering LoRA adapter '{}' at {:?}", name, path);
        self.adapters.insert(name.to_string(), path);
    }

    pub fn switch_adapter(&mut self, name: &str) -> Result<PathBuf> {
        let path = self
            .adapters
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Adapter '{}' not registered", name))?;
        info!("Swapped active adapter to '{}'", name);
        self.active_adapter = Some(name.to_string());
        Ok(path)
    }

    pub fn active_adapter(&self) -> Option<&str> {
        self.active_adapter.as_deref()
    }
}
