use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use web_forge::{
    AXNode, AXTree, ActionabilityConfig, ActionabilityEvaluator, BoundingBox, DomDistiller,
    ElementLayoutState, HitTestResult, InterceptDecision, InterceptedRequest, LayoutInspector,
    MacroCodegen, MockResponse, MockRule, NetworkInterceptor, WebAction, WebForgeError, WebScript,
};

/// Mock Layout Inspector for deterministic testing of the Actionability Engine.
struct MockInspector {
    element_state: Option<ElementLayoutState>,
    hit_result: Option<HitTestResult>,
}

impl LayoutInspector for MockInspector {
    fn query_element_state(
        &self,
        _selector: &str,
    ) -> Result<Option<ElementLayoutState>, WebForgeError> {
        Ok(self.element_state.clone())
    }

    fn perform_hit_test(&self, _x: f64, _y: f64) -> Result<Option<HitTestResult>, WebForgeError> {
        Ok(self.hit_result.clone())
    }
}

/// Dynamic Mock Inspector that simulates an element moving/animating and then settling.
struct AnimatingMockInspector {
    tick_count: Arc<AtomicUsize>,
}

impl LayoutInspector for AnimatingMockInspector {
    fn query_element_state(
        &self,
        selector: &str,
    ) -> Result<Option<ElementLayoutState>, WebForgeError> {
        let count = self.tick_count.fetch_add(1, Ordering::SeqCst);
        let x_pos = if count < 2 {
            (count as f64) * 20.0 // Moving: 0.0, 20.0
        } else {
            50.0 // Settled at 50.0
        };

        Ok(Some(ElementLayoutState::new_actionable(
            101,
            selector,
            BoundingBox::new(x_pos, 100.0, 120.0, 40.0),
        )))
    }

    fn perform_hit_test(&self, _x: f64, _y: f64) -> Result<Option<HitTestResult>, WebForgeError> {
        Ok(Some(HitTestResult {
            hit_node_id: 101,
            hit_selector: "#submit-btn".to_string(),
        }))
    }
}

#[tokio::test]
async fn test_actionability_all_checks_passed() {
    let bbox = BoundingBox::new(100.0, 150.0, 80.0, 32.0);
    let inspector = MockInspector {
        element_state: Some(ElementLayoutState::new_actionable(42, "#login-btn", bbox)),
        hit_result: Some(HitTestResult {
            hit_node_id: 42,
            hit_selector: "#login-btn".to_string(),
        }),
    };

    let evaluator = ActionabilityEvaluator::new();
    let config = ActionabilityConfig {
        timeout: Duration::from_millis(500),
        poll_interval: Duration::from_millis(10),
        stability_samples: 2,
        ..Default::default()
    };

    let result = evaluator
        .wait_for_actionable(&inspector, "#login-btn", &config)
        .await;

    assert!(result.is_ok(), "Expected actionability to pass");
    let state = result.unwrap();
    assert_eq!(state.node_id, 42);
    assert_eq!(state.selector, "#login-btn");
}

#[tokio::test]
async fn test_actionability_occluded_element_timeout() {
    let bbox = BoundingBox::new(100.0, 150.0, 80.0, 32.0);
    // Button is at node 42, but hit-test hits modal overlay at node 999
    let inspector = MockInspector {
        element_state: Some(ElementLayoutState::new_actionable(42, "#login-btn", bbox)),
        hit_result: Some(HitTestResult {
            hit_node_id: 999,
            hit_selector: "#modal-backdrop".to_string(),
        }),
    };

    let evaluator = ActionabilityEvaluator::new();
    let config = ActionabilityConfig {
        timeout: Duration::from_millis(100),
        poll_interval: Duration::from_millis(20),
        stability_samples: 1,
        ..Default::default()
    };

    let result = evaluator
        .wait_for_actionable(&inspector, "#login-btn", &config)
        .await;

    assert!(result.is_err(), "Expected occluded element to fail");
    match result.unwrap_err() {
        WebForgeError::ElementOccluded {
            selector,
            hit_element,
        } => {
            assert_eq!(selector, "#login-btn");
            assert_eq!(hit_element, "#modal-backdrop");
        }
        WebForgeError::ActionabilityTimeout(_) => {
            // Also acceptable if timeout expires
        }
        other => panic!("Unexpected error type: {:?}", other),
    }
}

#[tokio::test]
async fn test_actionability_unstable_animation_settling() {
    let inspector = AnimatingMockInspector {
        tick_count: Arc::new(AtomicUsize::new(0)),
    };

    let evaluator = ActionabilityEvaluator::new();
    let config = ActionabilityConfig {
        timeout: Duration::from_millis(800),
        poll_interval: Duration::from_millis(20),
        stability_samples: 3, // Requires 3 stable consecutive frames
        stability_epsilon: 0.1,
        ..Default::default()
    };

    let result = evaluator
        .wait_for_actionable(&inspector, "#submit-btn", &config)
        .await;

    assert!(
        result.is_ok(),
        "Expected auto-wait to succeed after settling"
    );
    let state = result.unwrap();
    assert_eq!(state.bounding_box.unwrap().x, 50.0);
}

