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
    /// Discuss / Plan mode (OpenWorker-aligned): Strict read-only, all mutations & execs blocked
    Plan,
}

impl AgentMode {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "architect" => AgentMode::Architect,
            "ask" => AgentMode::Ask,
            "plan" | "discuss" => AgentMode::Plan,
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
            AgentMode::Plan => "plan",
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
            AgentMode::Plan => {
                "OPERATIONAL MODE: PLAN (GOVERNED READ-ONLY).\n\
                 You may inspect ASTs, read files, search the code graph, and draft actionable execution plans.\n\
                 All write tools, shell commands, and external side effects are strictly blocked."
            }
        }
    }
}

/// 5-Tier Intrinsic Risk Classes (Aligned with OpenWorker Governance Architecture)
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RiskClass {
    /// No side effects — reading AST, code graph, files, ripgrep (Always allowed)
    Read,
    /// Outbound network egress — Scrapling, Kitesurf, web fetch, docs lookup
    Egress,
    /// Local workspace mutations — write_file, apply_diff, replace_in_file (Path-scoped)
    WriteLocal,
    /// Shell and command execution — cargo check, kicad-cli, tests (Opaque construct guarded)
    Exec,
    /// External side effects — Hardware MCU flashing (probe-rs), remote SSH, persistent authority (save_skill)
    External,
}

impl RiskClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            RiskClass::Read => "read",
            RiskClass::Egress => "egress",
            RiskClass::WriteLocal => "write_local",
            RiskClass::Exec => "exec",
            RiskClass::External => "external",
        }
    }

    /// Strictness score for floor enforcement
    pub fn strictness(&self) -> u8 {
        match self {
            RiskClass::Read => 0,
            RiskClass::Egress => 1,
            RiskClass::WriteLocal => 2,
            RiskClass::Exec => 3,
            RiskClass::External => 4,
        }
    }

    /// Returns true if the tool call carries consequential side effects
    pub fn is_consequential(&self) -> bool {
        !matches!(self, RiskClass::Read)
    }
}

/// Classify a tool call into its intrinsic Risk Class
pub fn classify_tool_call(tool_name: &str) -> RiskClass {
    match tool_name.to_lowercase().as_str() {
        // Reads
        "read_file" | "view_file" | "list_dir" | "grep_search" | "query_rag" | "tree_sitter"
        | "ast_query" | "lookup_crate_api" | "check_status" => RiskClass::Read,

        // Egress (Outbound Network)
        "web_search" | "web_fetch" | "fetch_docs" | "perception_deep_research"
        | "perception_visual_verify" | "kitesurf_fetch" | "scrapling_fetch" | "http_request" => {
            RiskClass::Egress
        }

        // Write Local (Workspace Mutations)
        "write_file" | "write_to_file" | "replace_file_content" | "multi_replace_file_content"
        | "apply_diff" | "apply_patch" | "delete_file" | "create_file" => RiskClass::WriteLocal,

        // Exec (Command & Subprocess Execution)
        "run_command" | "exec" | "terminal" | "cargo_check" | "cargo_clippy" | "cargo_test"
        | "kicad_drc" | "qemu_run" | "perception_browser_action" => RiskClass::Exec,

        // External & Persistent Authority
        "probe_rs_flash" | "hardware_flash" | "erase_flash" | "remote_ssh_exec" | "save_skill"
        | "create_scheduled_task" | "update_scheduled_task" | "delete_scheduled_task"
        | "bwrap_synthesize_tool" => RiskClass::External,

        _ => RiskClass::Exec, // Conservative default
    }
}

/// Opaque Construct Guard (OpenWorker-Grade Shell Security)
/// Detects command injection, opaque variables, process substitutions, and dangerous flags
pub struct OpaqueConstructGuard;

impl OpaqueConstructGuard {
    /// Opaque constructs whose contents cannot be deterministically evaluated
    const OPAQUE_PATTERNS: &'static [&'static str] = &["`", "$(", "${", ">", "<", ">>", "|&"];

    /// Dangerous flags that convert search/list commands into deletions or executions
    const DANGEROUS_FLAGS: &'static [&'static str] = &[
        "-exec", "-execdir", "-delete", "-ok", "-okdir", "-fprintf", "-c", "-e", "--eval",
        "--command", "-Command",
    ];

    /// Programs that run other programs named in their arguments
    const ARG_EXECUTORS: &'static [&'static str] = &[
        "xargs", "env", "nohup", "nice", "stdbuf", "timeout", "watch", "sudo", "doas", "ssh",
        "docker", "podman", "kubectl", "npx", "bunx", "uvx",
    ];

    /// Check if a command string contains opaque expansions or unvetted redirection
    pub fn has_opaque_constructs(command: &str) -> bool {
        for pat in Self::OPAQUE_PATTERNS {
            if command.contains(pat) {
                return true;
            }
        }
        false
    }

    /// Check if a command includes dangerous execution/deletion flags or argument executors
    pub fn has_dangerous_constructs(command: &str) -> bool {
        if Self::has_opaque_constructs(command) {
            return true;
        }

        let words: Vec<&str> = command.split_whitespace().collect();
        for word in &words {
            let lower = word.to_lowercase();
            if Self::DANGEROUS_FLAGS.contains(&lower.as_str()) {
                return true;
            }
            if Self::ARG_EXECUTORS.contains(&lower.as_str()) {
                return true;
            }
        }

        false
    }
}

