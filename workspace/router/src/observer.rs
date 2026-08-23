use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Observation report evaluating a single step of an execution loop
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoopObservationReport {
    pub step_index: usize,
    pub latency_ms: u64,
    pub is_redundant: bool,
    pub hallucination_flags: Vec<String>,
    pub step_efficiency_score: f32, // 0.0 to 1.0
    pub recommendation: LoopRecommendation,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum LoopRecommendation {
    Proceed,
    PruneNextStep,
    RollbackAndReflect,
    EscalateToUser,
}

pub struct ObserverAgent {
    known_crates: HashSet<String>,
    history_hashes: HashSet<u64>,
}

impl ObserverAgent {
    pub fn new() -> Self {
        let mut known_crates = HashSet::new();
        known_crates.insert("tokio".to_string());
        known_crates.insert("serde".to_string());
        known_crates.insert("surrealdb".to_string());
        known_crates.insert("actix-web".to_string());
        known_crates.insert("qdrant-client".to_string());
        known_crates.insert("tree-sitter".to_string());
        known_crates.insert("quinn".to_string());
        known_crates.insert("reqwest".to_string());

        Self {
            known_crates,
            history_hashes: HashSet::new(),
        }
    }

    /// Evaluate an agent's proposed action and generated code for hallucinations and circular loops
    pub fn evaluate_step(
        &mut self,
        step_index: usize,
        action_name: &str,
        generated_code: &str,
        latency_ms: u64,
    ) -> LoopObservationReport {
        let mut hallucination_flags = Vec::new();

        // Check for circular / identical action repeat
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        std::hash::Hash::hash(&(action_name, generated_code), &mut hasher);
        let action_hash = std::hash::Hasher::finish(&hasher);
        let is_redundant = !self.history_hashes.insert(action_hash);

        // Scan for hallucinated crate dependencies
        for line in generated_code.lines() {
            let trimmed = line.trim();
            if let Some(rest) = trimmed.strip_prefix("use ") {
                if rest.contains("::") {
                    let root_crate = rest
                        .split("::")
                        .next()
                        .unwrap_or("")
                        .trim();

                    if !root_crate.is_empty()
                        && root_crate != "crate"
                        && root_crate != "super"
                        && root_crate != "self"
                        && root_crate != "std"
                        && root_crate != "core"
                        && root_crate != "alloc"
                        && !self.known_crates.contains(root_crate)
                    {
                        hallucination_flags.push(format!("Unverified external crate referenced: '{}'", root_crate));
                    }
                }
            }
        }

        let mut step_efficiency_score = 1.0;
        if is_redundant {
            step_efficiency_score -= 0.5;
        }
        if !hallucination_flags.is_empty() {
            step_efficiency_score -= 0.3 * hallucination_flags.len() as f32;
        }
        step_efficiency_score = step_efficiency_score.clamp(0.0, 1.0);

        let recommendation = if step_efficiency_score < 0.3 || hallucination_flags.len() >= 3 {
            LoopRecommendation::RollbackAndReflect
        } else if is_redundant {
            LoopRecommendation::PruneNextStep
        } else {
            LoopRecommendation::Proceed
        };

        LoopObservationReport {
            step_index,
            latency_ms,
            is_redundant,
            hallucination_flags,
            step_efficiency_score,
            recommendation,
        }
    }
}

impl Default for ObserverAgent {
    fn default() -> Self {
        Self::new()
    }
}
