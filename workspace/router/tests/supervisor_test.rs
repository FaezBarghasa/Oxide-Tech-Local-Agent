use router::{AgentMode, SubAgentRole, SupervisorAgent, TaskStatus};

#[test]
fn test_supervisor_agent_modes_and_permissions() {
    let code_agent = SupervisorAgent::new(AgentMode::Code);
    assert!(code_agent.permissions.allows_action("write_file"));
    assert!(code_agent.permissions.allows_action("cargo_check"));
    assert!(!code_agent.permissions.allows_action("probe_rs_flash"));

    let arch_agent = SupervisorAgent::new(AgentMode::Architect);
    assert!(arch_agent.permissions.allows_action("read_file"));
    assert!(!arch_agent.permissions.allows_action("write_file"));

    let auto_agent = SupervisorAgent::new(AgentMode::Autonomous);
    assert!(auto_agent.permissions.allows_action("probe_rs_flash"));
    assert!(auto_agent.permissions.allows_action("git_commit"));
}

#[test]
fn test_supervisor_plan_dag_and_dependencies() {
    let supervisor = SupervisorAgent::new(AgentMode::Autonomous);
    let dag = supervisor.plan_goal("Implement high-precision SPI gyro driver and verify with cargo test");

    assert_eq!(dag.execution_order.len(), 4);
    assert!(dag.is_ready("plan"));
    assert!(!dag.is_ready("implement")); // requires plan to pass

    let mut dag_mut = dag;
    if let Some(plan_node) = dag_mut.nodes.get_mut("plan") {
        plan_node.status = TaskStatus::Passed;
    }

    assert!(dag_mut.is_ready("implement"));
    assert_eq!(dag_mut.nodes.get("implement").unwrap().role, SubAgentRole::Coder);
}

#[test]
fn test_oscillation_guard() {
    let mut supervisor = SupervisorAgent::new(AgentMode::Code);
    assert!(!supervisor.detect_oscillation("error[E0425]: cannot find value"));
    assert!(!supervisor.detect_oscillation("error[E0425]: cannot find value"));
    // Third identical error in a row triggers oscillation guard
    assert!(supervisor.detect_oscillation("error[E0425]: cannot find value"));
}
