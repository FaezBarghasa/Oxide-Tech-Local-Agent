use serde::{Deserialize, Serialize};
use std::thread;

/// Hardware-aware runtime configuration for Tokio execution engines.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeTopology {
    /// Number of worker threads allocated to the async runtime.
    pub worker_threads: usize,
    /// Stack size per worker thread in bytes (e.g. 4MB for deep AST recursion).
    pub stack_size_bytes: usize,
    /// Thread naming prefix for kernel debugging and perf tools.
    pub thread_prefix: String,
    /// Whether to attempt CPU core affinity pinning.
    pub enable_core_pinning: bool,
}

impl Default for RuntimeTopology {
    fn default() -> Self {
        let physical_cores = thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4);

        Self {
            worker_threads: physical_cores,
            stack_size_bytes: 4 * 1024 * 1024, // 4MB
            thread_prefix: "oxide-worker".to_string(),
            enable_core_pinning: true,
        }
    }
}

impl RuntimeTopology {
    /// Create a high-performance topology tailored for compute-heavy local inference.
    pub fn for_inference(worker_count: Option<usize>) -> Self {
        let mut topo = Self::default();
        if let Some(w) = worker_count {
            topo.worker_threads = w;
        }
        topo.thread_prefix = "oxide-infer".to_string();
        topo
    }

    /// Pin the calling thread to a specific CPU core ID on Linux.
    #[cfg(target_os = "linux")]
    pub fn pin_current_thread_to_core(core_id: usize) -> bool {
        unsafe {
            let mut cpuset: libc::cpu_set_t = std::mem::zeroed();
            libc::CPU_ZERO(&mut cpuset);
            libc::CPU_SET(core_id, &mut cpuset);
            let pid = libc::gettid();
            let res = libc::sched_setaffinity(
                pid,
                std::mem::size_of::<libc::cpu_set_t>(),
                &cpuset,
            );
            res == 0
        }
    }

    /// Fallback for non-Linux OS.
    #[cfg(not(target_os = "linux"))]
    pub fn pin_current_thread_to_core(_core_id: usize) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topology_default() {
        let topo = RuntimeTopology::default();
        assert!(topo.worker_threads >= 1);
        assert_eq!(topo.stack_size_bytes, 4 * 1024 * 1024);
        assert_eq!(topo.thread_prefix, "oxide-worker");
    }
}
