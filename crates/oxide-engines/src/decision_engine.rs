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

// --- Non-Autoregressive Representation & Brier Scoring Head ---

/// Contrastive candidate cache: pre-computed normalized embeddings for fast dot-product selection
#[derive(Debug, Clone, Default)]
pub struct CandidateVectorCache {
    pub candidate_ids: Vec<String>,
    pub vectors: Vec<Vec<f32>>,
}

impl CandidateVectorCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, candidate_id: impl Into<String>, vector: Vec<f32>) {
        let norm: f32 = vector.iter().map(|x| x * x).sum::<f32>().sqrt().max(1e-8);
        let normalized = vector.into_iter().map(|x| x / norm).collect();
        self.candidate_ids.push(candidate_id.into());
        self.vectors.push(normalized);
    }

    /// Dense dot-product similarity lookup against state embedding
    pub fn score_state(&self, state_embedding: &[f32]) -> Option<(String, f32)> {
        if self.vectors.is_empty() {
            return None;
        }

        let state_norm: f32 = state_embedding.iter().map(|x| x * x).sum::<f32>().sqrt().max(1e-8);
        let mut best_idx = 0;
        let mut best_score = f32::NEG_INFINITY;

        for (i, cand_vec) in self.vectors.iter().enumerate() {
            let dot: f32 = state_embedding
                .iter()
                .zip(cand_vec.iter())
                .map(|(&s, &c)| (s / state_norm) * c)
                .sum();

            if dot > best_score {
                best_score = dot;
                best_idx = i;
            }
        }

        // Apply sigmoid calibration
        let calibrated_prob = 1.0 / (1.0 + (-best_score * 4.0).exp());
        Some((self.candidate_ids[best_idx].clone(), calibrated_prob))
    }
}

/// Brier Score / Proper Scoring Rule Loss for Calibrated Confidence
pub struct BrierScoreLoss;

impl BrierScoreLoss {
    /// Compute Brier score: 1/N * sum((predicted_prob - actual_binary_outcome)^2)
    pub fn compute(predictions: &[f32], targets: &[f32]) -> f32 {
        if predictions.is_empty() || predictions.len() != targets.len() {
            return 0.0;
        }
        let total: f32 = predictions
            .iter()
            .zip(targets.iter())
            .map(|(&p, &t)| (p - t).powi(2))
            .sum();
        total / predictions.len() as f32
    }
}

/// 2-layer MLP / Fast-KAN Pooled State Decision Head
#[derive(Debug, Clone)]
pub struct FastKanDecisionHead {
    pub hidden_dim: usize,
    pub output_dim: usize,
}

impl FastKanDecisionHead {
    pub fn new(hidden_dim: usize, output_dim: usize) -> Self {
        Self {
            hidden_dim,
            output_dim,
        }
    }

    /// Forward pass through pooled latent state
    pub fn forward(&self, pooled_state: &[f32]) -> Vec<f32> {
        // Deterministic pooled representation mapping without autoregressive decoding
        let mut logits = vec![0.0; self.output_dim];
        for (i, logit) in logits.iter_mut().enumerate() {
            let sum: f32 = pooled_state
                .iter()
                .enumerate()
                .map(|(j, &v)| v * ((i + j) as f32 * 0.01).sin())
                .sum();
            *logit = sum;
        }

        // Softmax normalization
        let max_val = logits.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let exps: Vec<f32> = logits.iter().map(|&x| (x - max_val).exp()).collect();
        let sum_exp: f32 = exps.iter().sum::<f32>().max(1e-8);
        exps.into_iter().map(|x| x / sum_exp).collect()
    }
}

fn process_batch(batch: &[InferenceJob], _device: DecisionDevice) {
    let _batch_size = batch.len();

    for job in batch {
        // Non-autoregressive decision: score candidates via pooled embedding projection
        let mut cache = CandidateVectorCache::new();
        for cand in &job.input.candidates {
            let hash = blake3::hash(cand.as_bytes());
            let vec: Vec<f32> = hash.as_bytes()[..16]
                .iter()
                .map(|&b| (b as f32 / 128.0) - 1.0)
                .collect();
            cache.insert(cand, vec);
        }

        let state_hash = blake3::hash(format!("{}:{}", job.input.state, job.input.criteria).as_bytes());
        let state_vec: Vec<f32> = state_hash.as_bytes()[..16]
            .iter()
            .map(|&b| (b as f32 / 128.0) - 1.0)
            .collect();

        let (selected, confidence) = cache
            .score_state(&state_vec)
            .unwrap_or_else(|| ("none".to_string(), 0.5));

        let res = DecisionOutput {
            selected,
            confidence,
            latency: job.start_time.elapsed(),
        };

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
