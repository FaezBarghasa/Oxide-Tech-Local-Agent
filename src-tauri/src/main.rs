//! oxide-agent — single production binary.
//!
//! One binary runs everything:
//!   - `oxide-agent` (no args) or `oxide-agent desktop` → Tauri desktop window
//!     with the gateway embedded (background thread) and project memory via
//!     the bundled `oxide-embed` sidecar.
//!   - `oxide-agent daemon [--config PATH]` → headless gateway (systemd unit).
//!   - `oxide-agent doctor`                  → environment diagnostics.
//!   - `oxide-agent memory <args...>`        → `oxide-embed` passthrough.
//!   - `oxide-agent status`                  → probe the local gateway.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gateway_rt;
mod memory;

use std::path::PathBuf;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const DEFAULT_GATEWAY_URL: &str = "http://127.0.0.1:8080";

fn usage() -> String {
    format!(
        "oxide-agent {VERSION} — Oxide-Tech Local Agent OS (single-binary desktop)\n\
         \n\
         Usage:\n  \
           oxide-agent [desktop] [--config PATH]   Launch desktop UI (embedded gateway + memory)\n  \
           oxide-agent daemon [--config PATH]      Run headless gateway (systemd service mode)\n  \
           oxide-agent doctor [--gateway-url URL]  Environment diagnostics\n  \
           oxide-agent status [--gateway-url URL]  Probe local gateway\n  \
           oxide-agent memory <oxide-embed args>   Project memory passthrough (init/index/search/...)\n  \
           oxide-agent --help | --version"
    )
}

/// Extract `--flag value` from a raw arg slice.
fn flag_value(args: &[String], flag: &str) -> Option<String> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].clone())
}

fn run_desktop(config: Option<String>) {
    // Structured logs go to stdout; the WebView renders the UI.
    let _ = tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .try_init();

    gateway_rt::spawn_background(config);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            memory::memory_env,
            memory::memory_status,
            memory::memory_init,
            memory::memory_index,
            memory::memory_search,
            memory::memory_context,
            memory::memory_remember,
            memory::memory_recall,
            memory::memory_explain,
            gateway_status,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Oxide Agent desktop");
}

#[tauri::command]
async fn gateway_status(base_url: Option<String>) -> Result<u16, String> {
    let base = base_url.unwrap_or_else(|| DEFAULT_GATEWAY_URL.to_string());
    gateway_rt::probe_gateway(&base, 3).await
}

fn run_doctor(gateway_url: &str) {
    println!("Oxide Agent doctor — {VERSION}");
    let mut failed = 0;

    match memory::resolve_embed_bin() {
        Ok(bin) => {
            let v = std::process::Command::new(&bin)
                .arg("--version")
                .output()
                .map(|o| {
                    let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
                    if s.is_empty() {
                        String::from_utf8_lossy(&o.stderr).trim().to_string()
                    } else {
                        s
                    }
                })
                .unwrap_or_else(|e| format!("(version probe failed: {e})"));
            println!("[ok] oxide-embed : {} ({v})", bin.display());
        }
        Err(e) => {
            println!("[!!] oxide-embed : {e}");
            failed += 1;
        }
    }

    let manifest = PathBuf::from(".").join(".oxide").join("manifest.json");
    if manifest.is_file() {
        println!("[ok] project memory : .oxide/manifest.json present");
    } else {
        println!(
            "[--] project memory : no .oxide/manifest.json in current dir (run `oxide-agent memory init`)"
        );
    }

    let probe = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map(|rt| rt.block_on(gateway_rt::probe_gateway(gateway_url, 3)));
    match probe {
        Ok(Ok(code)) => println!("[ok] gateway      : {gateway_url} → HTTP {code}"),
        Ok(Err(e)) => {
            println!("[--] gateway      : {e} (start with `oxide-agent desktop` or `daemon`)");
        }
        Err(e) => {
            println!("[!!] gateway      : runtime build failed: {e}");
            failed += 1;
        }
    }

    if failed == 0 {
        println!("doctor: all required checks passed");
    } else {
        println!("doctor: {failed} required check(s) failed");
        std::process::exit(1);
    }
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
    // Handle explicit flags first (--help, --version) before command dispatch
    if raw.iter().any(|a| a == "-h" || a == "--help" || a == "help") {
        println!("{}", usage());
        return;
    }
    if raw.iter().any(|a| a == "-V" || a == "--version" || a == "version") {
        println!("oxide-agent {VERSION}");
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
            let url = flag_value(&raw, "--gateway-url").unwrap_or_else(|| DEFAULT_GATEWAY_URL.into());
            run_doctor(&url);
        }
        "status" => {
            let url = flag_value(&raw, "--gateway-url").unwrap_or_else(|| DEFAULT_GATEWAY_URL.into());
            run_status(&url);
        }
        "memory" => run_memory_passthrough(rest),
        "-h" | "--help" | "help" => println!("{}", usage()),
        "-V" | "--version" | "version" => println!("oxide-agent {VERSION}"),
        unknown => {
            eprintln!("error: unknown command '{unknown}'\n\n{}", usage());
            std::process::exit(2);
        }
    }
}
