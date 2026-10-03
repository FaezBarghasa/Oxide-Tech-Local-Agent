use oxide_gateway::db::GatewayDb;
use oxide_gateway::gecko_driver::GeckoSession;
use oxide_gateway::rate_pacer::RatePacer;
use oxide_gateway::universal_router::ModelResolver;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;

#[tokio::test]
async fn test_gateway_db_initialization_and_seeded_combos() {
    let db = GatewayDb::init_mem()
        .await
        .expect("Failed to init in-memory GatewayDb");
    let combos = db.list_combos().await.expect("Failed to list combos");
    assert!(!combos.is_empty(), "Combos should be pre-seeded");

    let auto_combo = db
        .get_combo("auto")
        .await
        .expect("query auto")
        .expect("auto combo missing");
    assert_eq!(auto_combo.name, "auto");
    assert_eq!(auto_combo.strategy, "lkgp");
    assert_eq!(auto_combo.targets.len(), 4);

    let coding_combo = db
        .get_combo("auto/coding")
        .await
        .expect("query coding")
        .expect("coding combo missing");
    assert_eq!(coding_combo.name, "auto/coding");
    assert_eq!(coding_combo.strategy, "priority");

    let models = db.list_models().await.expect("Failed to list models");
    assert!(!models.is_empty(), "Model catalog should be pre-seeded");
    assert!(models.iter().any(|m| m.model_id == "claude-3-7-sonnet"));
    assert!(models.iter().any(|m| m.model_id == "deepseek-reasoner"));
}

#[tokio::test]
async fn test_universal_model_resolver_combos() {
    let db = Arc::new(GatewayDb::init_mem().await.expect("init mem db"));
    let rr = Arc::new(AtomicUsize::new(0));

    // Test virtual combos
    let target_auto = ModelResolver::resolve("auto", &db, &rr)
        .await
        .expect("resolve auto");
    assert_eq!(target_auto.provider, "anthropic");
    assert_eq!(target_auto.model, "claude-3-7-sonnet");

    let target_coding = ModelResolver::resolve("auto/coding", &db, &rr)
        .await
        .expect("resolve auto/coding");
    assert_eq!(target_coding.provider, "anthropic");

    let target_fast = ModelResolver::resolve("auto/fast", &db, &rr)
        .await
        .expect("resolve auto/fast");
    assert_eq!(target_fast.provider, "cerebras");
    assert_eq!(target_fast.model, "llama-3.1-8b");

    let target_cheap = ModelResolver::resolve("auto/cheap", &db, &rr)
        .await
        .expect("resolve auto/cheap");
    assert_eq!(target_cheap.provider, "google");
}

#[tokio::test]
async fn test_universal_model_resolver_aliases_and_core_ids() {
    let db = Arc::new(GatewayDb::init_mem().await.expect("init mem db"));
    let rr = Arc::new(AtomicUsize::new(0));

    // OpenRouter router aliases
    let or_auto = ModelResolver::resolve("openrouter/auto", &db, &rr)
        .await
        .expect("resolve openrouter/auto");
    assert_eq!(or_auto.model, "claude-3-7-sonnet");

    // Provider prefixed IDs
    let deepseek_prefixed = ModelResolver::resolve("deepseek/deepseek-r1:free", &db, &rr)
        .await
        .expect("resolve deepseek prefix");
    assert_eq!(deepseek_prefixed.provider, "deepseek");
    assert_eq!(deepseek_prefixed.model, "deepseek-r1");
    assert!(deepseek_prefixed.supports_thinking);

    // Normalized Core IDs
    let (p, m, thinking, ctx) = ModelResolver::map_core_id_to_provider("claude-opus-5");
    assert_eq!(p, "anthropic");
    assert_eq!(m, "claude-opus-5");
    assert!(thinking);
    assert_eq!(ctx, 500_000);

    let (p, m, thinking, ctx) = ModelResolver::map_core_id_to_provider("gpt-5.6-sol");
    assert_eq!(p, "openai");
    assert_eq!(m, "gpt-5.6-sol");
    assert!(thinking);
    assert_eq!(ctx, 1_000_000);

    let (p, m, thinking, _) = ModelResolver::map_core_id_to_provider("deepseek-reasoner");
    assert_eq!(p, "deepseek");
    assert_eq!(m, "deepseek-reasoner");
    assert!(thinking);

    let (p, _m, _, ctx) = ModelResolver::map_core_id_to_provider("gemini-2.5-pro");
    assert_eq!(p, "google");
    assert_eq!(ctx, 2_000_000);

    let (p, m, _, _) = ModelResolver::map_core_id_to_provider("qwen-2.5-coder-32b");
    assert_eq!(p, "qwen");
    assert_eq!(m, "qwen-2.5-coder-32b-instruct");
}

#[test]
fn test_rate_pacer_exponential_backoff_and_recovery() {
    let pacer = RatePacer::new();
    assert!(pacer.is_provider_available("openai"));

    // Report 429
    let backoff1 = pacer.report_429("openai");
    assert_eq!(backoff1, 4); // 2 * 2^1
    assert!(!pacer.is_provider_available("openai"));

    // Other providers remain available
    assert!(pacer.is_provider_available("anthropic"));

    // Success resets rate state
    pacer.report_success("openai");
}

#[tokio::test]
async fn test_session_relay_checkpoint() {
    let db = GatewayDb::init_mem().await.expect("init mem db");
    let session_id = "sess_test_1234";
    let summary = "Agent verified AST syntax and synthesized fast FP8 kernel";

    db.save_session_relay(session_id, summary, 1280)
        .await
        .expect("save relay");
    let loaded = db
        .get_session_relay(session_id)
        .await
        .expect("get relay")
        .expect("relay missing");

    assert_eq!(loaded.session_id, session_id);
    assert_eq!(loaded.handoff_summary, summary);
    assert_eq!(loaded.token_count, 1280);
}

#[test]
fn test_gecko_session_instantiation() {
    let session = GeckoSession::new(2828, PathBuf::from("/tmp/firefox_oxide_test"));
    assert_eq!(session.port, 2828);
    assert_eq!(
        session.profile_path,
        PathBuf::from("/tmp/firefox_oxide_test")
    );
    assert!(session.process.is_none());
}
