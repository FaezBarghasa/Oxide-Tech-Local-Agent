//! oxide-tech-local-agent — Universal Single Production Binary.
//!
//! One binary runs everything:
//!   - `oxide-tech-local-agent` (no args) or `oxide-tech-local-agent desktop` → Tauri desktop window
//!     with the embedded gateway (background thread), full workstation UI, and oxide-embed memory.
//!   - `oxide-tech-local-agent daemon [--config PATH]` → headless gateway service (systemd unit).
//!   - `oxide-tech-local-agent doctor`                 → comprehensive environment diagnostics.
//!   - `oxide-tech-local-agent re-forge <file>`         → pure-Rust binary / PTX GPU reverse engineering.
//!   - `oxide-tech-local-agent verify [--workspace .]`  → deterministic verifier suite & evidence bundle.
//!   - `oxide-tech-local-agent memory <args...>` / `embed <args...>` → STAIR Code-ToC & Memanto memory.
//!   - `oxide-tech-local-agent status`                 → probe the local gateway.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config_ipc;
mod doctor;
mod gateway_rt;
mod hardware_ipc;
mod memory;
mod model_ipc;
mod reforge_ipc;
mod verifier_ipc;

use anyhow::Result;
use std::path::PathBuf;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const DEFAULT_GATEWAY_URL: &str = "http://127.0.0.1:8080";

fn usage() -> String {
    format!(
        "oxide-tech-local-agent {VERSION} — Oxide-Tech Local Agent OS (Single-Binary Workstation)\n\
         \n\
         Usage:\n  \
           oxide-tech-local-agent [desktop] [--config PATH]       Launch Desktop UI (Embedded Gateway + Memory)\n  \
           oxide-tech-local-agent daemon [--config PATH]          Run Headless Gateway (Systemd Service Mode)\n  \
           oxide-tech-local-agent doctor [--json]                 Run Environment & Toolchain Diagnostics\n  \
           oxide-tech-local-agent re-forge <FILE> [--arch ARCH]   Reverse Engineer Binary / PTX GPU Code\n  \
           oxide-tech-local-agent verify [--workspace PATH]       Run Deterministic Verifier Suite\n  \
           oxide-tech-local-agent memory <subcommand> [args...]   STAIR Code-ToC & Memanto Memory Passthrough\n  \
           oxide-tech-local-agent embed <subcommand> [args...]    Alias for oxide-embed commands\n  \
           oxide-tech-local-agent status [--gateway-url URL]      Probe Running Gateway Liveness\n  \
           oxide-tech-local-agent --help | --version"
    )
}

/// Extract `--flag value` from a raw arg slice.
fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1].clone())
}

fn run_desktop(config: Option<String>) {
    if std::env::var("SUDO_USER").is_ok() {
        eprintln!(
            "\x1b[1;33m[!] WARNING: oxide-tech-local-agent desktop should NOT be run with 'sudo'.\x1b[0m\n\
             Running WebKit/GTK desktop apps as root creates root-owned caches in ~/.config/ and ~/.local/,\n\
             which can disrupt your desktop session. Run 'oxide-tech-local-agent' as your regular user."
        );
    }

    // Structured logs go to stdout; the WebView renders the UI.
    let _ = tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .try_init();

    gateway_rt::spawn_background(config);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            // Memory & STAIR Code-ToC
            memory::memory_env,
            memory::memory_status,
            memory::memory_init,
            memory::memory_index,
            memory::memory_search,
            memory::memory_context,
            memory::memory_remember,
            memory::memory_recall,
            memory::memory_explain,
            memory::memory_conflicts,
            // Gateway
            gateway_status,
            // Doctor
            doctor::doctor_run_diagnostics,
            doctor::doctor_install_udev_rules,
            // Hardware & probe-rs
            hardware_ipc::hardware_list_probes,
            hardware_ipc::hardware_get_chip_info,
            hardware_ipc::hardware_flash_firmware,
            // Model & Unsloth-Style Execution
            model_ipc::model_list_available,
            model_ipc::model_run_prompt,
            // RE-Forge
            reforge_ipc::reforge_analyze_file,
            // Verifier
            verifier_ipc::verifier_run_suite,
            verifier_ipc::verifier_export_evidence,
            // Config
            config_ipc::config_read,
            config_ipc::config_save,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Oxide-Tech Local Agent desktop");
}

#[tauri::command]
async fn gateway_status(base_url: Option<String>) -> Result<u16, String> {
    let base = base_url.unwrap_or_else(|| DEFAULT_GATEWAY_URL.to_string());
    gateway_rt::probe_gateway(&base, 3).await
}

