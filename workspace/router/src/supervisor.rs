use agent_journal::{AgentJournal, JournalEvent, TaskResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::oneshot;
use tracing::warn;
use uuid::Uuid;

use crate::agent_modes::{AgentMode, ToolPermissions};

// ── Sub-Agent Roles ───────────────────────────────────────────────────────────

/// Specialized Sub-Agent roles in the multi-agent swarm.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SubAgentRole {
    /// System architecture, interface contracts, specs
    Architect,
    /// Multi-file AST implementation and coding
    Coder,
    /// Compiler error analysis, panic triage, runtime traces
    Debugger,
    /// Containerization, CI/CD pipelines, remote SSH & cloud control
    DevOps,
    /// Security audit, static analysis, SBOM and code review
    Reviewer,
}

impl SubAgentRole {
    pub fn name(&self) -> &'static str {
        match self {
            SubAgentRole::Architect => "ArchitectAgent",
            SubAgentRole::Coder => "CoderAgent",
            SubAgentRole::Debugger => "DebuggerAgent",
            SubAgentRole::DevOps => "DevOpsAgent",
            SubAgentRole::Reviewer => "ReviewerAgent",
        }
    }

    pub fn system_prompt(&self) -> &'static str {
        match self {
            SubAgentRole::Architect => {
                "You are the Lead System Architect Agent.\n\
                 Your job is to produce high-level architecture designs, formal API contracts, data flow diagrams,\n\
                 and task DAGs. Output structured markdown and precise specifications."
            }
            SubAgentRole::Coder => {
                "You are the Execution Coder Agent.\n\
                 Your job is to produce clean, modular, and idiomatic code across multiple files.\n\
                 Adhere strictly to architect specifications. All code must compile cleanly."
            }
            SubAgentRole::Debugger => {
                "You are the Compiler & Runtime Debugger Agent.\n\
                 Your job is to analyze compiler diagnostics (cargo check stderr), runtime panics, and stack traces.\n\
                 Produce targeted root-cause analysis and exact corrective diffs."
            }
            SubAgentRole::DevOps => {
                "You are the DevOps & Infrastructure Agent.\n\
                 Your job is to manage Dockerfiles, CI/CD pipeline workflows, environment configurations, and remote SSH tasks.\n\
                 Ensure reproducible, secure execution environments."
            }
            SubAgentRole::Reviewer => {
                "You are the Security & Quality Reviewer Agent.\n\
                 Your job is to audit diffs for security vulnerabilities, safety hazards, memory leaks, performance bottlenecks,\n\
                 and compliance against project conventions."
            }
        }
    }
}

// ── Task Status ───────────────────────────────────────────────────────────────

/// Status of an individual task node in the execution DAG.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Passed,
    Failed(String),
    Skipped,
    /// Execution is suspended — awaiting human HITL decision.
    AwaitingHitl { inbox_id: Uuid },
}

// ── Task Node ─────────────────────────────────────────────────────────────────

/// A node in the execution DAG representing a delegated sub-agent task.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskNode {
    pub id: String,
    pub title: String,
    pub role: SubAgentRole,
    pub description: String,
    pub dependencies: Vec<String>,
    pub status: TaskStatus,
    /// Typed result — replaces the old bare `Option<String>`.
    pub result: Option<TaskResult>,
    pub retry_count: usize,
}

// ── Task DAG ──────────────────────────────────────────────────────────────────

/// Directed Acyclic Graph (DAG) for multi-agent coordination.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskDag {
    pub dag_id: Uuid,
    pub nodes: HashMap<String, TaskNode>,
    pub execution_order: Vec<String>,
}

impl TaskDag {
    pub fn new() -> Self {
        Self {
            dag_id: Uuid::new_v4(),
            nodes: HashMap::new(),
            execution_order: Vec::new(),
        }
    }

    pub fn add_task(
        &mut self,
        id: &str,
        title: &str,
        role: SubAgentRole,
        description: &str,
        dependencies: Vec<&str>,
    ) {
        let node = TaskNode {
            id: id.to_string(),
            title: title.to_string(),
            role,
            description: description.to_string(),
            dependencies: dependencies.iter().map(|s| s.to_string()).collect(),
            status: TaskStatus::Pending,
            result: None,
            retry_count: 0,
        };
        self.nodes.insert(id.to_string(), node);
        if !self.execution_order.contains(&id.to_string()) {
            self.execution_order.push(id.to_string());
        }
    }

    /// Check if all dependencies for a node have passed.
    pub fn is_ready(&self, task_id: &str) -> bool {
        if let Some(node) = self.nodes.get(task_id) {
            if node.status != TaskStatus::Pending {
                return false;
            }
            for dep_id in &node.dependencies {
                match self.nodes.get(dep_id) {
                    Some(dep_node) if dep_node.status == TaskStatus::Passed => continue,
                    _ => return false,
                }
            }
            true
        } else {
            false
        }
    }

