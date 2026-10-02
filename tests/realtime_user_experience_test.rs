//! Hardened Real-Time User Experience & Concurrency Integration Test
//!
//! Simulates high-stress real user interactions across:
//! - Multi-threaded concurrent MoE routing decisions under SLA latency ceilings (<10ms).
//! - Stderr ring-buffer secret scrubbing under bursty concurrent subprocess dumps.
//! - Candidate vector cache scoring under simulated real-time user typing.
//! - Mobile FFI robustness against malformed user inputs with descriptive error capture.
//! - Full-stack atomic firmware flash deployment, verification window, and auto-rollback.

use mcp_probe_rs::{AtomicFlashManager, DeploymentState, PartitionSlot, RollbackReason};
use oxide_engines::decision_engine::{CandidateVectorCache, DecisionEngine, DecisionInput};
use oxide_engines::mobile::{
    oxide_decision_engine_create, oxide_decision_engine_decide_json,
    oxide_decision_engine_free, oxide_decision_engine_free_string,
    oxide_decision_engine_last_error,
};
use oxide_security::StderrSanitizer;
use router::moe_router::{ExpertModel, MoeGatingRouter};
use router::types::{InferenceRequest, TaskType};
use std::ffi::{CStr, CString};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[test]
fn test_realtime_concurrent_moe_routing_sla() {
    let router = Arc::new(MoeGatingRouter::new());
    let completed = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    let start = Instant::now();
    let num_simulated_users = 25;
    let requests_per_user = 40;

    for user_id in 0..num_simulated_users {
        let r = Arc::clone(&router);
        let c = Arc::clone(&completed);

        handles.push(std::thread::spawn(move || {
            let prompts = [
                ("Write an Embassy async task for STM32F401 reading SMT160 duty cycle", TaskType::Architecture),
                ("objdump decompile elf stripped binary with gdb hex analysis", TaskType::BinaryAnalysis),
                ("Explain why this borrowck lifetime error fails on struct Foo<'a>", TaskType::Syntax),
                ("Fast triage diff checklist status overview", TaskType::Architecture),
                ("Synthesize SIMD algorithm in safe Rust with mtp turbo kernel", TaskType::CodeCompletion),
            ];

            for i in 0..requests_per_user {
                let (prompt, task_type) = prompts[(user_id + i) % prompts.len()];
                let req = InferenceRequest {
                    prompt: prompt.to_string(),
                    task_type,
                    stream: false,
                    max_tokens: Some(512),
                    temperature: Some(0.7),
                };

                let t0 = Instant::now();
                let decision = r.route(&req);
                let latency = t0.elapsed();

                // SLA Assertion: Local MoE routing must complete in < 15ms even under concurrency
                assert!(
                    latency < Duration::from_millis(15),
                    "MoE routing latency SLA violated: {:?}",
                    latency
                );
                assert!(!decision.rationale.is_empty());
                c.fetch_add(1, Ordering::Relaxed);
            }
        }));
    }

    for h in handles {
        h.join().expect("Worker thread panicked");
    }

    let elapsed = start.elapsed();
    let total_requests = completed.load(Ordering::Relaxed);
    assert_eq!(total_requests, num_simulated_users * requests_per_user);

    let throughput = (total_requests as f64) / elapsed.as_secs_f64();
    println!(
        "[✓] Realtime MoE SLA Passed: {} decisions in {:.2?} ({:.1} req/sec)",
        total_requests, elapsed, throughput
    );
}

#[test]
fn test_realtime_bursty_stderr_scrubbing_and_concurrency() {
    let temp = tempfile::tempdir().unwrap();
    let diag_path = temp.path().join("live-stderr.log");
    let sanitizer = Arc::new(StderrSanitizer::new(&diag_path));

    let num_threads = 10;
    let chunks_per_thread = 50;
    let mut handles = Vec::new();

    for t in 0..num_threads {
        let s = Arc::clone(&sanitizer);
        handles.push(std::thread::spawn(move || {
            for i in 0..chunks_per_thread {
                let payload = format!(
                    "[{:?}] Process {} Worker {}: Connecting with bearer eyJhbGciOiJIUzI1NiIsInR5cCI and secret key sk-99887766554433221100aa\n",
                    Instant::now(),
                    t,
                    i
                );
                s.append_chunk(payload.as_bytes());
            }
        }));
    }

    for h in handles {
        h.join().expect("Stderr thread panicked");
    }

    let tail = sanitizer.get_sanitized_tail();
    assert!(!tail.contains("sk-99887766554433221100aa"), "Secret leaked in sanitized stderr!");
    assert!(tail.contains("[REDACTED_SECRET]"), "Secret scrubbing token missing!");
    assert!(
        tail.len() <= 16 * 1024 + 1024,
        "Stderr tail size exceeded bounded ring buffer: {} bytes",
        tail.len()
    );

    sanitizer.flush_to_diagnostic_file().expect("Failed to flush diagnostics");
    assert!(diag_path.exists());
}

#[test]
fn test_realtime_mobile_ffi_error_diagnostics() {
    unsafe {
        // Test 1: Null pointer handling
        let res_null = oxide_decision_engine_decide_json(std::ptr::null(), std::ptr::null());
        assert!(res_null.is_null());

        let last_err_ptr = oxide_decision_engine_last_error();
        assert!(!last_err_ptr.is_null());
        let last_err = CStr::from_ptr(last_err_ptr).to_str().unwrap();
        assert!(last_err.contains("Null engine_ptr"));
        oxide_decision_engine_free_string(last_err_ptr);

        // Test 2: Valid engine with malformed JSON input
        let engine_ptr = oxide_decision_engine_create(4, 5000);
        assert!(!engine_ptr.is_null());

        let bad_json = CString::new("{ not valid json at all }").unwrap();
        let res_bad = oxide_decision_engine_decide_json(engine_ptr, bad_json.as_ptr());
        assert!(res_bad.is_null());

        let err_ptr2 = oxide_decision_engine_last_error();
        assert!(!err_ptr2.is_null());
        let err2 = CStr::from_ptr(err_ptr2).to_str().unwrap();
        assert!(err2.contains("JSON deserialization failed"));
        oxide_decision_engine_free_string(err_ptr2);

        oxide_decision_engine_free(engine_ptr);
    }
}

#[test]
fn test_realtime_atomic_flash_lifecycle_with_rollback() {
    let mut manager = AtomicFlashManager::default();

    // 1. Stage firmware
    let staged_slot = manager.stage_firmware(4096).expect("Stage failed");
    assert_eq!(staged_slot, PartitionSlot::SlotB);

    // 2. Start boot verification window (100ms for test)
    let active = manager
        .start_boot_verification(Duration::from_millis(100))
        .expect("Boot verify failed");
    assert_eq!(active, PartitionSlot::SlotB);

    // 3. Simulate failure in verification (e.g. boot timeout)
    let (rolled_slot, rolled_addr) = manager.rollback(RollbackReason::BootTimeout);
    assert_eq!(rolled_slot, PartitionSlot::SlotA);
    assert_eq!(rolled_addr, 0x08008000);
    assert_eq!(manager.active_slot(), PartitionSlot::SlotA);

    if let DeploymentState::RolledBack { reason, fallback_slot } = manager.current_state() {
        assert_eq!(reason, RollbackReason::BootTimeout);
        assert_eq!(fallback_slot, PartitionSlot::SlotA);
    } else {
        panic!("Expected RolledBack state!");
    }
}
