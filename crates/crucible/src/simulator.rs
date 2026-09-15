use crate::mcts::MctsDecisionEngine;
use crate::shadow_state::WorkspaceSnapshot;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimulationEvaluation {
    pub chosen_action: String,
    pub projected_score: f64,
    pub branch_depth: usize,
    pub latency_us: u64,
}

pub trait CausalSimulator: Send + Sync {
    fn simulate_best_action(
        &self,
        snapshot: &WorkspaceSnapshot,
        candidate_actions: &[String],
    ) -> Option<SimulationEvaluation>;
}

#[derive(Default)]
pub struct CrucibleEngine {
    engine: MctsDecisionEngine,
}

impl CrucibleEngine {
    pub fn new(exploration_constant: f64) -> Self {
        Self {
            engine: MctsDecisionEngine::new(exploration_constant),
        }
    }
}

impl CausalSimulator for CrucibleEngine {
    fn simulate_best_action(
        &self,
        snapshot: &WorkspaceSnapshot,
        candidate_actions: &[String],
    ) -> Option<SimulationEvaluation> {
        let start = std::time::Instant::now();

        let result =
            self.engine
                .evaluate_candidates(snapshot, candidate_actions, |_state, action| {
                    score_candidate_action(action)
                });

        result.map(|(action, score)| SimulationEvaluation {
            chosen_action: action,
            projected_score: score,
            branch_depth: 1,
            latency_us: start.elapsed().as_micros() as u64,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionKind {
    Verification,
    Inspection,
    Destructive,
    Modification,
    Neutral,
}

pub fn classify_action(action: &str) -> ActionKind {
    let lower = action.to_lowercase();
    let tokens: Vec<&str> = lower.split_whitespace().collect();

    if tokens
        .iter()
        .any(|&t| t == "cargo" || t == "clippy" || t == "test" || t == "check" || t == "verify")
        || lower.contains("cargo check")
        || lower.contains("cargo test")
        || lower.contains("cargo clippy")
    {
        return ActionKind::Verification;
    }

    if tokens
        .iter()
        .any(|&t| t == "rm" || t == "drop" || t == "delete" || t == "kill" || t == "destroy")
        || lower.contains("rm -rf")
        || lower.contains("--force")
    {
        return ActionKind::Destructive;
    }

    if tokens.iter().any(|&t| {
        t == "read" || t == "cat" || t == "inspect" || t == "view" || t == "list" || t == "ls"
    }) || lower.contains("view_file")
        || lower.contains("list_dir")
    {
        return ActionKind::Inspection;
    }

    if tokens
        .iter()
        .any(|&t| t == "edit" || t == "write" || t == "replace" || t == "patch" || t == "update")
    {
        return ActionKind::Modification;
    }

    ActionKind::Neutral
}

pub fn score_candidate_action(action: &str) -> f64 {
    match classify_action(action) {
        ActionKind::Verification => 3.5,
        ActionKind::Inspection => 1.5,
        ActionKind::Modification => 1.0,
        ActionKind::Neutral => 1.0,
        ActionKind::Destructive => -10.0,
    }
}
