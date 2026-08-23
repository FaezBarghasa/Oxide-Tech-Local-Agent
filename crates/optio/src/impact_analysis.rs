use std::collections::{HashSet, VecDeque};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactSurface {
    pub modified_node: String,
    pub affected_callers: HashSet<String>,
    pub affected_files: HashSet<String>,
    pub recommended_test_targets: Vec<String>,
}

pub struct ImpactAnalyzer;

impl ImpactAnalyzer {
    pub fn calculate_impact_from_graph(
        modified_fn_id: &str,
        caller_map: &std::collections::HashMap<String, Vec<(String, String)>>, // node_id -> Vec<(caller_id, file_path)>
    ) -> ImpactSurface {
        let mut affected_callers = HashSet::new();
        let mut affected_files = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(modified_fn_id.to_string());

        while let Some(current) = queue.pop_front() {
            if let Some(callers) = caller_map.get(&current) {
                for (caller_id, file_path) in callers {
                    affected_files.insert(file_path.clone());
                    if affected_callers.insert(caller_id.clone()) {
                        queue.push_back(caller_id.clone());
                    }
                }
            }
        }

        let recommended_test_targets = affected_files
            .iter()
            .map(|f| format!("cargo test --test {}", f.replace(".rs", "").replace("src/", "")))
            .collect();

        ImpactSurface {
            modified_node: modified_fn_id.to_string(),
            affected_callers,
            affected_files,
            recommended_test_targets,
        }
    }
}
