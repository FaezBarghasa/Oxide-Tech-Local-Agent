use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HfHubError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Model config not found in: {0}")]
    ConfigNotFound(PathBuf),
    #[error("SafeTensors index corrupted or missing: {0}")]
    InvalidSafeTensorsIndex(String),
    #[error("Architecture '{0}' is not supported")]
    UnsupportedArchitecture(String),
}

/// Normalized AutoModel Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    #[serde(default = "default_arch")]
    pub architectures: Vec<String>,
    #[serde(alias = "hidden_size")]
    pub dim: usize,
    #[serde(alias = "num_attention_heads")]
    pub n_heads: usize,
    #[serde(alias = "num_key_value_heads")]
    pub n_kv_heads: Option<usize>,
    #[serde(alias = "num_hidden_layers")]
    pub n_layers: usize,
    #[serde(alias = "intermediate_size")]
    pub intermediate_dim: usize,
    #[serde(alias = "vocab_size")]
    pub vocab_size: usize,
    #[serde(alias = "rms_norm_eps")]
    pub norm_eps: Option<f32>,
    #[serde(alias = "rope_theta")]
    pub rope_theta: Option<f32>,
    #[serde(alias = "max_position_embeddings")]
    pub max_seq_len: Option<usize>,
}

fn default_arch() -> Vec<String> {
    vec!["LlamaForCausalLM".to_string()]
}

/// Sharded SafeTensors Index (`model.safetensors.index.json`)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafeTensorsIndex {
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub weight_map: HashMap<String, String>,
}

/// Hugging Face AutoModel Loader for Causal LM architectures
pub struct AutoModelForCausalLM;

impl AutoModelForCausalLM {
    /// Load model architecture configuration from a model directory
    pub fn from_pretrained_config(model_dir: &Path) -> Result<ModelConfig, HfHubError> {
        let config_path = model_dir.join("config.json");
        if !config_path.exists() {
            return Err(HfHubError::ConfigNotFound(config_path));
        }

        let content = std::fs::read_to_string(&config_path)?;
        let config: ModelConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Load the sharded weight index or single safetensors file map
    pub fn load_weight_map(model_dir: &Path) -> Result<HashMap<String, PathBuf>, HfHubError> {
        let index_path = model_dir.join("model.safetensors.index.json");
        let single_file = model_dir.join("model.safetensors");

        let mut map = HashMap::new();

        if index_path.exists() {
            let content = std::fs::read_to_string(&index_path)?;
            let index: SafeTensorsIndex = serde_json::from_str(&content)?;
            for (tensor_name, shard_file) in index.weight_map {
                map.insert(tensor_name, model_dir.join(shard_file));
            }
        } else if single_file.exists() {
            map.insert("*".to_string(), single_file);
        } else {
            // Scan for any .safetensors shards
            if let Ok(entries) = std::fs::read_dir(model_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.extension().and_then(|s| s.to_str()) == Some("safetensors")
                        && let Some(name) = path.file_name().and_then(|s| s.to_str())
                    {
                        map.insert(name.to_string(), path);
                    }
                }
            }
        }

        if map.is_empty() {
            return Err(HfHubError::InvalidSafeTensorsIndex(
                "No .safetensors files or index found in model directory".to_string(),
            ));
        }

        Ok(map)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_config_parsing_llama_and_qwen() {
        let temp = tempdir().unwrap();
        let config_json = r#"{
            "architectures": ["Qwen2ForCausalLM"],
            "hidden_size": 3584,
            "intermediate_size": 18944,
            "num_attention_heads": 28,
            "num_hidden_layers": 28,
            "num_key_value_heads": 4,
            "vocab_size": 152064,
            "rope_theta": 1000000.0
        }"#;

        std::fs::write(temp.path().join("config.json"), config_json).unwrap();

        let config = AutoModelForCausalLM::from_pretrained_config(temp.path()).unwrap();
        assert_eq!(config.architectures[0], "Qwen2ForCausalLM");
        assert_eq!(config.dim, 3584);
        assert_eq!(config.n_heads, 28);
        assert_eq!(config.n_kv_heads, Some(4));
        assert_eq!(config.n_layers, 28);
        assert_eq!(config.intermediate_dim, 18944);
        assert_eq!(config.vocab_size, 152064);
        assert_eq!(config.rope_theta, Some(1000000.0));
    }

    #[test]
    fn test_safetensors_index_loading() {
        let temp = tempdir().unwrap();
        let index_json = r#"{
            "metadata": {"total_size": 1000000},
            "weight_map": {
                "model.embed_tokens.weight": "model-00001-of-00002.safetensors",
                "model.layers.0.self_attn.q_proj.weight": "model-00001-of-00002.safetensors",
                "model.layers.27.mlp.down_proj.weight": "model-00002-of-00002.safetensors"
            }
        }"#;

        std::fs::write(temp.path().join("model.safetensors.index.json"), index_json).unwrap();

        let map = AutoModelForCausalLM::load_weight_map(temp.path()).unwrap();
        assert_eq!(map.len(), 3);
        assert!(map.contains_key("model.embed_tokens.weight"));
        assert!(map.contains_key("model.layers.27.mlp.down_proj.weight"));
    }
}
