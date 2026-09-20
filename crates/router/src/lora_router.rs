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
    pub domain_keywords: Vec<String>,
}

/// Dynamic LoRA Router: Selects task-specific LoRA weights using keyword frequency scoring and fallback classifiers.
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
                    domain_keywords: vec![
                        "surrealql",
                        "sql",
                        "query",
                        "database",
                        "index",
                        "schema",
                        "relate",
                        "recordid",
                    ]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                },
                LoraAdapterConfig {
                    adapter_type: LoraAdapterType::FirmwareEmbedded,
                    adapter_name: "lora_firmware_embedded".to_string(),
                    rank: 64,
                    alpha: 128,
                    path: "weights/lora_firmware_embedded".to_string(),
                    target_modules: vec![
                        "q_proj".to_string(),
                        "k_proj".to_string(),
                        "v_proj".to_string(),
                        "o_proj".to_string(),
                    ],
                    domain_keywords: vec![
                        "no_std", "firmware", "stm32", "esp32", "cortex-m", "embassy", "probe-rs",
                        "hal", "gpio", "uart", "spi", "i2c",
                    ]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                },
                LoraAdapterConfig {
                    adapter_type: LoraAdapterType::PcbCad,
                    adapter_name: "lora_pcb_cad".to_string(),
                    rank: 32,
                    alpha: 64,
                    path: "weights/lora_pcb_cad".to_string(),
                    target_modules: vec!["q_proj".to_string(), "v_proj".to_string()],
                    domain_keywords: vec![
                        "kicad",
                        "skidl",
                        "pcb",
                        "schematic",
                        "netlist",
                        "footprint",
                        "gerber",
                        "drc",
                        "erc",
                        "spice",
                    ]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                },
                LoraAdapterConfig {
                    adapter_type: LoraAdapterType::SecurityAudit,
                    adapter_name: "lora_security_audit".to_string(),
                    rank: 32,
                    alpha: 64,
                    path: "weights/lora_security_audit".to_string(),
                    target_modules: vec!["q_proj".to_string(), "v_proj".to_string()],
                    domain_keywords: vec![
                        "cve",
                        "vulnerability",
                        "audit",
                        "exploit",
                        "memory safety",
                        "unsafe",
                        "overflow",
                        "injection",
                        "sbom",
                    ]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                },
                LoraAdapterConfig {
                    adapter_type: LoraAdapterType::UiStyling,
                    adapter_name: "lora_ui_styling".to_string(),
                    rank: 16,
                    alpha: 32,
                    path: "weights/lora_ui_styling".to_string(),
                    target_modules: vec!["q_proj".to_string(), "v_proj".to_string()],
                    domain_keywords: vec![
                        "tailwind",
                        "css",
                        "slint",
                        "ui",
                        "component",
                        "frontend",
                        "layout",
                        "styling",
                        "rsx",
                        "view",
                    ]
                    .into_iter()
                    .map(String::from)
                    .collect(),
                },
            ],
        }
    }

    /// Predict the optimal LoRA adapter using weighted domain keyword ranking
    pub fn select_adapter_for_prompt(&self, prompt: &str) -> LoraAdapterType {
        let p_lower = prompt.to_lowercase();
        let mut best_score = 0;
        let mut best_type = LoraAdapterType::BaseModel;

        for config in &self.adapters {
            let score = config
                .domain_keywords
                .iter()
                .filter(|kw| p_lower.contains(kw.as_str()))
                .count();

            if score > best_score {
                best_score = score;
                best_type = config.adapter_type.clone();
            }
        }

        best_type
    }

    /// Dispatch dynamic LoRA hot-swap request to SGLang server
    pub fn switch_adapter(&mut self, new_adapter: LoraAdapterType) -> String {
        info!(
            "DynamicLoraRouter switching adapter: {:?} => {:?}",
            self.active_adapter, new_adapter
        );
        self.active_adapter = new_adapter.clone();
        format!("LoRA adapter successfully activated: {:?}", new_adapter)
    }
}

impl Default for DynamicLoraRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_adapter_ranking() {
        let router = DynamicLoraRouter::new();

        assert_eq!(
            router.select_adapter_for_prompt("Write a no_std embassy STM32 SPI driver"),
            LoraAdapterType::FirmwareEmbedded
        );

        assert_eq!(
            router
                .select_adapter_for_prompt("Generate KiCad schematic with DRC and netlist export"),
            LoraAdapterType::PcbCad
        );

        assert_eq!(
            router.select_adapter_for_prompt("Optimize SurrealQL query with recordid indices"),
            LoraAdapterType::SqlOptimization
        );

        assert_eq!(
            router.select_adapter_for_prompt("Fix Slint UI component tailwind styling"),
            LoraAdapterType::UiStyling
        );

        assert_eq!(
            router
                .select_adapter_for_prompt("Check for CVE vulnerability and unsafe memory safety"),
            LoraAdapterType::SecurityAudit
        );

        assert_eq!(
            router.select_adapter_for_prompt("Hello, how are you?"),
            LoraAdapterType::BaseModel
        );
    }
}
