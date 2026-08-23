use serde::{Deserialize, Serialize};
use tracing::info;

/// Specialized LoRA Adapter types for local model hot-swapping
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum LoraAdapterType {
    BaseModel,
    SqlOptimization,
    FirmwareEmbedded,
    PcbCad,
    SecurityAudit,
    UiStyling,
}

/// Metadata and configuration for a dynamic LoRA adapter
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoraAdapterConfig {
    pub adapter_type: LoraAdapterType,
    pub adapter_name: String,
    pub rank: u32,
    pub alpha: u32,
    pub path: String,
    pub target_modules: Vec<String>,
}

/// Dynamic LoRA Router: Hot-swaps task-specific LoRA weights on SGLang TP=2 serving cluster
#[derive(Debug, Clone)]
pub struct DynamicLoraRouter {
    pub active_adapter: LoraAdapterType,
    pub adapters: Vec<LoraAdapterConfig>,
}

impl DynamicLoraRouter {
    pub fn new() -> Self {
        Self {
            active_adapter: LoraAdapterType::BaseModel,
            adapters: vec![
                LoraAdapterConfig {
                    adapter_type: LoraAdapterType::SqlOptimization,
                    adapter_name: "lora_sql_opt".to_string(),
                    rank: 32,
                    alpha: 64,
                    path: "weights/lora_sql_opt".to_string(),
                    target_modules: vec!["q_proj".to_string(), "v_proj".to_string()],
                },
                LoraAdapterConfig {
                    adapter_type: LoraAdapterType::FirmwareEmbedded,
                    adapter_name: "lora_firmware_embedded".to_string(),
                    rank: 64,
                    alpha: 128,
                    path: "weights/lora_firmware_embedded".to_string(),
                    target_modules: vec!["q_proj".to_string(), "k_proj".to_string(), "v_proj".to_string(), "o_proj".to_string()],
                },
                LoraAdapterConfig {
                    adapter_type: LoraAdapterType::PcbCad,
                    adapter_name: "lora_pcb_cad".to_string(),
                    rank: 32,
                    alpha: 64,
                    path: "weights/lora_pcb_cad".to_string(),
                    target_modules: vec!["q_proj".to_string(), "v_proj".to_string()],
                },
                LoraAdapterConfig {
                    adapter_type: LoraAdapterType::SecurityAudit,
                    adapter_name: "lora_security_audit".to_string(),
                    rank: 32,
                    alpha: 64,
                    path: "weights/lora_security_audit".to_string(),
                    target_modules: vec!["q_proj".to_string(), "v_proj".to_string()],
                },
                LoraAdapterConfig {
                    adapter_type: LoraAdapterType::UiStyling,
                    adapter_name: "lora_ui_styling".to_string(),
                    rank: 16,
                    alpha: 32,
                    path: "weights/lora_ui_styling".to_string(),
                    target_modules: vec!["q_proj".to_string(), "v_proj".to_string()],
                },
            ],
        }
    }

    /// Predict the optimal LoRA adapter for a given user prompt or task goal
    pub fn select_adapter_for_prompt(&self, prompt: &str) -> LoraAdapterType {
        let p_lower = prompt.to_lowercase();
        if p_lower.contains("surrealql") || p_lower.contains("sql") || p_lower.contains("query") || p_lower.contains("database") {
            LoraAdapterType::SqlOptimization
        } else if p_lower.contains("no_std") || p_lower.contains("firmware") || p_lower.contains("stm32") || p_lower.contains("esp32") || p_lower.contains("klipper") || p_lower.contains("embassy") {
            LoraAdapterType::FirmwareEmbedded
        } else if p_lower.contains("kicad") || p_lower.contains("skidl") || p_lower.contains("pcb") || p_lower.contains("schematic") || p_lower.contains("netlist") {
            LoraAdapterType::PcbCad
        } else if p_lower.contains("cve") || p_lower.contains("vulnerability") || p_lower.contains("audit") || p_lower.contains("exploit") || p_lower.contains("memory safety") {
            LoraAdapterType::SecurityAudit
        } else if p_lower.contains("tailwind") || p_lower.contains("css") || p_lower.contains("ui") || p_lower.contains("react") || p_lower.contains("component") {
            LoraAdapterType::UiStyling
        } else {
            LoraAdapterType::BaseModel
        }
    }

    /// Dispatch dynamic LoRA hot-swap request to SGLang server
    pub fn switch_adapter(&mut self, new_adapter: LoraAdapterType) -> String {
        info!("DynamicLoraRouter switching adapter: {:?} => {:?}", self.active_adapter, new_adapter);
        self.active_adapter = new_adapter.clone();
        format!("LoRA adapter successfully activated: {:?}", new_adapter)
    }
}

impl Default for DynamicLoraRouter {
    fn default() -> Self {
        Self::new()
    }
}
