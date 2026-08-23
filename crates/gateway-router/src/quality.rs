use tracing::debug;

/// Parses `cargo check` stderr to count Rust compiler errors and produce a
/// normalised quality score in [0.0, 1.0].
///
/// A score of 1.0 means the output is error-free.
/// A score of 0.0 means `max_errors` or more errors were found.
pub struct QualityGate {
    /// Number of errors that maps to score 0.0 (linearly interpolated below).
    max_errors: usize,
    /// Minimum acceptable score — below this threshold the current backend is
    /// considered "unsatisfying".
    threshold: f32,
}

impl QualityGate {
    /// Create a gate with the given threshold and a `max_errors` cap of 20.
    pub fn new(threshold: f32) -> Self {
        Self {
            max_errors: 20,
            threshold,
        }
    }

    /// Score the output of a `cargo check` / `cargo clippy` run.
    ///
    /// `stderr` — the captured standard error from the Cargo invocation.
    /// Returns `(score, error_count)`.
    pub fn score(&self, stderr: &str) -> (f32, usize) {
        let error_count = count_compiler_errors(stderr);
        let score = if error_count == 0 {
            1.0_f32
        } else {
            let ratio = error_count as f32 / self.max_errors as f32;
            (1.0 - ratio.min(1.0)).max(0.0)
        };

        debug!(
            error_count = error_count,
            score = score,
            "QualityGate scored cargo check output"
        );

        (score, error_count)
    }

    /// Returns `true` if the score meets the configured threshold.
    pub fn is_satisfying(&self, stderr: &str) -> bool {
        let (score, _) = self.score(stderr);
        score >= self.threshold
    }
}

/// Count `error[E...]` and bare `error:` lines in Cargo stderr.
///
/// Cargo emits errors in two forms:
/// - `error[E0308]: ...`  (typed compiler errors)
/// - `error: ...`         (linker / proc-macro errors)
///
/// We count both conservatively to avoid false positives from warnings
/// that start with "warning: unused ...".
fn count_compiler_errors(stderr: &str) -> usize {
    stderr
        .lines()
        .filter(|line| {
            // Match lines that start with "error" at the beginning of the line
            // (possibly preceded by whitespace).
            let trimmed = line.trim_start();
            trimmed.starts_with("error[E") || trimmed.starts_with("error: ")
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_output_scores_one() {
        let gate = QualityGate::new(0.85);
        let (score, count) = gate.score("   Compiling my_crate v0.1.0\n    Finished dev profile");
        assert_eq!(count, 0);
        assert!((score - 1.0).abs() < f32::EPSILON);
        assert!(gate.is_satisfying("Finished dev profile"));
    }

    #[test]
    fn single_error_reduces_score() {
        let gate = QualityGate::new(0.95);
        let stderr = "error[E0308]: mismatched types\nerror: aborting due to previous error";
        let (score, count) = gate.score(stderr);
        assert_eq!(count, 2);
        assert!(score < 1.0);
        assert!(!gate.is_satisfying(stderr));
    }
}
