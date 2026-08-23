use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::warn;
use crate::agent_modes::{AgentMode, ToolPermissions};

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

/// Status of an individual task node in the execution DAG
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TaskStatus {
    Pending,
    Running,
    Passed,
    Failed(String),
    Skipped,
}

/// A node in the execution DAG representing a delegated sub-agent task
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskNode {
    pub id: String,
    pub title: String,
    pub role: SubAgentRole,
    pub description: String,
    pub dependencies: Vec<String>,
    pub status: TaskStatus,
    pub result: Option<String>,
    pub retry_count: usize,
}

/// Directed Acyclic Graph (DAG) for multi-agent coordination
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TaskDag {
    pub nodes: HashMap<String, TaskNode>,
    pub execution_order: Vec<String>,
}

impl TaskDag {
    pub fn new() -> Self {
        Self {
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

    /// Check if all dependencies for a node have passed
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
}

/// Multi-Agent Supervisor: plans, delegates, monitors, and guards against oscillation.
pub struct SupervisorAgent {
    pub mode: AgentMode,
    pub permissions: ToolPermissions,
    pub max_retries: usize,
    pub error_history: Vec<String>,
}

impl SupervisorAgent {
    pub fn new(mode: AgentMode) -> Self {
        let permissions = ToolPermissions::for_mode(mode);
        Self {
            mode,
            permissions,
            max_retries: 3,
            error_history: Vec::new(),
        }
    }

    /// Decomposes a user goal into a standard verified multi-agent DAG
    pub fn plan_goal(&self, goal: &str) -> TaskDag {
        let mut dag = TaskDag::new();

        match self.mode {
            AgentMode::Architect => {
                dag.add_task(
                    "arch_spec",
                    "Design Architecture Specification",
                    SubAgentRole::Architect,
                    &format!("Analyze goal and create detailed architectural specification for: {}", goal),
                    vec![],
                );
            }
            AgentMode::Ask => {
                dag.add_task(
                    "research_answer",
                    "Research Knowledge & Formulate Answer",
                    SubAgentRole::Architect,
                    &format!("Query documentation and explain: {}", goal),
                    vec![],
                );
            }
            AgentMode::Code | AgentMode::Autonomous => {
                dag.add_task(
                    "plan",
                    "Architectural Task Decomposition",
                    SubAgentRole::Architect,
                    &format!("Create interface specification and edit plan for: {}", goal),
                    vec![],
                );
                dag.add_task(
                    "implement",
                    "Multi-File AST Code Synthesis",
                    SubAgentRole::Coder,
                    &format!("Implement code modifications for: {}", goal),
                    vec!["plan"],
                );
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
                    vec!["verify"],
                );
            }
        }

        dag
    }

    /// Detect oscillation if the same error is seen N >= 3 times in a row
    pub fn detect_oscillation(&mut self, error_signature: &str) -> bool {
        self.error_history.push(error_signature.to_string());
        if self.error_history.len() >= 3 {
            let last_three = &self.error_history[self.error_history.len() - 3..];
            if last_three[0] == last_three[1] && last_three[1] == last_three[2] {
                warn!("Oscillation detected! Same failure repeated 3 times: {}", error_signature);
                return true;
            }
        }
        false
    }
}
