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

pub struct CrucibleEngine {
    engine: MctsDecisionEngine,
}

impl Default for CrucibleEngine {
    fn default() -> Self {
        Self {
            engine: MctsDecisionEngine::default(),
        }
    }
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
                    // Heuristic scoring: prefer compile checks, verify actions, and low complexity operations
                    let mut score = 1.0;
                    if action.contains("cargo check") || action.contains("verify") {
                        score += 2.5;
                    }
                    if action.contains("rm -rf") || action.contains("force") {
                        score -= 10.0;
                    }
                    if action.contains("read") || action.contains("inspect") {
                        score += 0.5;
                    }
                    score
                });

        result.map(|(action, score)| SimulationEvaluation {
            chosen_action: action,
            projected_score: score,
            branch_depth: 1,
            latency_us: start.elapsed().as_micros() as u64,
        })
    }
}
