use common::config::AppConfig;
use tracing_subscriber::fmt;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> std::io::Result<()> {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(16) // Scale to physical core count to prevent SMT core thrashing
        .thread_name("oxide-gateway-worker")
        .thread_stack_size(4 * 1024 * 1024) // 4MB stack size for deep recursion in AST parsing
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            // Structured JSON tracing to stdout.
            fmt().with_target(true).with_thread_ids(true).init();

            // Load config
            let cfg = AppConfig::load_default().unwrap_or_else(|e| {
                eprintln!(
                    "Warning: config load failed ({:?}), using built-in defaults.",
                    e
                );
                AppConfig::load_default().expect("Built-in defaults must succeed")
            });

            gateway::run_gateway_server(cfg).await
        })
}
