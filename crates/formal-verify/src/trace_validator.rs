use serde::{Deserialize, Serialize};

/// Extracted reasoning trace and step breakdown
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningTrace {
    pub raw_thinking: String,
    pub step_sequence: Vec<String>,
    pub token_estimate: usize,
    pub has_reflection: bool,
    pub has_backtracking: bool,
}

/// Judgement outcome from TraceValidator (LLM-as-judge)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceJudgement {
    pub is_sound: bool,
    pub logical_coherence_score: f32, // 0.0 to 1.0
    pub identified_hallucinations: Vec<String>,
    pub rationale: String,
}

pub struct TraceValidator;

impl TraceValidator {
    /// Extracts `<think>...</think>` reasoning trace blocks from raw model generation
    pub fn extract_reasoning_trace(text: &str) -> Option<ReasoningTrace> {
        let think_start = text.find("<think>")?;
        let think_end = text.find("</think>").unwrap_or(text.len());
        let raw_thinking = text[think_start + 7..think_end].trim().to_string();

        if raw_thinking.is_empty() {
            return None;
        }

        let token_estimate = raw_thinking.split_whitespace().count() * 4 / 3;
        let p_lower = raw_thinking.to_lowercase();
        let has_reflection = p_lower.contains("wait,")
            || p_lower.contains("alternatively")
            || p_lower.contains("let me recheck");
        let has_backtracking = p_lower.contains("error:")
            || p_lower.contains("this won't work")
            || p_lower.contains("revising");

        let step_sequence = raw_thinking
            .lines()
            .map(|l| l.trim())
            .filter(|l| {
                l.starts_with("1.")
                    || l.starts_with("2.")
                    || l.starts_with("3.")
                    || l.starts_with("Step ")
                    || l.starts_with("- ")
            })
            .map(|s| s.to_string())
            .collect();

        Some(ReasoningTrace {
            raw_thinking,
            step_sequence,
            token_estimate,
            has_reflection,
            has_backtracking,
        })
    }

    /// Evaluates the soundness and coherence of a reasoning trace against domain constraints
    pub fn validate_trace_heuristics(trace: &ReasoningTrace, domain: &str) -> TraceJudgement {
        let mut score = 0.85f32;
        let mut hallucinations = Vec::new();

        if trace.has_reflection {
            score += 0.1;
        }
        if trace.has_backtracking {
            score += 0.05;
        }

        let p_lower = trace.raw_thinking.to_lowercase();

        if domain.contains("no_std")
            && (p_lower.contains("std::") || p_lower.contains("alloc::vec"))
        {
            hallucinations.push(
                "Trace references standard library std / dynamic heap in no_std target".to_string(),
            );
            score -= 0.3;
        }

        if domain.contains("embedded") && p_lower.contains(".unwrap()") {
            hallucinations
                .push("Trace suggests unwrap() in safety-critical embedded path".to_string());
            score -= 0.25;
        }

        let score = score.clamp(0.0, 1.0);
        let is_sound = score >= 0.70 && hallucinations.is_empty();

        TraceJudgement {
            is_sound,
            logical_coherence_score: score,
            identified_hallucinations: hallucinations,
            rationale: format!(
                "Trace evaluation completed. Coherence score: {:.2}, reflection: {}",
                score, trace.has_reflection
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_and_validate_trace() {
        let text = "<think>\n1. Examine SPI register\n2. Wait, the prescaler must be checked against APB1 clock\n3. Revising baud rate\n</think>\nfn init() {}";
        let trace =
            TraceValidator::extract_reasoning_trace(text).expect("Trace should be extracted");
        assert!(trace.has_reflection);
        assert!(trace.has_backtracking);
        assert_eq!(trace.step_sequence.len(), 3);

        let judgement = TraceValidator::validate_trace_heuristics(&trace, "embedded");
        assert!(judgement.is_sound);
        assert!(judgement.logical_coherence_score >= 0.8);
    }
}
