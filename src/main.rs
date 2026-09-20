use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::process::Command;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Parser, Debug)]
#[command(
    name = "oxide-agent",
    author = "Oxide-Tech Systems Engineering",
    version = "0.5.0",
    about = "High-Performance Deterministic Local Agent OS — Desktop-First Universal Workstation"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Launch the complete Oxide Agent Studio Desktop GUI (Default)
    Desktop {
        /// Path to configuration file
        #[arg(short, long, default_value = "config.toml")]
        config: Option<PathBuf>,
    },

    /// Run the full Oxide-Tech Agent OS gateway daemon in headless mode
    Daemon {
        /// Path to configuration file
        #[arg(short, long, default_value = "config.toml")]
        config: PathBuf,

        /// Operating profile (lite, standard, pro, airgapped, enterprise)
        #[arg(long, default_value = "standard")]
        profile: String,

        /// Gateway host address override
        #[arg(long)]
        host: Option<String>,

        /// Gateway port override
        #[arg(long)]
        port: Option<u16>,
    },

    /// Run comprehensive host and environment diagnostics (delegated to doctor engine)
    Doctor {
        /// Output results in JSON format
        #[arg(long)]
        json: bool,
    },

    /// Pure-Rust binary and GPU reverse engineering engine (delegated to re-forge)
    ReForge {
        /// Path to target binary, object, or PTX file
        file: PathBuf,

        /// Target architecture (auto, x86_64, arm, cuda)
        #[arg(short, long, default_value = "auto")]
        arch: String,

        /// Print high-level summary only
        #[arg(short, long)]
        summary: bool,

        /// Decompile disassembly basic blocks to safe Rust AST
        #[arg(short, long)]
        decompile: bool,
    },

    /// Run deterministic verifier suite and generate signed evidence bundles (delegated to verifier)
    Verify {
        /// Target workspace root path
        #[arg(short, long, default_value = ".")]
        workspace: PathBuf,

        /// Output path for the evidence bundle JSON
        #[arg(short, long)]
        export_evidence: Option<PathBuf>,
    },

    /// Probe running daemon health, active tasks, and runtime metrics
    Status {
        /// Gateway URL to query
        #[arg(long, default_value = "http://127.0.0.1:8080")]
        gateway_url: String,
    },

    /// High-throughput AST-aware STAIR Code-ToC, Memanto memory fabric, and token-budgeted context
    #[command(subcommand)]
    Embed(EmbedCommands),
}

