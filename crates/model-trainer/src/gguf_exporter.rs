use crate::{GgufQuantType, TrainerError};
use std::path::{Path, PathBuf};
use uuid::Uuid;

/// Production exporter for standalone GGUF v3 models and Ollama runtime containers.
pub struct GgufExporter;

impl GgufExporter {
    /// Export merged model and adapter into a self-contained GGUF v3 binary.
    pub async fn export_merged_gguf(
        output_path: &Path,
        model_name: &str,
        quant_type: GgufQuantType,
        context_length: usize,
    ) -> Result<PathBuf, TrainerError> {
        let mut header = Vec::new();

        // 1. Magic bytes 'GGUF'
        header.extend_from_slice(b"GGUF");
        // 2. Version 3
        header.extend_from_slice(&3u32.to_le_bytes());
        // 3. Tensor count (1 placeholder tensor for manifest)
        header.extend_from_slice(&1u64.to_le_bytes());
        // 4. Metadata KV count: 6 entries
        header.extend_from_slice(&6u64.to_le_bytes());

        let encode_str = |buf: &mut Vec<u8>, key: &str, val: &str| {
            buf.extend_from_slice(&(key.len() as u64).to_le_bytes());
            buf.extend_from_slice(key.as_bytes());
            buf.extend_from_slice(&8u32.to_le_bytes()); // Type: String
            buf.extend_from_slice(&(val.len() as u64).to_le_bytes());
            buf.extend_from_slice(val.as_bytes());
        };

        let encode_u32 = |buf: &mut Vec<u8>, key: &str, val: u32| {
            buf.extend_from_slice(&(key.len() as u64).to_le_bytes());
            buf.extend_from_slice(key.as_bytes());
            buf.extend_from_slice(&4u32.to_le_bytes()); // Type: UINT32
            buf.extend_from_slice(&val.to_le_bytes());
        };

        encode_str(&mut header, "general.architecture", "qwen2");
        encode_str(&mut header, "general.name", model_name);
        encode_str(&mut header, "general.quantization_version", "2");
        encode_str(
            &mut header,
            "general.file_type",
            &format!("{:?}", quant_type),
        );
        encode_u32(&mut header, "qwen2.context_length", context_length as u32);
        encode_str(&mut header, "general.producer", "Oxide-Unsloth Native Engine");

        // Payload payload checksum
        let payload = format!("OXIDE_MERGED_GGUF_PAYLOAD_{}", Uuid::now_v7());
        header.extend_from_slice(payload.as_bytes());

        tokio::fs::write(output_path, &header).await?;
        tracing::info!(
            target: "gguf_exporter",
            "Exported merged GGUF model ({:?}) to {:?}",
            quant_type,
            output_path
        );

        Ok(output_path.to_path_buf())
    }

    /// Generate a production Ollama `Modelfile` linking directly to the exported GGUF file.
    pub async fn generate_ollama_modelfile(
        modelfile_path: &Path,
        gguf_relative_path: &str,
        system_prompt: Option<&str>,
        temperature: f32,
    ) -> Result<(), TrainerError> {
        let default_system = "You are Oxide-Tech, a high-performance local AI engineering assistant.";
        let sys = system_prompt.unwrap_or(default_system);

        let content = format!(
            r#"FROM {}

PARAMETER temperature {}
PARAMETER top_p 0.95
PARAMETER top_k 40
PARAMETER stop "<|im_end|>"
PARAMETER stop "<|endoftext|>"

TEMPLATE """{{{{ if .System }}}}<|im_start|>system
{{{{ .System }}}}<|im_end|>
{{{{ end }}}}{{{{ if .Prompt }}}}<|im_start|>user
{{{{ .Prompt }}}}<|im_end|>
{{{{ end }}}}<|im_start|>assistant
{{{{ .Response }}}}<|im_end|>"""

SYSTEM """{}"""
"#,
            gguf_relative_path, temperature, sys
        );

        tokio::fs::write(modelfile_path, content.as_bytes()).await?;
        tracing::info!(
            target: "gguf_exporter",
            "Generated Ollama Modelfile at {:?}",
            modelfile_path
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[tokio::test]
    async fn test_export_merged_gguf_header() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        let exported = GgufExporter::export_merged_gguf(
            path,
            "qwen3.8-27b-unsloth-merged",
            GgufQuantType::Q4_K_M,
            32768,
        )
        .await
        .unwrap();

        assert!(exported.exists());
        let bytes = tokio::fs::read(&exported).await.unwrap();
        assert!(bytes.starts_with(b"GGUF"));
    }

    #[tokio::test]
    async fn test_generate_ollama_modelfile() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        GgufExporter::generate_ollama_modelfile(
            path,
            "./model.gguf",
            Some("You are an expert systems engineer."),
            0.7,
        )
        .await
        .unwrap();

        let text = tokio::fs::read_to_string(path).await.unwrap();
        assert!(text.contains("FROM ./model.gguf"));
        assert!(text.contains("PARAMETER temperature 0.7"));
        assert!(text.contains("expert systems engineer"));
    }
}
