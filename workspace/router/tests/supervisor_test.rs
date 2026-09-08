use router::{AgentMode, SubAgentRole, SupervisorAgent, TaskStatus};
use uuid::Uuid;

#[test]
fn test_supervisor_agent_modes_and_permissions() {
    let code_agent = SupervisorAgent::new(AgentMode::Code);
    assert!(code_agent.permissions.allow_write_files);
    assert!(code_agent.permissions.allow_terminal_exec);
    assert!(!code_agent.permissions.allow_hardware_flash);

    let arch_agent = SupervisorAgent::new(AgentMode::Architect);
    assert!(arch_agent.permissions.allow_read_files);
    assert!(!arch_agent.permissions.allow_write_files);

    let auto_agent = SupervisorAgent::new(AgentMode::Autonomous);
    assert!(auto_agent.permissions.allow_hardware_flash);
    assert!(auto_agent.permissions.allow_git_commit);
}

#[tokio::test]
async fn test_supervisor_plan_dag_and_dependencies() {
    let supervisor = SupervisorAgent::new(AgentMode::Autonomous);
    let dag = supervisor
        .plan_goal("Implement high-precision SPI gyro driver and verify with cargo test")
        .await;

    assert_eq!(dag.execution_order.len(), 4);
    assert!(dag.is_ready("plan"));
    assert!(!dag.is_ready("implement")); // requires plan to pass

    let mut dag_mut = dag;
    if let Some(plan_node) = dag_mut.nodes.get_mut("plan") {
        plan_node.status = TaskStatus::Passed;
    }

    assert!(dag_mut.is_ready("implement"));
    assert_eq!(
        dag_mut.nodes.get("implement").unwrap().role,
        SubAgentRole::Coder
    );
}

#[tokio::test]
async fn test_oscillation_guard() {
    let mut supervisor = SupervisorAgent::new(AgentMode::Code);
    let dag_id = Uuid::new_v4();
    assert!(
        !supervisor
            .detect_oscillation(dag_id, "task1", "error[E0425]: cannot find value")
            .await
    );
    assert!(
        !supervisor
            .detect_oscillation(dag_id, "task1", "error[E0425]: cannot find value")
            .await
    );
    // Third identical error in a row triggers oscillation guard
    assert!(
        supervisor
            .detect_oscillation(dag_id, "task1", "error[E0425]: cannot find value")
            .await
    );
}