#[test]
fn test_axtree_distillation_token_reduction() {
    let mut tree = AXTree::new();
    tree.add_node(AXNode::new(1, "heading").with_name("Cluster Metrics Overview"));
    tree.add_node(
        AXNode::new(2, "textbox")
            .with_name("Search pods")
            .with_value("core-agent"),
    );
    tree.add_node(AXNode::new(3, "button").with_name("Deploy Application"));
    tree.add_node(
        AXNode::new(4, "button")
            .with_name("Purge Cache")
            .with_disabled(true),
    );

    // Add an ignored decorative SVG/div container
    let mut ignored_node = AXNode::new(5, "generic");
    ignored_node.ignored = true;
    tree.add_node(ignored_node);

    let distilled_md = DomDistiller::distill_axtree(&tree);

    assert!(distilled_md.contains("## Interactive Page State"));
    assert!(distilled_md.contains("- [1] heading 'Cluster Metrics Overview'"));
    assert!(distilled_md.contains("- [2] textbox 'Search pods' value='core-agent'"));
    assert!(distilled_md.contains("- [3] button 'Deploy Application'"));
    assert!(distilled_md.contains("- [4] button 'Purge Cache' (disabled)"));
    assert!(!distilled_md.contains("[5]")); // Ignored node omitted

    // Test HTML parsing fallback
    let raw_html = r#"
        <div style="display:none">Hidden noise</div>
        <h1>Telemetry Console</h1>
        <input type="text" placeholder="Node IP" value="192.168.1.100" />
        <button disabled>Reboot Node</button>
    "#;
    let parsed_tree = DomDistiller::parse_semantic_html(raw_html);
    let html_distilled = DomDistiller::distill_axtree(&parsed_tree);

    assert!(html_distilled.contains("Telemetry Console"));
    assert!(html_distilled.contains("Node IP"));
    assert!(html_distilled.contains("Reboot Node"));
    assert!(html_distilled.contains("disabled"));
}

#[test]
fn test_causal_network_interception_mock() {
    let mut interceptor = NetworkInterceptor::new();

    // 1. Inject 500 error fault on telemetry endpoint
    let rule = MockRule::new("rule-telemetry-fault", r#".*/api/telemetry.*"#).with_method("GET");
    let response = MockResponse::error_500("Simulated Crucible Fault Injection");
    interceptor.add_rule(rule, response);

    // 2. Normal non-matching request
    let normal_req = InterceptedRequest::new("https://app.oxide.tech/api/auth/session", "GET");
    let decision1 = interceptor.intercept(normal_req);
    assert_eq!(decision1, InterceptDecision::Continue);

    // 3. Faulted request matching rule
    let target_req = InterceptedRequest::new("https://app.oxide.tech/api/telemetry/stats", "GET");
    let decision2 = interceptor.intercept(target_req);

    match decision2 {
        InterceptDecision::Mock(mock_res) => {
            assert_eq!(mock_res.status_code, 500);
            let body_str = String::from_utf8_lossy(&mock_res.body);
            assert!(body_str.contains("Simulated Crucible Fault Injection"));
        }
        other => panic!("Expected Mock decision, got: {:?}", other),
    }

    assert_eq!(interceptor.history().len(), 2);
}

#[test]
fn test_codegen_macro_synthesis() {
    let mut script = WebScript::new("LoginFlow");
    script.add_action(WebAction::Navigate {
        url: "https://cloud.oxide.tech/auth".to_string(),
    });
    script.add_action(WebAction::WaitForSelector {
        selector: "#user-email".to_string(),
        timeout_ms: 3000,
    });
    script.add_action(WebAction::Type {
        selector: "#user-email".to_string(),
        text: "operator@oxide.tech".to_string(),
    });
    script.add_action(WebAction::Click {
        selector: "#btn-login".to_string(),
    });
    script.add_action(WebAction::AssertText {
        selector: ".dashboard-title".to_string(),
        expected: "Welcome back".to_string(),
    });

    let rust_code = MacroCodegen::generate_rust_code(&script);
    assert!(rust_code.contains("pub async fn execute_flow"));
    assert!(rust_code.contains("harness.navigate(\"https://cloud.oxide.tech/auth\").await?"));
    assert!(
        rust_code.contains("harness.type_text(\"#user-email\", \"operator@oxide.tech\").await?")
    );
    assert!(rust_code.contains("harness.click(\"#btn-login\").await?"));

    let macro_code = MacroCodegen::generate_macro(&script);
    assert!(macro_code.contains("macro_rules! loginflow"));
    assert!(macro_code.contains("$harness.click(\"#btn-login\").await?"));
}