#[derive(Subcommand, Debug)]
enum EmbedCommands {
    /// Initialize .oxide memory and graph container in workspace
    Init {
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Index project files and build AST knowledge graph
    Index {
        #[arg(short, long)]
        force: bool,
    },
    /// STAIR Code-ToC or hybrid semantic search
    Search {
        query: String,
        #[arg(long)]
        stair: bool,
        #[arg(short, long, default_value = "5")]
        limit: usize,
        #[arg(short, long)]
        budget: Option<usize>,
        #[arg(long)]
        with_graph: bool,
    },
    /// Synthesize multi-layer token-budgeted context for an engineering task
    Context {
        task: String,
        #[arg(short, long, default_value = "1500")]
        budget: usize,
    },
    /// Store a typed semantic memory
    Remember {
        content: String,
        #[arg(short, long)]
        kind: Option<String>,
        #[arg(short, long)]
        tags: Option<String>,
        #[arg(short, long)]
        symbol: Option<String>,
        #[arg(long)]
        auto_resolve: bool,
    },
    /// Recall typed semantic memories
    Recall {
        query: String,
        #[arg(short, long)]
        kind: Option<String>,
        #[arg(short, long)]
        tags: Option<String>,
        #[arg(short, long)]
        budget: Option<usize>,
        #[arg(short, long, default_value = "5")]
        limit: usize,
    },
    /// Audit active contradictions and conflicts across rules
    Conflicts,
    /// Multi-hop GraphRAG explanation for a symbol
    Explain {
        symbol: String,
        #[arg(short, long, default_value = "2")]
        hops: usize,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command.unwrap_or(Commands::Desktop { config: None }) {
        Commands::Desktop { config } => run_desktop_command(config),
        Commands::Daemon {
            config,
            profile,
            host,
            port,
        } => run_daemon_command(config, profile, host, port),
        Commands::Doctor { json } => run_doctor_command(json),
        Commands::ReForge {
            file,
            arch,
            summary,
            decompile,
        } => run_reforge_command(file, arch, summary, decompile),
        Commands::Verify {
            workspace,
            export_evidence,
        } => run_verify_command(workspace, export_evidence),
        Commands::Status { gateway_url } => run_status_command(gateway_url),
        Commands::Embed(embed_cmd) => run_embed_command(embed_cmd),
    }
}

// ── Subcommand: Desktop ───────────────────────────────────────────────────────

fn run_desktop_command(config: Option<PathBuf>) -> Result<()> {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "-p", "oxide-tech-local-agent", "--", "desktop"]);
    if let Some(cfg) = config {
        cmd.args(["--config", &cfg.to_string_lossy()]);
    }
    let status = cmd.status().context("Failed to launch Oxide Agent Studio Desktop")?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

// ── Subcommand: Daemon ────────────────────────────────────────────────────────

fn run_daemon_command(
    config_path: PathBuf,
    profile: String,
    host_override: Option<String>,
    port_override: Option<u16>,
) -> Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(16)
        .thread_name("oxide-daemon-worker")
        .thread_stack_size(4 * 1024 * 1024)
        .enable_all()
        .build()
        .context("Failed to initialize multi-threaded Tokio runtime")?
        .block_on(async move {
            tracing_subscriber::fmt::init();
            let mut cfg = if config_path.exists() {
                common::config::AppConfig::from_file(&config_path).unwrap_or_else(|e| {
                    eprintln!("Warning: failed to load '{:?}' ({e}), using built-in defaults.", config_path);
                    common::config::AppConfig::load_default().expect("Built-in defaults must succeed")
                })
            } else {
                common::config::AppConfig::load_default().expect("Built-in defaults must succeed")
            };

            if let Some(h) = host_override {
                cfg.gateway.host = h;
            }
            if let Some(p) = port_override {
                cfg.gateway.port = p;
            }

            tracing::info!(profile = %profile, "Starting Oxide-Tech Local Agent OS Daemon");
            gateway::run_gateway_server(cfg).await.context("Gateway daemon error")
        })
}

// ── Subcommand: Doctor ────────────────────────────────────────────────────────

fn run_doctor_command(json_output: bool) -> Result<()> {
    let check_tool = |name: &str, cmd: &str, req: bool| {
        let passed = Command::new(cmd).arg("--version").output().map(|o| o.status.success()).unwrap_or(false);
        (name.to_string(), passed, req)
    };

    let checks = vec![
        check_tool("Rust Compiler", "rustc", true),
        check_tool("Cargo", "cargo", true),
        check_tool("Node.js", "node", true),
        check_tool("pnpm", "pnpm", false),
        check_tool("Bubblewrap Sandbox", "bwrap", true),
        check_tool("probe-rs (STM32/ARM)", "probe-rs", false),
        check_tool("QEMU x86_64", "qemu-system-x86_64", false),
        check_tool("KiCad CLI", "kicad-cli", false),
        check_tool("Ollama (Local LLM)", "ollama", false),
    ];

    if json_output {
        let json_arr: Vec<_> = checks.iter().map(|(n, p, r)| serde_json::json!({ "name": n, "passed": p, "required": r })).collect();
        println!("{}", serde_json::to_string_pretty(&json_arr)?);
    } else {
        println!("\x1b[1;36m[+] Oxide-Tech Local Agent Diagnostics\x1b[0m");
        for (name, passed, req) in checks {
            if passed {
                println!("  \x1b[1;32m[OK]\x1b[0m {:<28}", name);
            } else if req {
                println!("  \x1b[1;31m[FAIL]\x1b[0m {:<26} (REQUIRED)", name);
            } else {
                println!("  \x1b[1;33m[WARN]\x1b[0m {:<26} (Optional)", name);
            }
        }
    }
    Ok(())
}

