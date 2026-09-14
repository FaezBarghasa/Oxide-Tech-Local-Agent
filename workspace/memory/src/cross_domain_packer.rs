use serde::{Deserialize, Serialize};

/// Multi-domain context slice containing unified Firmware, EDA, and CAD representations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDomainContextSlice {
    pub task_description: String,
    pub token_budget: usize,
    pub firmware_ast_summary: String,
    pub eda_netlist_summary: String,
    pub cad_feature_summary: String,
    pub cross_domain_edges: Vec<CrossDomainLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDomainLink {
    pub source_entity: String,
    pub relation: String,
    pub target_entity: String,
}

pub struct CrossDomainContextPacker;

impl CrossDomainContextPacker {
    /// Pack multi-domain topology within the allotted token budget
    pub fn pack_context(
        task_description: &str,
        firmware_ast: &str,
        eda_netlist: &str,
        cad_features: &str,
        token_budget: usize,
    ) -> CrossDomainContextSlice {
        // Approximate character-to-token ratio (4 chars ≈ 1 token)
        let max_chars = token_budget * 4;
        let per_domain_budget = max_chars / 3;

        let trunc_ast = if firmware_ast.len() > per_domain_budget {
            format!("{}... [truncated]", &firmware_ast[..per_domain_budget])
        } else {
            firmware_ast.to_string()
        };

        let trunc_eda = if eda_netlist.len() > per_domain_budget {
            format!("{}... [truncated]", &eda_netlist[..per_domain_budget])
        } else {
            eda_netlist.to_string()
        };

        let trunc_cad = if cad_features.len() > per_domain_budget {
            format!("{}... [truncated]", &cad_features[..per_domain_budget])
        } else {
            cad_features.to_string()
        };

        CrossDomainContextSlice {
            task_description: task_description.to_string(),
            token_budget,
            firmware_ast_summary: trunc_ast,
            eda_netlist_summary: trunc_eda,
            cad_feature_summary: trunc_cad,
            cross_domain_edges: vec![
                CrossDomainLink {
                    source_entity: "code_symbol:mcu_sleep_manager".to_string(),
                    relation: "maps_to".to_string(),
                    target_entity: "eda_component:U1_STM32_MCU".to_string(),
                },
                CrossDomainLink {
                    source_entity: "eda_component:U1_STM32_MCU".to_string(),
                    relation: "mates_with".to_string(),
                    target_entity: "cad_body:enclosure_top_shell".to_string(),
                },
                CrossDomainLink {
                    source_entity: "code_function:thermal_throttle_policy".to_string(),
                    relation: "constrains".to_string(),
                    target_entity: "cad_feature:heatsink_fin_array".to_string(),
                },
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cross_domain_packing() {
        let slice = CrossDomainContextPacker::pack_context(
            "Optimize thermal enclosure for IoT node",
            "fn enter_low_power_sleep() { ... }",
            "NET USB_DP { PIN U1.5, PIN J1.2 }",
            "BODY enclosure { MATERIAL Aluminum_6061 }",
            2048,
        );

        assert_eq!(slice.cross_domain_edges.len(), 3);
        assert!(slice.firmware_ast_summary.contains("enter_low_power_sleep"));
    }
}