fn run_doctor_cli(json_output: bool) {
    let res = doctor::run_diagnostics_scan();
    if json_output {
        println!("{}", serde_json::to_string_pretty(&res).unwrap_or_default());
    } else {
        println!(
            "\x1b[1;34m====================================================================\x1b[0m"
        );
        println!(
            "\x1b[1;36m   Oxide-Tech Local Agent OS — System & Environment Diagnostics     \x1b[0m"
        );
        println!(
            "\x1b[1;34m====================================================================\x1b[0m\n"
        );

        for c in &res.checks {
            if c.passed {
                println!("  \x1b[1;32m[OK]\x1b[0m {:<35} : {}", c.name, c.version);
            } else if c.required {
                println!(
                    "  \x1b[1;31m[FAIL]\x1b[0m {:<33} : REQUIRED ({})",
                    c.name,
                    c.error.as_deref().unwrap_or("missing")
                );
            } else {
                println!(
                    "  \x1b[1;33m[WARN]\x1b[0m {:<33} : Optional ({})",
                    c.name,
                    c.error.as_deref().unwrap_or("not found")
                );
            }
        }

        println!(
            "\n\x1b[1;34m--------------------------------------------------------------------\x1b[0m"
        );
        println!(
            "Summary: \x1b[1;32m{} passed\x1b[0m, \x1b[1;33m{} warnings\x1b[0m, \x1b[1;31m{} failed\x1b[0m",
            res.passed, res.warnings, res.failed
        );

        if res.failed == 0 {
            println!(
                "\x1b[1;32m[✓] System is fully verified and ready to run Oxide-Tech Agent OS.\x1b[0m"
            );
        } else {
            println!(
                "\x1b[1;31m[✗] Critical requirements are missing. Please inspect failures above.\x1b[0m"
            );
        }
    }
}

fn run_reforge_cli(file_path: PathBuf, arch: String, summary: bool, decompile: bool) -> Result<()> {
    let req = reforge_ipc::ReforgeRequest {
        file_path: file_path.display().to_string(),
        arch: Some(arch),
        summary: Some(summary),
        decompile: Some(decompile),
    };
    let res = reforge_ipc::analyze_file(req)?;

    println!(
        "\x1b[1;36m[+] RE-Forge Analysis: {}\x1b[0m",
        file_path.display()
    );
    println!("  Domain        : {}", res.domain);
    println!("  File Size     : {} bytes", res.file_size);
    println!("  Binary Format : {}", res.format);

    if let Some(entry) = res.entry_point {
        println!("  Entry Point   : {}", entry);
    }
    if let Some(ivt) = res.arm_vector_table {
        println!("  [+] ARM Cortex-M Interrupt Vector Table:");
        println!("      Initial SP      : {}", ivt.initial_sp);
        println!("      Reset Handler   : {}", ivt.reset_handler);
        println!("      HardFault       : {}", ivt.hardfault_handler);
        println!("      SysTick Handler : {}", ivt.systick_handler);
        println!("      Active IRQs     : {}", ivt.external_irqs_count);
    }
    if let Some(rtos) = res.rtos {
        if let Some(name) = rtos.detected_rtos {
            println!(
                "  [+] Inferred Runtime : {} (Confidence: {:.0}%)",
                name,
                rtos.confidence * 100.0
            );
            for sig in rtos.signatures_found {
                println!("      - {}", sig);
            }
        }
    }
    if !res.entropy_chunks.is_empty() {
        println!(
            "  [+] Shannon Entropy  : {:.2} / 8.0 (Avg across {} blocks)",
            res.avg_entropy,
            res.entropy_chunks.len()
        );
    }
    if let Some(ptx) = res.ptx_analysis {
        println!("  Target Architecture : {}", ptx.target_arch);
        println!("  Entry Kernel        : {}", ptx.kernel_name);
        println!("  Shared Memory       : {} bytes", ptx.shared_memory_bytes);
        println!(
            "  Async Copy (cp.async): {}",
            if ptx.uses_async_copy { "Yes" } else { "No" }
        );
        println!("  Inferred Operation  : {}", ptx.inferred_operation);
        println!("  Tensor Core Patterns: {}", ptx.tensor_core_patterns.len());
    }
    if !res.functions.is_empty() {
        println!("  Disassembled Functions : {}", res.functions.len());
        println!("  Total Instructions Decoded: {}", res.total_instructions);
    }
    if let Some(code) = res.decompiled_code {
        println!(
            "\n\x1b[1;33m[+] Neural Safe-Rust Decompiler Output:\x1b[0m\n{}",
            code
        );
    }

    Ok(())
}

