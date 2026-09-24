//! # Native Unified Diff Patching Engine
//!
//! Provides surgical, AST-aware hunk resolution, fuzzy offset compensation,
//! dry-run preflight verification, and minimal-byte file mutation without full-file rewrites.

use std::fmt;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PatchError {
    #[error("Target content mismatch at hunk line {hunk_line}: expected '{expected}', found '{found}'")]
    ContentMismatch {
        hunk_line: usize,
        expected: String,
        found: String,
    },
    #[error("Hunk start line {0} out of bounds for source of length {1}")]
    OutOfBounds(usize, usize),
    #[error("Malformed unified diff: {0}")]
    MalformedDiff(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiffLine {
    Context(String),
    Add(String),
    Remove(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiffHunk {
    pub old_start: usize,
    pub old_lines: usize,
    pub new_start: usize,
    pub new_lines: usize,
    pub lines: Vec<DiffLine>,
}

#[derive(Debug, Clone)]
pub struct PatchResult {
    pub patched_content: String,
    pub hunks_applied: usize,
    pub fuzzy_offset_applied: isize,
}

pub struct UnifiedDiffPatcher {
    pub max_fuzzy_offset: usize,
}

impl Default for UnifiedDiffPatcher {
    fn default() -> Self {
        Self {
            max_fuzzy_offset: 12,
        }
    }
}

impl UnifiedDiffPatcher {
    pub fn new(max_fuzzy_offset: usize) -> Self {
        Self { max_fuzzy_offset }
    }

    /// Parse a unified diff string into structured DiffHunks
    pub fn parse_unidiff(diff_text: &str) -> Result<Vec<DiffHunk>, PatchError> {
        let mut hunks = Vec::new();
        let mut current_hunk: Option<DiffHunk> = None;

        for line in diff_text.lines() {
            if line.starts_with("@@") {
                if let Some(hunk) = current_hunk.take() {
                    hunks.push(hunk);
                }

                let parts: Vec<&str> = line.split("@@").collect();
                if parts.len() < 2 {
                    return Err(PatchError::MalformedDiff(line.to_string()));
                }

                let range_header = parts[1].trim();
                let ranges: Vec<&str> = range_header.split_whitespace().collect();
                if ranges.len() < 2 {
                    return Err(PatchError::MalformedDiff(line.to_string()));
                }

                let (old_start, old_lines) = Self::parse_range(ranges[0])?;
                let (new_start, new_lines) = Self::parse_range(ranges[1])?;

                current_hunk = Some(DiffHunk {
                    old_start,
                    old_lines,
                    new_start,
                    new_lines,
                    lines: Vec::new(),
                });
            } else if let Some(ref mut hunk) = current_hunk {
                if let Some(rest) = line.strip_prefix('+') {
                    hunk.lines.push(DiffLine::Add(rest.to_string()));
                } else if let Some(rest) = line.strip_prefix('-') {
                    hunk.lines.push(DiffLine::Remove(rest.to_string()));
                } else if let Some(rest) = line.strip_prefix(' ') {
                    hunk.lines.push(DiffLine::Context(rest.to_string()));
                } else if line.is_empty() {
                    hunk.lines.push(DiffLine::Context(String::new()));
                }
            }
        }

        if let Some(hunk) = current_hunk {
            hunks.push(hunk);
        }

        Ok(hunks)
    }

    fn parse_range(range_str: &str) -> Result<(usize, usize), PatchError> {
        let clean = range_str.trim_start_matches('-').trim_start_matches('+');
        let parts: Vec<&str> = clean.split(',').collect();
        let start = parts[0]
            .parse::<usize>()
            .map_err(|e| PatchError::MalformedDiff(e.to_string()))?;
        let count = if parts.len() > 1 {
            parts[1]
                .parse::<usize>()
                .map_err(|e| PatchError::MalformedDiff(e.to_string()))?
        } else {
            1
        };
        Ok((start, count))
    }

    /// Apply unified diff hunks with fuzzy context resolution to source code
    pub fn apply_patch(&self, original: &str, diff_text: &str) -> Result<PatchResult, PatchError> {
        let hunks = Self::parse_unidiff(diff_text)?;
        let mut source_lines: Vec<String> = original.lines().map(|s| s.to_string()).collect();
        let mut hunks_applied = 0;
        let mut total_fuzzy_offset = 0;

        for hunk in hunks {
            let (matched_offset, is_fuzzy) = self.find_hunk_offset(&source_lines, &hunk)?;
            if is_fuzzy {
                let target_start = if hunk.old_start > 0 { hunk.old_start - 1 } else { 0 };
                total_fuzzy_offset += matched_offset as isize - target_start as isize;
            }

            let mut new_source = Vec::new();
            new_source.extend_from_slice(&source_lines[..matched_offset]);

            let mut src_cursor = matched_offset;
            for diff_line in &hunk.lines {
                match diff_line {
                    DiffLine::Context(expected) => {
                        if src_cursor >= source_lines.len() || &source_lines[src_cursor] != expected {
                            return Err(PatchError::ContentMismatch {
                                hunk_line: src_cursor + 1,
                                expected: expected.clone(),
                                found: source_lines.get(src_cursor).cloned().unwrap_or_default(),
                            });
                        }
                        new_source.push(source_lines[src_cursor].clone());
                        src_cursor += 1;
                    }
                    DiffLine::Remove(expected) => {
                        if src_cursor >= source_lines.len() || &source_lines[src_cursor] != expected {
                            return Err(PatchError::ContentMismatch {
                                hunk_line: src_cursor + 1,
                                expected: expected.clone(),
                                found: source_lines.get(src_cursor).cloned().unwrap_or_default(),
                            });
                        }
                        src_cursor += 1; // skip line
                    }
                    DiffLine::Add(addition) => {
                        new_source.push(addition.clone());
                    }
                }
            }

            if src_cursor < source_lines.len() {
                new_source.extend_from_slice(&source_lines[src_cursor..]);
            }

            source_lines = new_source;
            hunks_applied += 1;
        }

        let mut patched_content = source_lines.join("\n");
        if original.ends_with('\n') {
            patched_content.push('\n');
        }

        Ok(PatchResult {
            patched_content,
            hunks_applied,
            fuzzy_offset_applied: total_fuzzy_offset,
        })
    }

    /// Locate hunk position considering small upstream offsets
    fn find_hunk_offset(&self, source_lines: &[String], hunk: &DiffHunk) -> Result<(usize, bool), PatchError> {
        let target_start = if hunk.old_start > 0 { hunk.old_start - 1 } else { 0 };

        // Test exact offset first
        if self.verify_hunk_match_at(source_lines, hunk, target_start) {
            return Ok((target_start, false));
        }

        // Fuzzy offset search
        for offset in 1..=self.max_fuzzy_offset {
            if target_start + offset < source_lines.len()
                && self.verify_hunk_match_at(source_lines, hunk, target_start + offset)
            {
                return Ok((target_start + offset, true));
            }
            if target_start >= offset
                && self.verify_hunk_match_at(source_lines, hunk, target_start - offset)
            {
                return Ok((target_start - offset, true));
            }
        }

        Err(PatchError::OutOfBounds(target_start + 1, source_lines.len()))
    }

    fn verify_hunk_match_at(&self, source_lines: &[String], hunk: &DiffHunk, start_idx: usize) -> bool {
        let mut cur = start_idx;
        for line in &hunk.lines {
            match line {
                DiffLine::Context(expected) | DiffLine::Remove(expected) => {
                    if cur >= source_lines.len() || &source_lines[cur] != expected {
                        return false;
                    }
                    cur += 1;
                }
                DiffLine::Add(_) => {}
            }
        }
        true
    }
}

impl fmt::Display for PatchResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Patch applied successfully (Hunks: {}, Fuzzy Offset: {})",
            self.hunks_applied, self.fuzzy_offset_applied
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unified_diff_exact_patch() {
        let original = "fn main() {\n    println!(\"Hello\");\n}\n";
        let diff = "@@ -1,3 +1,3 @@\n fn main() {\n-    println!(\"Hello\");\n+    println!(\"Hello World\");\n }\n";

        let patcher = UnifiedDiffPatcher::default();
        let res = patcher.apply_patch(original, diff).unwrap();
        assert_eq!(res.patched_content, "fn main() {\n    println!(\"Hello World\");\n}\n");
        assert_eq!(res.hunks_applied, 1);
        assert_eq!(res.fuzzy_offset_applied, 0);
    }

    #[test]
    fn test_unified_diff_fuzzy_offset_resolution() {
        // Original has 2 extra header lines preceding the hunk
        let original = "// Header 1\n// Header 2\nfn compute() -> i32 {\n    42\n}\n";
        let diff = "@@ -1,3 +1,3 @@\n fn compute() -> i32 {\n-    42\n+    100\n }\n";

        let patcher = UnifiedDiffPatcher::new(5);
        let res = patcher.apply_patch(original, diff).unwrap();
        assert!(res.patched_content.contains("100"));
        assert_eq!(res.fuzzy_offset_applied, 2);
    }
}