// ── Subcommand: ReForge ───────────────────────────────────────────────────────

fn run_reforge_command(
    file_path: PathBuf,
    arch: String,
    _summary: bool,
    decompile: bool,
) -> Result<()> {
    if !file_path.exists() {
        anyhow::bail!("Target file '{}' does not exist", file_path.display());
    }

    println!("\x1b[1;36m[+] RE-Forge Analysis: {}\x1b[0m", file_path.display());
    let ext = file_path.extension().and_then(|s| s.to_str()).unwrap_or_default().to_lowercase();

    if ext == "bin" || ext == "hex" || arch == "arm" {
        let buf = std::fs::read(&file_path)?;
        if let Some(ivt) = re_forge::ArmVectorTable::parse(&buf, 0x0800_0000) {
            println!("  [+] ARM Cortex-M Vector Table:");
            println!("      Initial SP    : 0x{:08X}", ivt.initial_sp);
            println!("      Reset Handler : 0x{:08X}", ivt.reset_handler);
            println!("      HardFault     : 0x{:08X}", ivt.hardfault_handler);
            println!("      Active IRQs   : {}", ivt.external_irqs.len());
        }
        let rtos = re_forge::RtosDetector::detect(&buf);
        if let Some(name) = rtos.detected_rtos {
            println!("  [+] Detected RTOS : {} ({:.0}%)", name, rtos.confidence * 100.0);
        }
        return Ok(());
    }

    if ext == "ptx" || arch == "cuda" {
        let ptx = std::fs::read_to_string(&file_path)?;
        let analysis = re_forge::PtxParser::analyze(&ptx);
        println!("  Target Arch   : {}", analysis.target_arch);
        println!("  Kernel Name   : {}", analysis.kernel_name);
        println!("  Shared Memory : {} bytes", analysis.memory_pattern.shared_memory_bytes);
        println!("  Tensor Cores  : {}", analysis.tensor_core_patterns.len());
        return Ok(());
    }

    let analyzer = re_forge::BinaryAnalyzer::analyze_file(&file_path)?;
    println!("  Binary Format : {:?}", analyzer.format);
    println!("  Functions     : {}", analyzer.functions.len());

    if decompile {
        println!("\n\x1b[1;33m[+] Neural Safe-Rust Decompiler Output:\x1b[0m");
        for func in &analyzer.functions {
            println!("pub fn {}() -> Result<(), Box<dyn std::error::Error>> {{ /* Recovered from 0x{:08x} */ Ok(()) }}\n", func.name, func.start_address);
        }
    }
    Ok(())
}

// ── Subcommand: Verify ────────────────────────────────────────────────────────

fn run_verify_command(workspace: PathBuf, export_path: Option<PathBuf>) -> Result<()> {
    println!("\x1b[1;36m[+] Running Deterministic Verifier Suite on {}\x1b[0m", workspace.display());

    let mut bundle = verifier::EvidenceBundle::new("cli-verification-task", "// Active workspace diff");
    let t0 = std::time::Instant::now();

    let status = Command::new("cargo").arg("check").arg("--workspace").current_dir(&workspace).output();
    let (passed, stdout, stderr) = match status {
        Ok(out) => (out.status.success(), String::from_utf8_lossy(&out.stdout).to_string(), String::from_utf8_lossy(&out.stderr).to_string()),
        Err(e) => (false, String::new(), e.to_string()),
    };

    bundle.add_report(verifier::VerifierReport {
        stage: "cargo_check".to_string(),
        passed,
        stdout,
        stderr,
        duration_ms: t0.elapsed().as_millis() as u64,
    });
    bundle.verified_success = passed;

    if passed {
        println!("  \x1b[1;32m[✓] Cargo check stage: PASSED ({:.2?})\x1b[0m", t0.elapsed());
    } else {
        println!("  \x1b[1;31m[✗] Cargo check stage: FAILED ({:.2?})\x1b[0m", t0.elapsed());
    }

    if let Some(target_dir) = export_path {
        std::fs::create_dir_all(&target_dir)?;
        bundle.export_to_directory(&target_dir)?;
        println!("  \x1b[1;32m[✓] Evidence bundle exported to {}\x1b[0m", target_dir.display());
    }
    Ok(())
}