fn run_verify_cli(workspace: PathBuf, export_path: Option<PathBuf>) -> Result<()> {
    println!(
        "\x1b[1;36m[+] Running Deterministic Verifier Suite on {}\x1b[0m",
        workspace.display()
    );

    let req = verifier_ipc::VerifierRequest {
        workspace: workspace.display().to_string(),
        task_id: None,
        export_path: export_path.map(|p| p.display().to_string()),
    };

    let bundle = verifier_ipc::run_verification(req)?;

    for r in &bundle.reports {
        if r.passed {
            println!(
                "  \x1b[1;32m[✓] {}: PASSED ({}ms)\x1b[0m",
                r.stage, r.duration_ms
            );
        } else {
            println!(
                "  \x1b[1;31m[✗] {}: FAILED ({}ms)\x1b[0m",
                r.stage, r.duration_ms
            );
        }
    }

    if let Some(p) = bundle.exported_path {
        println!("\x1b[1;32m[✓] Evidence bundle exported to {}\x1b[0m", p);
    }

    Ok(())
}

fn run_memory_passthrough(args: &[String]) {
    match memory::run_embed_blocking(args, &std::env::current_dir().unwrap_or(".".into())) {
        Ok(output) => {
            use std::io::Write as _;
            let _ = std::io::stdout().write_all(&output.stdout);
            let _ = std::io::stderr().write_all(&output.stderr);
            std::process::exit(output.status.code().unwrap_or(1));
        }
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(127);
        }
    }
}

fn run_status(gateway_url: &str) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    match rt.block_on(gateway_rt::probe_gateway(gateway_url, 3)) {
        Ok(code) => println!("gateway {gateway_url} → ONLINE (HTTP {code})"),
        Err(e) => {
            eprintln!("gateway {gateway_url} → OFFLINE ({e})");
            std::process::exit(1);
        }
    }
}

fn main() {
    let raw: Vec<String> = std::env::args().skip(1).collect();
    if raw.is_empty() {
        run_desktop(None);
        return;
    }
    if matches!(raw[0].as_str(), "-h" | "--help" | "help") {
        println!("{}", usage());
        return;
    }
    if matches!(raw[0].as_str(), "-V" | "--version" | "version") {
        println!("oxide-tech-local-agent {VERSION}");
        return;
    }
    let (cmd, rest) = match raw.split_first() {
        Some((head, _)) if !head.starts_with('-') => (head.as_str(), &raw[1..]),
        _ => ("desktop", raw.as_slice()),
    };

    match cmd {
        "desktop" => run_desktop(flag_value(&raw, "--config")),
        "daemon" => {
            if let Err(e) = gateway_rt::run_headless(flag_value(&raw, "--config").as_deref()) {
                eprintln!("gateway fatal: {e:?}");
                std::process::exit(1);
            }
        }
        "doctor" => {
            let json = raw.iter().any(|a| a == "--json");
            run_doctor_cli(json);
        }
        "re-forge" => {
            if rest.is_empty() {
                eprintln!(
                    "Usage: oxide-tech-local-agent re-forge <FILE> [--arch <arch>] [--summary] [--decompile]"
                );
                std::process::exit(1);
            }
            let file = PathBuf::from(&rest[0]);
            let arch = flag_value(&raw, "--arch").unwrap_or_else(|| "auto".to_string());
            let summary = raw.iter().any(|a| a == "--summary" || a == "-s");
            let decompile = raw.iter().any(|a| a == "--decompile" || a == "-d");
            if let Err(e) = run_reforge_cli(file, arch, summary, decompile) {
                eprintln!("re-forge error: {e:?}");
                std::process::exit(1);
            }
        }
        "verify" => {
            let workspace = flag_value(&raw, "--workspace")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            let export = flag_value(&raw, "--export-evidence").map(PathBuf::from);
            if let Err(e) = run_verify_cli(workspace, export) {
                eprintln!("verify error: {e:?}");
                std::process::exit(1);
            }
        }
        "status" => {
            let url =
                flag_value(&raw, "--gateway-url").unwrap_or_else(|| DEFAULT_GATEWAY_URL.into());
            run_status(&url);
        }
        "memory" | "embed" => run_memory_passthrough(rest),
        "-h" | "--help" | "help" => println!("{}", usage()),
        "-V" | "--version" | "version" => println!("oxide-tech-local-agent {VERSION}"),
        unknown => {
            eprintln!("error: unknown command '{unknown}'\n\n{}", usage());
            std::process::exit(2);
        }
    }
}