    /// Group tasks that have no mutual dependencies — these can run in parallel.
    ///
    /// Returns a `Vec<Vec<String>>` where each inner vec is a parallel execution tier.
    /// Tiers are ordered so that a later tier only starts after all earlier tiers complete.
    pub fn parallel_execution_tiers(&self) -> Vec<Vec<String>> {
        let mut tiers: Vec<Vec<String>> = Vec::new();
        let mut assigned: HashMap<String, usize> = HashMap::new();

        // Topological sort — assign each task to the earliest tier it can occupy.
        for task_id in &self.execution_order {
            let node = match self.nodes.get(task_id) {
                Some(n) => n,
                None => continue,
            };

            let tier = node
                .dependencies
                .iter()
                .filter_map(|dep| assigned.get(dep))
                .max()
                .copied()
                .map(|t| t + 1)
                .unwrap_or(0);

            assigned.insert(task_id.clone(), tier);

            if tiers.len() <= tier {
                tiers.resize(tier + 1, Vec::new());
            }
            tiers[tier].push(task_id.clone());
        }

        tiers
    }
}

// ── HITL Inbox Handle ─────────────────────────────────────────────────────────

/// A pending HITL approval that can suspend and resume a task.
pub struct HitlHandle {
    pub inbox_id: Uuid,
    pub task_id: String,
    pub reason: String,
    /// Send `true` (approved) or `false` (denied) to resume the parked task.
    pub resume_tx: oneshot::Sender<bool>,
}

// ── Multi-Agent Supervisor ────────────────────────────────────────────────────

/// Multi-Agent Supervisor: plans, delegates, monitors, and guards against oscillation.
pub struct SupervisorAgent {
    pub mode: AgentMode,
    pub permissions: ToolPermissions,
    pub max_retries: usize,
    pub error_history: Vec<String>,
    /// Optional durable journal — if `None`, journaling is disabled (e.g. in tests).
    pub journal: Option<AgentJournal>,
}

impl SupervisorAgent {
    pub fn new(mode: AgentMode) -> Self {
        let permissions = ToolPermissions::for_mode(mode);
        Self {
            mode,
            permissions,
            max_retries: 3,
            error_history: Vec::new(),
            journal: None,
        }
    }

    /// Attach a durable journal to this supervisor for crash-safe execution.
    pub fn with_journal(mut self, journal: AgentJournal) -> Self {
        self.journal = Some(journal);
        self
    }

    /// Convenience: append a journal event if a journal is attached.
    pub async fn journal_append(&self, event: JournalEvent) {
        if let Some(j) = &self.journal {
            if let Err(e) = j.append(event).await {
                warn!("journal_append failed (non-fatal): {e}");
            }
        }
    }

    /// Decomposes a user goal into a verified multi-agent task DAG and journals it.
    pub async fn plan_goal(&self, goal: &str) -> TaskDag {
        let mut dag = TaskDag::new();

        // Journal the DAG creation event
        self.journal_append(JournalEvent::DagCreated {
            dag_id: dag.dag_id,
            goal: goal.to_string(),
            mode: self.mode.as_str().to_string(),
        })
        .await;

        match self.mode {
            AgentMode::Architect => {
                dag.add_task(
                    "arch_spec",
                    "Design Architecture Specification",
                    SubAgentRole::Architect,
                    &format!("Analyze goal and create detailed architectural specification for: {goal}"),
                    vec![],
                );
            }
            AgentMode::Ask => {
                dag.add_task(
                    "research_answer",
                    "Research Knowledge & Formulate Answer",
                    SubAgentRole::Architect,
                    &format!("Query documentation and explain: {goal}"),
                    vec![],
                );
            }
            AgentMode::Review => {
                dag.add_task(
                    "security_audit",
                    "Security & Code Quality Audit",
                    SubAgentRole::Reviewer,
                    &format!("Audit AST, dependencies, and memory safety for: {goal}"),
                    vec![],
                );
            }
            AgentMode::Debug => {
                dag.add_task(
                    "diagnose",
                    "Compiler & Diagnostic Analysis",
                    SubAgentRole::Debugger,
                    &format!("Diagnose error and synthesize minimal repair for: {goal}"),
                    vec![],
                );
            }
            AgentMode::Research => {
                dag.add_task(
                    "deep_research",
                    "Multi-Source Documentation Synthesis",
                    SubAgentRole::Architect,
                    &format!("Perform deep RAG and web research for: {goal}"),
                    vec![],
                );
            }
            AgentMode::Code | AgentMode::Autonomous => {
                // Tier 0: Architect (no deps)
                dag.add_task(
                    "plan",
                    "Architectural Task Decomposition",
                    SubAgentRole::Architect,
                    &format!("Create interface specification and edit plan for: {goal}"),
                    vec![],
                );
                // Tier 1: Coder (depends on plan)
                dag.add_task(
                    "implement",
                    "Multi-File AST Code Synthesis",
                    SubAgentRole::Coder,
                    &format!("Implement code modifications for: {goal}"),
                    vec!["plan"],
                );
                // Tier 2a: Debugger & Reviewer can run in parallel (both depend on implement only)
                dag.add_task(
                    "verify",
                    "Compiler & Test Suite Verification",
                    SubAgentRole::Debugger,
                    "Run cargo check, tests, and analyze any diagnostics",
                    vec!["implement"],
                );
                dag.add_task(
                    "review",
                    "Security & Code Quality Audit",
                    SubAgentRole::Reviewer,
                    "Verify security posture, dependencies, and code conventions",
                    vec!["implement"],
                );
            }
            AgentMode::Plan => {
                dag.add_task(
                    "draft_plan",
                    "Read-Only Planning Pass",
                    SubAgentRole::Architect,
                    &format!("Draft a read-only execution plan for: {goal}"),
                    vec![],
                );
            }
        }

        dag
    }

