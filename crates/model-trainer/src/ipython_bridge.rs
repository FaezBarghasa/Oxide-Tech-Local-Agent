//! # IPython & Jupyter Notebook Integration Bridge for Model Trainer
//!
//! Generates and executes interactive Python/IPython training scripts,
//! marshals datasets and LoRA configuration into notebook-compatible snippets,
//! and captures stdout/stderr telemetry back into the Rust model-trainer pipeline.

use crate::{TrainKind, TrainRequest, TrainerError};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use tracing::info;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IPythonNotebookConfig {
    pub python_binary: String,
    pub work_dir: PathBuf,
    pub enable_triton_kernels: bool,
    pub max_seq_length: usize,
}

impl Default for IPythonNotebookConfig {
    fn default() -> Self {
        Self {
            python_binary: "python3".to_string(),
            work_dir: std::env::temp_dir(),
            enable_triton_kernels: true,
            max_seq_length: 4096,
        }
    }
}

pub struct IPythonTrainerBridge {
    pub config: IPythonNotebookConfig,
}

impl IPythonTrainerBridge {
    pub fn new(config: IPythonNotebookConfig) -> Self {
        Self { config }
    }

    /// Generates executable Python/IPython code snippet matching Unsloth v0.1.900-beta API
    pub fn generate_training_script(&self, req: &TrainRequest) -> String {
        let lora_rank = req.lora_rank;
        let lora_alpha = req.lora_alpha;
        let _epochs = req.epochs;
        let lr = req.learning_rate;
        let _batch_size = req.batch_size;
        let base_model = &req.base_model;
        let out_dir = req.output_dir.display();
        let ds_path = req.dataset_path.display();

        match req.kind {
            TrainKind::Grpo => format!(
                r#"# Auto-generated Oxide-Unsloth GRPO IPython Script
from oxide_unsloth import FastLanguageModel, OxideGRPOTrainer
from oxide_unsloth.rewards import rust_compiler_reward, memory_safety_reward, spice_simulation_reward

model, tokenizer = FastLanguageModel.from_pretrained(
    "{base_model}",
    max_seq_length={max_seq_len},
    load_in_4bit=True,
)
model = FastLanguageModel.get_peft_model(
    model,
    r={lora_rank},
    lora_alpha={lora_alpha},
)

trainer = OxideGRPOTrainer(
    model=model,
    reward_funcs=[rust_compiler_reward, memory_safety_reward, spice_simulation_reward],
    train_dataset="{ds_path}",
    group_size=4,
    beta=0.04,
    learning_rate={lr},
    output_dir="{out_dir}",
)
trainer.train()
model.save_pretrained_merged("{out_dir}/merged_16bit", tokenizer, save_method="merged_16bit")
model.save_pretrained_gguf("{out_dir}/gguf", tokenizer, quantization_method="q4_k_m")
print("[IPython-Bridge] GRPO Training and GGUF export complete.")
"#,
                base_model = base_model,
                max_seq_len = self.config.max_seq_length,
                lora_rank = lora_rank,
                lora_alpha = lora_alpha,
                lr = lr,
                out_dir = out_dir,
                ds_path = ds_path,
            ),
            _ => format!(
                r#"# Auto-generated Oxide-Unsloth SFT IPython Script
from oxide_unsloth import FastLanguageModel, OxideSFTTrainer

model, tokenizer = FastLanguageModel.from_pretrained(
    "{base_model}",
    max_seq_length={max_seq_len},
    load_in_4bit=True,
)
model = FastLanguageModel.get_peft_model(
    model,
    r={lora_rank},
    lora_alpha={lora_alpha},
)

trainer = OxideSFTTrainer(
    model=model,
    tokenizer=tokenizer,
    train_dataset="{ds_path}",
)
trainer.train()
model.save_pretrained_merged("{out_dir}/merged_16bit", tokenizer, save_method="merged_16bit")
model.save_pretrained_gguf("{out_dir}/gguf", tokenizer, quantization_method="q4_k_m")
print("[IPython-Bridge] SFT Training and GGUF export complete.")
"#,
                base_model = base_model,
                max_seq_len = self.config.max_seq_length,
                lora_rank = lora_rank,
                lora_alpha = lora_alpha,
                out_dir = out_dir,
                ds_path = ds_path,
            ),
        }
    }

    /// Executes training through python-bridge subprocess and collects output
    pub async fn execute_notebook_session(
        &self,
        req: &TrainRequest,
        python_bridge_dir: &Path,
    ) -> Result<String, TrainerError> {
        let script_code = self.generate_training_script(req);
        let script_path = self
            .config
            .work_dir
            .join(format!("oxide_train_ipython_{}.py", Uuid::now_v7()));

        tokio::fs::write(&script_path, &script_code).await?;
        info!("Executing IPython bridge script at {:?}", script_path);

        let output = Command::new(&self.config.python_binary)
            .arg(&script_path)
            .env("PYTHONPATH", python_bridge_dir)
            .output();

        let _ = tokio::fs::remove_file(&script_path).await;

        match output {
            Ok(res) => {
                let stdout = String::from_utf8_lossy(&res.stdout).to_string();
                let stderr = String::from_utf8_lossy(&res.stderr).to_string();
                if res.status.success() {
                    Ok(stdout)
                } else {
                    Err(TrainerError::ProcessError(format!(
                        "IPython training failed with exit code {:?}:\nStdout: {}\nStderr: {}",
                        res.status.code(),
                        stdout,
                        stderr
                    )))
                }
            }
            Err(e) => Err(TrainerError::ProcessError(format!(
                "Failed to spawn Python process {}: {}",
                self.config.python_binary, e
            ))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipython_script_generation_grpo_and_sft() {
        let bridge = IPythonTrainerBridge::new(IPythonNotebookConfig::default());
        let req = TrainRequest {
            task_id: Uuid::now_v7(),
            kind: TrainKind::Grpo,
            base_model: "qwen2.5-coder:14b".to_string(),
            dataset_path: PathBuf::from("/tmp/ds.json"),
            output_dir: PathBuf::from("/tmp/out"),
            epochs: 2,
            learning_rate: 2e-5,
            batch_size: 4,
            lora_rank: 16,
            lora_alpha: 32,
            qlora_config: None,
        };

        let grpo_code = bridge.generate_training_script(&req);
        assert!(grpo_code.contains("OxideGRPOTrainer"));
        assert!(grpo_code.contains("rust_compiler_reward"));

        let mut req_sft = req.clone();
        req_sft.kind = TrainKind::Sft;
        let sft_code = bridge.generate_training_script(&req_sft);
        assert!(sft_code.contains("OxideSFTTrainer"));
    }
}
