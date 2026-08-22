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
