use std::sync::mpsc::{channel, Sender as SyncSender};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DecisionInput {
    pub state: String,
    pub criteria: String,
    pub candidates: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DecisionOutput {
    pub selected: String,
    pub confidence: f32,
    pub latency: Duration,
}

/// Internal job passing the task and a one-time response channel
struct InferenceJob {
    input: DecisionInput,
    start_time: Instant,
    tx: SyncSender<Result<DecisionOutput, String>>,
}

/// Execution device identifier for local backend compute
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionDevice {
    Cpu,
    Cuda(usize),
}

/// Thread-safe client handle. Can be cloned across OS threads or Rayon tasks.
#[derive(Clone)]
pub struct DecisionEngine {
    tx: flume::Sender<InferenceJob>,
}

impl DecisionEngine {
    pub fn new(device: DecisionDevice, max_batch_size: usize, timeout_ms: u64) -> Self {
        let (tx, rx) = flume::unbounded::<InferenceJob>();

        // Dedicated OS thread for compute.
        // Isolates inference runtime and avoids GPU/CPU context contention.
        thread::Builder::new()
            .name("decision-worker".into())
            .spawn(move || {
                run_worker_loop(rx, device, max_batch_size, Duration::from_millis(timeout_ms));
            })
            .expect("Failed to spawn inference worker thread");

        Self { tx }
    }

    /// Synchronous, blocking call. Thread-safe and re-entrant.
    pub fn decide(&self, input: DecisionInput) -> Result<DecisionOutput, String> {
        let (resp_tx, resp_rx) = channel();

        self.tx
            .send(InferenceJob {
                input,
                start_time: Instant::now(),
                tx: resp_tx,
            })
            .map_err(|e| format!("Worker thread unreachable: {e}"))?;

        resp_rx
            .recv()
            .map_err(|e| format!("Failed to receive response: {e}"))?
    }
}

// --- Dedicated Worker Loop with Dynamic Micro-Batching ---

fn run_worker_loop(
    rx: flume::Receiver<InferenceJob>,
    device: DecisionDevice,
    max_batch_size: usize,
    max_wait: Duration,
) {
    loop {
        let mut batch = Vec::new();

        // Step 1: Wait for at least 1 incoming item (blocks without burning CPU)
        let first_job = match rx.recv() {
            Ok(job) => job,
            Err(_) => break, // All senders dropped, exit thread cleanly
        };
        batch.push(first_job);

        // Step 2: Dynamic micro-batching window
        // Opportunistically drain remaining queued tasks up to max_batch_size
        let deadline = Instant::now() + max_wait;
        while batch.len() < max_batch_size {
            let now = Instant::now();
            if now >= deadline {
                break;
            }

            match rx.recv_timeout(deadline - now) {
                Ok(job) => batch.push(job),
                Err(flume::RecvTimeoutError::Timeout) => break,
                Err(flume::RecvTimeoutError::Disconnected) => break,
            }
        }

        // Step 3: Run the forward pass on the collected micro-batch
        process_batch(&batch, device);
    }
}

fn process_batch(batch: &[InferenceJob], _device: DecisionDevice) {
    let _batch_size = batch.len();

    // Mock computation & logit output resolution:
    for job in batch {
        let selected = job
            .input
            .candidates
            .first()
            .cloned()
            .unwrap_or_else(|| "none".to_string());

        let res = DecisionOutput {
            selected,
            confidence: 0.985,
            latency: job.start_time.elapsed(),
        };

        // Send back to caller thread
        let _ = job.tx.send(Ok(res));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decision_engine_concurrent_batching() {
        let engine = DecisionEngine::new(DecisionDevice::Cpu, 16, 5);
        let mut handles = Vec::new();

        for i in 0..8 {
            let engine_clone = engine.clone();
            let handle = thread::spawn(move || {
                let input = DecisionInput {
                    state: format!("state_{i}"),
                    criteria: "urgency".into(),
                    candidates: vec!["critical".into(), "normal".into()],
                };
                let output = engine_clone.decide(input).expect("Decision should succeed");
                assert_eq!(output.selected, "critical");
                assert!(output.confidence > 0.9);
            });
            handles.push(handle);
        }

        for h in handles {
            h.join().expect("Worker thread should join cleanly");
        }
    }
}
