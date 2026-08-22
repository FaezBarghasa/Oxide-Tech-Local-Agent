use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OkfMetadata {
    pub path: String,
    pub domain: String,
    pub exports: Vec<String>,
    pub dependencies: Vec<String>,
    pub checksum: String,
}

pub struct OkfDocument {
    pub metadata: OkfMetadata,
    pub raw_content: String,
}

impl OkfDocument {
    pub fn parse_okf(input: &str) -> Result<Self, anyhow::Error> {
        if !input.starts_with("---") {
            return Err(anyhow::anyhow!("Invalid OKF format: Missing Metadata Block"));
        }
        let parts: Vec<&str> = input.splitn(3, "---").collect();
        if parts.len() < 3 {
            return Err(anyhow::anyhow!("Invalid OKF formatting boundaries"));
        }
        let metadata: OkfMetadata = serde_yaml::from_str(parts[1])?;
        let raw_content = parts[2].trim().to_string();

        Ok(Self { metadata, raw_content })
    }
}

pub struct StenoCompactor;

impl StenoCompactor {
    /// Strips excessive noise and formats compiler outputs to preserve key diagnostics while saving 60-80% tokens.
    pub fn compact_compiler_log(raw_log: &str) -> String {
        let mut output = Vec::new();
        for line in raw_log.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("error[E")
                || trimmed.starts_with("-->")
                || trimmed.starts_with("warning:")
                || trimmed.contains("DRC Violation")
                || trimmed.contains("FAILED")
                || trimmed.contains("error:")
            {
                output.push(line);
            }
        }
        if output.is_empty() {
            raw_log.to_string()
        } else {
            output.join("\n")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_steno_compression() {
        let raw = "    Checking foo v0.1.0\nerror[E0425]: cannot find value `bar` in this scope\n  --> src/main.rs:10:5\n   |\n10 |     bar();\n   |     ^^^\nwarning: unused variable `x`";
        let compacted = StenoCompactor::compact_compiler_log(raw);
        assert!(compacted.contains("error[E0425]"));
        assert!(compacted.contains("--> src/main.rs:10:5"));
        assert!(!compacted.contains("Checking foo"));
    }
}