/// Granular Governance Decision
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum PermissionDecision {
    Allow,
    Deny { reason: String },
    NeedsApproval { reason: String, risk: RiskClass },
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
            AgentMode::Architect | AgentMode::Plan => Self {
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

    /// Comprehensive governance decision based on mode, tool risk, and arguments
    pub fn evaluate_execution(
        &self,
        mode: AgentMode,
        tool_name: &str,
        command_arg: Option<&str>,
        unattended: bool,
    ) -> PermissionDecision {
        let risk = classify_tool_call(tool_name);

        // 1. Pure Reads are always allowed in every mode
        if risk == RiskClass::Read {
            return PermissionDecision::Allow;
        }

        // 2. Plan / Architect / Ask modes block all consequential actions
        if matches!(mode, AgentMode::Plan | AgentMode::Architect | AgentMode::Ask) {
            return PermissionDecision::Deny {
                reason: format!(
                    "Tool '{}' with risk {:?} is blocked in {} mode",
                    tool_name,
                    risk,
                    mode.as_str()
                ),
            };
        }

        // 3. Opaque Construct / Shell Injection Defense
        if risk == RiskClass::Exec {
            if let Some(cmd) = command_arg {
                if OpaqueConstructGuard::has_dangerous_constructs(cmd) {
                    return PermissionDecision::NeedsApproval {
                        reason: format!(
                            "Command '{}' contains opaque or dangerous constructs (pipes, expansions, or flags)",
                            cmd
                        ),
                        risk,
                    };
                }
            }
        }

        // 4. External / Physical Hardware Flashing Always Requires Human Approval
        if risk == RiskClass::External {
            return PermissionDecision::NeedsApproval {
                reason: format!(
                    "Tool '{}' performs persistent or external hardware operations",
                    tool_name
                ),
                risk,
            };
        }

        // 5. Unattended Mode: If not pure read and consequential, route to Inbox
        if unattended && risk.is_consequential() {
            return PermissionDecision::NeedsApproval {
                reason: format!(
                    "Unattended run encountered consequential action '{}' ({:?})",
                    tool_name, risk
                ),
                risk,
            };
        }

        // 6. Check individual mode capability flags
        match risk {
            RiskClass::WriteLocal if !self.allow_write_files => PermissionDecision::Deny {
                reason: "File writes are disabled in current permission profile".to_string(),
            },
            RiskClass::Exec if !self.allow_terminal_exec => PermissionDecision::Deny {
                reason: "Terminal execution is disabled in current permission profile".to_string(),
            },
            RiskClass::Egress if !self.allow_network_outbound => PermissionDecision::Deny {
                reason: "Network egress is disabled in current permission profile".to_string(),
            },
            _ => PermissionDecision::Allow,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_risk_classification() {
        assert_eq!(classify_tool_call("read_file"), RiskClass::Read);
        assert_eq!(classify_tool_call("grep_search"), RiskClass::Read);
        assert_eq!(classify_tool_call("web_search"), RiskClass::Egress);
        assert_eq!(classify_tool_call("perception_deep_research"), RiskClass::Egress);
        assert_eq!(classify_tool_call("write_file"), RiskClass::WriteLocal);
        assert_eq!(classify_tool_call("run_command"), RiskClass::Exec);
        assert_eq!(classify_tool_call("probe_rs_flash"), RiskClass::External);
        assert_eq!(classify_tool_call("save_skill"), RiskClass::External);
    }

    #[test]
    fn test_opaque_construct_guard() {
        assert!(OpaqueConstructGuard::has_opaque_constructs("echo $(cat /etc/passwd)"));
        assert!(OpaqueConstructGuard::has_opaque_constructs("echo `whoami`"));
        assert!(OpaqueConstructGuard::has_opaque_constructs("cat foo > /dev/sda"));
        assert!(!OpaqueConstructGuard::has_opaque_constructs("cargo check --target thumbv7em-none-eabihf"));
        assert!(OpaqueConstructGuard::has_dangerous_constructs("find . -name '*.rs' -delete"));
        assert!(OpaqueConstructGuard::has_dangerous_constructs("sudo systemctl restart"));
        assert!(OpaqueConstructGuard::has_dangerous_constructs("python -c 'import os; os.system(\"rm -rf /\")'"));
    }

    #[test]
    fn test_plan_mode_blocks_writes() {
        let perms = ToolPermissions::for_mode(AgentMode::Plan);
        let dec = perms.evaluate_execution(AgentMode::Plan, "write_file", None, false);
        assert!(matches!(dec, PermissionDecision::Deny { .. }));
        let dec_read = perms.evaluate_execution(AgentMode::Plan, "read_file", None, false);
        assert_eq!(dec_read, PermissionDecision::Allow);
    }

    #[test]
    fn test_unattended_parks_consequential() {
        let perms = ToolPermissions::for_mode(AgentMode::Code);
        let dec = perms.evaluate_execution(AgentMode::Code, "write_file", None, true);
        assert!(matches!(dec, PermissionDecision::NeedsApproval { .. }));
    }
}