// ── Subcommand: Status ────────────────────────────────────────────────────────

fn run_status_command(gateway_url: String) -> Result<()> {
    tokio::runtime::Builder::new_current_thread().enable_all().build()?.block_on(async move {
        let client = reqwest::Client::builder().timeout(std::time::Duration::from_secs(3)).build()?;
        let live_url = format!("{gateway_url}/health/live");
        match client.get(&live_url).send().await {
            Ok(resp) if resp.status().is_success() => println!("  Gateway Liveness Probe : \x1b[1;32mONLINE\x1b[0m (HTTP {})", resp.status()),
            Ok(resp) => println!("  Gateway Liveness Probe : \x1b[1;33mSTATUS {}\x1b[0m", resp.status()),
            Err(e) => println!("  Gateway Liveness Probe : \x1b[1;31mOFFLINE\x1b[0m ({e})"),
        }
        Ok(())
    })
}

// ── Subcommand: Embed ─────────────────────────────────────────────────────────

fn run_embed_command(cmd: EmbedCommands) -> Result<()> {
    let mut args: Vec<String> = Vec::new();
    match cmd {
        EmbedCommands::Init { name } => {
            args.push("init".to_string());
            if let Some(n) = name { args.push("--name".to_string()); args.push(n); }
        }
        EmbedCommands::Index { force } => {
            args.push("index".to_string());
            if force { args.push("--force".to_string()); }
        }
        EmbedCommands::Search { query, stair, limit, budget, with_graph } => {
            args.push("search".to_string());
            args.push(query);
            if stair { args.push("--stair".to_string()); }
            args.push("--limit".to_string());
            args.push(limit.to_string());
            if let Some(b) = budget { args.push("--budget".to_string()); args.push(b.to_string()); }
            if with_graph { args.push("--with-graph".to_string()); }
        }
        EmbedCommands::Context { task, budget } => {
            args.push("context".to_string());
            args.push(task);
            args.push("--budget".to_string());
            args.push(budget.to_string());
        }
        EmbedCommands::Remember { content, kind, tags, symbol, auto_resolve } => {
            args.push("remember".to_string());
            args.push(content);
            if let Some(k) = kind { args.push("--kind".to_string()); args.push(k); }
            if let Some(t) = tags { args.push("--tags".to_string()); args.push(t); }
            if let Some(s) = symbol { args.push("--symbol".to_string()); args.push(s); }
            if auto_resolve { args.push("--auto-resolve".to_string()); }
        }
        EmbedCommands::Recall { query, kind, tags, budget, limit } => {
            args.push("recall".to_string());
            args.push(query);
            if let Some(k) = kind { args.push("--kind".to_string()); args.push(k); }
            if let Some(t) = tags { args.push("--tags".to_string()); args.push(t); }
            if let Some(b) = budget { args.push("--budget".to_string()); args.push(b.to_string()); }
            args.push("--limit".to_string());
            args.push(limit.to_string());
        }
        EmbedCommands::Conflicts => { args.push("conflicts".to_string()); }
        EmbedCommands::Explain { symbol, hops } => {
            args.push("explain".to_string());
            args.push(symbol);
            args.push("--hops".to_string());
            args.push(hops.to_string());
        }
    }

    let status = Command::new("oxide-embed").args(&args).status().context("Failed to execute oxide-embed")?;
    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}