    /// Detect oscillation if the same error is seen N >= 3 times in a row.
    pub async fn detect_oscillation(&mut self, dag_id: Uuid, task_id: &str, error_signature: &str) -> bool {
        self.error_history.push(error_signature.to_string());
        if self.error_history.len() >= 3 {
            let last_three = &self.error_history[self.error_history.len() - 3..];
            if last_three[0] == last_three[1] && last_three[1] == last_three[2] {
                warn!("Oscillation detected! Same failure repeated 3 times: {}", error_signature);
                self.journal_append(JournalEvent::OscillationDetected {
                    dag_id,
                    task_id: task_id.to_string(),
                    error_signature: error_signature.to_string(),
                    count: 3,
                })
                .await;
                return true;
            }
        }
        false
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn plan_goal_code_mode_produces_parallel_tiers() {
        let supervisor = SupervisorAgent::new(AgentMode::Code);
        let dag = supervisor.plan_goal("Implement UART driver").await;

        let tiers = dag.parallel_execution_tiers();

        // Tier 0: plan (no deps)
        assert_eq!(tiers[0], vec!["plan"]);
        // Tier 1: implement (depends on plan)
        assert_eq!(tiers[1], vec!["implement"]);
        // Tier 2: verify AND review are parallel (both depend on implement only)
        assert_eq!(tiers.len(), 3);
        let tier2: std::collections::HashSet<&String> = tiers[2].iter().collect();
        assert!(tier2.contains(&"verify".to_string()));
        assert!(tier2.contains(&"review".to_string()));
    }

    #[tokio::test]
    async fn plan_goal_architect_mode_produces_one_task() {
        let supervisor = SupervisorAgent::new(AgentMode::Architect);
        let dag = supervisor.plan_goal("Design embedded HAL").await;
        assert_eq!(dag.nodes.len(), 1);
        assert!(dag.nodes.contains_key("arch_spec"));
    }

    #[tokio::test]
    async fn plan_goal_debug_mode_produces_diagnose_task() {
        let supervisor = SupervisorAgent::new(AgentMode::Debug);
        let dag = supervisor.plan_goal("Fix borrow checker error").await;
        assert_eq!(dag.nodes.len(), 1);
        assert!(dag.nodes.contains_key("diagnose"));
    }

    #[test]
    fn dag_is_ready_respects_dependencies() {
        let mut dag = TaskDag::new();
        dag.add_task("a", "A", SubAgentRole::Architect, "desc", vec![]);
        dag.add_task("b", "B", SubAgentRole::Coder, "desc", vec!["a"]);

        // "b" is NOT ready until "a" passes
        assert!(!dag.is_ready("b"));

        dag.nodes.get_mut("a").unwrap().status = TaskStatus::Passed;
        assert!(dag.is_ready("b"));
    }

    #[test]
    fn parallel_tiers_single_chain() {
        let mut dag = TaskDag::new();
        dag.add_task("x", "X", SubAgentRole::Architect, "", vec![]);
        dag.add_task("y", "Y", SubAgentRole::Coder, "", vec!["x"]);
        dag.add_task("z", "Z", SubAgentRole::Debugger, "", vec!["y"]);

        let tiers = dag.parallel_execution_tiers();
        assert_eq!(tiers.len(), 3);
        assert_eq!(tiers[0], vec!["x"]);
        assert_eq!(tiers[1], vec!["y"]);
        assert_eq!(tiers[2], vec!["z"]);
    }
}
