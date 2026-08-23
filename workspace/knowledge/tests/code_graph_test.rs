use knowledge::{
    CodeEdgeType, CodeNode, CodeNodeType, ImpactAnalysisEngine, ImpactRiskLevel,
    MultiModalCodeGraph, SubgraphPruner,
};

#[test]
fn test_code_graph_topology_and_call_paths() {
    let mut graph = MultiModalCodeGraph::new();

    // Node 1: Entry API handler
    graph.add_node(CodeNode {
        id: "fn_handle_login".to_string(),
        name: "handle_login".to_string(),
        node_type: CodeNodeType::Function,
        file_path: "src/auth/handler.rs".to_string(),
        span_start: 10,
        span_end: 45,
        signature: "pub async fn handle_login(req: LoginRequest) -> HttpResponse".to_string(),
        doc_comment: Some("Authenticates user credentials".to_string()),
        vector_id: Some("vec-auth-1".to_string()),
    });

    // Node 2: Auth Service verify password
    graph.add_node(CodeNode {
        id: "fn_verify_password".to_string(),
        name: "verify_password".to_string(),
        node_type: CodeNodeType::Function,
        file_path: "src/auth/service.rs".to_string(),
        span_start: 50,
        span_end: 80,
        signature: "pub fn verify_password(hash: &str, raw: &str) -> bool".to_string(),
        doc_comment: None,
        vector_id: Some("vec-auth-2".to_string()),
    });

    // Node 3: DB Query
    graph.add_node(CodeNode {
        id: "fn_fetch_user".to_string(),
        name: "fetch_user".to_string(),
        node_type: CodeNodeType::Function,
        file_path: "src/db/user.rs".to_string(),
        span_start: 100,
        span_end: 130,
        signature: "pub async fn fetch_user(db: &SurrealClient, id: &str) -> Option<User>".to_string(),
        doc_comment: None,
        vector_id: Some("vec-db-1".to_string()),
    });

    // Edits: handle_login calls verify_password & fetch_user
    graph.add_edge("fn_handle_login", "fn_verify_password", CodeEdgeType::Calls, 1.0);
    graph.add_edge("fn_handle_login", "fn_fetch_user", CodeEdgeType::Calls, 1.0);

    let (nodes, edges) = graph.stats();
    assert_eq!(nodes, 3);
    assert_eq!(edges, 2);

    let callers = graph.get_callers("fn_verify_password");
    assert_eq!(callers.len(), 1);
    assert_eq!(callers[0].id, "fn_handle_login");

    let callees = graph.get_callees("fn_handle_login");
    assert_eq!(callees.len(), 2);
}

#[test]
fn test_impact_analysis_blast_radius() {
    let mut graph = MultiModalCodeGraph::new();

    graph.add_node(CodeNode {
        id: "db_client_core".to_string(),
        name: "SurrealClient".to_string(),
        node_type: CodeNodeType::Struct,
        file_path: "src/db/client.rs".to_string(),
        span_start: 1,
        span_end: 50,
        signature: "pub struct SurrealClient".to_string(),
        doc_comment: None,
        vector_id: None,
    });

    graph.add_node(CodeNode {
        id: "user_repo".to_string(),
        name: "UserRepository".to_string(),
        node_type: CodeNodeType::Struct,
        file_path: "src/db/user_repo.rs".to_string(),
        span_start: 10,
        span_end: 90,
        signature: "pub struct UserRepository".to_string(),
        doc_comment: None,
        vector_id: None,
    });

    graph.add_node(CodeNode {
        id: "auth_service".to_string(),
        name: "AuthService".to_string(),
        node_type: CodeNodeType::Struct,
        file_path: "src/auth/service.rs".to_string(),
        span_start: 20,
        span_end: 110,
        signature: "pub struct AuthService".to_string(),
        doc_comment: None,
        vector_id: None,
    });

    // user_repo references db_client_core; auth_service references user_repo
    graph.add_edge("user_repo", "db_client_core", CodeEdgeType::References, 1.0);
    graph.add_edge("auth_service", "user_repo", CodeEdgeType::References, 1.0);

    let report = ImpactAnalysisEngine::analyze(&graph, "db_client_core");
    assert_eq!(report.blast_radius_count, 2);
    assert!(report.impacted_nodes.contains(&"user_repo".to_string()));
    assert!(report.impacted_nodes.contains(&"auth_service".to_string()));
    assert_eq!(report.risk_level, ImpactRiskLevel::Low);
}

#[test]
fn test_subgraph_pruner_context_reduction() {
    let mut graph = MultiModalCodeGraph::new();

    graph.add_node(CodeNode {
        id: "target_fn".to_string(),
        name: "synthesize_firmware".to_string(),
        node_type: CodeNodeType::Function,
        file_path: "src/firmware/builder.rs".to_string(),
        span_start: 25,
        span_end: 60,
        signature: "pub fn synthesize_firmware(config: BuildConfig) -> Result<Vec<u8>, BuildError>".to_string(),
        doc_comment: Some("Compiles bare-metal no_std ELF".to_string()),
        vector_id: None,
    });

    graph.add_node(CodeNode {
        id: "caller_fn".to_string(),
        name: "api_build_firmware".to_string(),
        node_type: CodeNodeType::Function,
        file_path: "src/api/firmware.rs".to_string(),
        span_start: 1,
        span_end: 30,
        signature: "pub async fn api_build_firmware() -> HttpResponse".to_string(),
        doc_comment: None,
        vector_id: None,
    });

    graph.add_edge("caller_fn", "target_fn", CodeEdgeType::Calls, 1.0);

    let pruned = SubgraphPruner::prune(&graph, "target_fn", 1);
    assert!(pruned.center_node.is_some());
    assert_eq!(pruned.neighbor_nodes.len(), 1);
    assert_eq!(pruned.neighbor_nodes[0].id, "caller_fn");

    let md = pruned.to_markdown_context();
    assert!(md.contains("Topology-Aware Subgraph Context"));
    assert!(md.contains("synthesize_firmware"));
    assert!(md.contains("api_build_firmware"));
}
