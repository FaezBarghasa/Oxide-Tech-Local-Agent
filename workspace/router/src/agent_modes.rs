use serde::{Deserialize, Serialize};

/// Operational modes that alter agent behavior, depth of reasoning, and tool permissions.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default)]
pub enum AgentMode {
    /// High-velocity code editing with compiler auto-healing
    #[default]
    Code,
    /// High-level system design, schema specification, read-only
    Architect,
    /// Conversational inquiry, code explanation, search without mutations
    Ask,
    /// Full-stack autonomous plan-to-deploy execution with automated verification
    Autonomous,
}

impl AgentMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "architect" => AgentMode::Architect,
            "ask" => AgentMode::Ask,
            "autonomous" | "auto" => AgentMode::Autonomous,
            _ => AgentMode::Code,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            AgentMode::Code => "code",
            AgentMode::Architect => "architect",
            AgentMode::Ask => "ask",
            AgentMode::Autonomous => "autonomous",
        }
    }

    /// Returns the system prompt modifier customized for the operational mode.
    pub fn system_prompt_directive(&self) -> &'static str {
        match self {
            AgentMode::Code => {
                "OPERATIONAL MODE: CODE.\n\
                 Focus on concise, idiomatic, verified code edits. All code must pass compiler checks.\n\
                 Minimize conversational fluff. Output precise tool calls."
            }
            AgentMode::Architect => {
                "OPERATIONAL MODE: ARCHITECT.\n\
                 Focus on comprehensive high-level system design, module boundaries, database schemas,\n\
                 and contract specifications. Do NOT modify source files directly. Provide architectural blueprints."
            }
            AgentMode::Ask => {
                "OPERATIONAL MODE: ASK.\n\
                 Focus on insightful explanations, documentation lookups, and clear conceptual answers.\n\
                 Do NOT execute modifying tools or file changes."
            }
            AgentMode::Autonomous => {
                "OPERATIONAL MODE: AUTONOMOUS.\n\
                 You have full agency to plan, research, code across multiple files, run tests, fix errors,\n\
                 and deploy. Follow through until the goal is completely achieved and verified."
            }
        }
    }
}

/// Granular tool use permissions
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ToolPermissions {
    pub allow_read_files: bool,
    pub allow_write_files: bool,
    pub allow_terminal_exec: bool,
    pub allow_network_outbound: bool,
    pub allow_hardware_flash: bool,
    pub allow_git_commit: bool,
}

impl ToolPermissions {
    pub fn for_mode(mode: AgentMode) -> Self {
        match mode {
            AgentMode::Code => Self {
                allow_read_files: true,
                allow_write_files: true,
                allow_terminal_exec: true,
                allow_network_outbound: true,
                allow_hardware_flash: false,
                allow_git_commit: true,
            },
            AgentMode::Architect => Self {
                allow_read_files: true,
                allow_write_files: false,
                allow_terminal_exec: false,
                allow_network_outbound: true,
                allow_hardware_flash: false,
                allow_git_commit: false,
            },
            AgentMode::Ask => Self {
                allow_read_files: true,
                allow_write_files: false,
                allow_terminal_exec: false,
                allow_network_outbound: true,
                allow_hardware_flash: false,
                allow_git_commit: false,
            },
            AgentMode::Autonomous => Self {
                allow_read_files: true,
                allow_write_files: true,
                allow_terminal_exec: true,
                allow_network_outbound: true,
                allow_hardware_flash: true,
                allow_git_commit: true,
            },
        }
    }

    pub fn allows_action(&self, action: &str) -> bool {
        match action.to_lowercase().as_str() {
            "read" | "read_file" | "search" | "query_rag" | "tree_sitter" => self.allow_read_files,
            "write" | "write_file" | "apply_diff" | "apply_patch" => self.allow_write_files,
            "exec" | "terminal" | "cargo_check" | "cargo_clippy" | "cargo_test" => self.allow_terminal_exec,
            "web_search" | "fetch_docs" | "http" => self.allow_network_outbound,
            "flash" | "erase" | "probe_rs_flash" => self.allow_hardware_flash,
            "git_commit" | "git_branch" | "create_pr" => self.allow_git_commit,
            _ => true,
        }
    }
}
