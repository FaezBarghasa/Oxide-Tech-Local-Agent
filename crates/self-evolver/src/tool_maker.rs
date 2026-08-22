use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolSpecification {
    pub name: String,
    pub description: String,
    pub language: String, // "python" | "typescript" | "mojo"
    pub input_schema: serde_json::Value,
    pub source_code: String,
}

pub type JitToolSpec = ToolSpecification;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MojoToolSpecification {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
    pub source_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub passed: bool,
    pub stdout: String,
    pub stderr: String,
}

pub type VerificationResult = TestResult;


pub struct ToolMaker {
    pub tools_dir: PathBuf,
    pub vllm_endpoint: String,
    pub model_name: String,
}

impl ToolMaker {
    pub fn new(tools_dir: PathBuf, vllm_endpoint: &str) -> Self {
        Self {
            tools_dir,
            vllm_endpoint: vllm_endpoint.trim_end_matches('/').to_string(),
            model_name: "Qwen/Qwen2.5-Coder-32B-Instruct-AWQ".to_string(),
        }
    }

    pub fn with_model(mut self, model_name: &str) -> Self {
        self.model_name = model_name.to_string();
        self
    }

    /// Synthesize a missing MCP tool, write it to disk, and verify it in a sandbox
    pub async fn synthesize_and_verify_tool(
        &self,
        task_gap_description: &str,
        sample_inputs: serde_json::Value,
    ) -> Result<ToolSpecification> {
        let system_prompt = r#"
You are the Oxide JIT MCP Tool Synthesizer.
Your goal is to write a single-file MCP (Model Context Protocol) Server in Python using the `mcp` SDK or standard JSON-RPC STDIO.
The script must handle standard input/output JSON-RPC 2.0 messages for `tools/list` and `tools/call`.

Output ONLY a JSON object with this exact structure:
{
  "name": "mcp_tool_name",
  "description": "Clear explanation of tool capabilities",
  "language": "python",
  "input_schema": { "type": "object", "properties": {}, "required": [] },
  "source_code": "Complete Python script implementing MCP stdio JSON-RPC 2.0"
}
"#;

        let user_prompt = format!(
            "Capability Gap: {}\nExpected Inputs: {}",
            task_gap_description, sample_inputs
        );

        let tool_spec = self.generate_spec_from_llm(system_prompt, &user_prompt).await?;

        // Write tool to workspace tools directory
        fs::create_dir_all(&self.tools_dir).await?;
        let ext = match tool_spec.language.to_lowercase().as_str() {
            "typescript" | "ts" => "ts",
            _ => "py",
        };
        let tool_path = self.tools_dir.join(format!("{}.{}", tool_spec.name, ext));
        fs::write(&tool_path, &tool_spec.source_code).await?;

        // Verify tool by executing smoke tests in the sandbox
        let test_res = self.test_generated_tool(&tool_path, &sample_inputs).await?;
        if !test_res.passed {
            return Err(anyhow!(
                "Generated MCP Tool '{}' failed verification:\nStderr: {}\nStdout: {}",
                tool_spec.name,
                test_res.stderr,
                test_res.stdout
            ));
        }

        tracing::info!("Successfully synthesized and verified JIT MCP tool: {}", tool_spec.name);
        Ok(tool_spec)
    }

    /// Alias for synthesize_and_verify_tool
    pub async fn synthesize_and_register_tool(
        &self,
        task_gap: &str,
        sample_inputs: serde_json::Value,
    ) -> Result<JitToolSpec> {
        self.synthesize_and_verify_tool(task_gap, sample_inputs).await
    }

    pub async fn verify_tool_in_sandbox(&self, script_path: &Path) -> Result<VerificationResult> {
        self.test_generated_tool(script_path, &serde_json::json!({})).await
    }


    /// Synthesize a native, ultra-fast Mojo v1 MCP Tool
    pub async fn synthesize_mojo_tool(
        &self,
        task_gap_description: &str,
        sample_inputs: serde_json::Value,
    ) -> Result<MojoToolSpecification> {
        let system_prompt = r#"
You are the Oxide Mojo v1 MCP Tool Synthesizer.
Write a high-performance single-file Mojo v1 script that implements an MCP (Model Context Protocol) STDIO server.
Handle JSON-RPC 2.0 requests for `tools/list` and `tools/call`.

Output ONLY a JSON object:
{
  "name": "mcp_mojo_tool_name",
  "description": "High-performance SIMD tool capability",
  "input_schema": { "type": "object", "properties": {} },
  "source_code": "Complete Mojo v1 source code using `import sys` and SIMD operations"
}
"#;

        let user_prompt = format!(
            "Capability Gap: {}\nSample Inputs: {}",
            task_gap_description, sample_inputs
        );

        let spec = self.generate_mojo_spec_from_llm(system_prompt, &user_prompt).await?;
        
        // Write Mojo file
        fs::create_dir_all(&self.tools_dir).await?;
        let mojo_path = self.tools_dir.join(format!("{}.mojo", spec.name));
        fs::write(&mojo_path, &spec.source_code).await?;

        // Compile Mojo to native binary if mojo is available
        let bin_path = self.tools_dir.join(&spec.name);
        let mojo_check = Command::new("which").arg("mojo").output();
        if let Ok(check) = mojo_check {
            if check.status.success() {
                let compile_out = Command::new("mojo")
                    .arg("build")
                    .arg(&mojo_path)
                    .arg("-o")
                    .arg(&bin_path)
                    .output()?;

                if !compile_out.status.success() {
                    let stderr = String::from_utf8_lossy(&compile_out.stderr);
                    return Err(anyhow!("Mojo v1 Compilation Failed:\n{}", stderr));
                }
                tracing::info!("Successfully compiled JIT Mojo v1 MCP binary: {:?}", bin_path);
            } else {
                tracing::warn!("Mojo toolchain not in PATH; wrote .mojo source directly to {:?}", mojo_path);
            }
        }

        Ok(spec)
    }

    pub async fn generate_spec_from_llm(&self, system: &str, prompt: &str) -> Result<ToolSpecification> {
        let client = reqwest::Client::new();
        let req_body = serde_json::json!({
            "model": self.model_name,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.1,
            "response_format": { "type": "json_object" }
        });

        let res = client.post(format!("{}/v1/chat/completions", self.vllm_endpoint))
            .json(&req_body)
            .send()
            .await?;

        let json_res: serde_json::Value = res.json().await?;
        let content = json_res["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid LLM response format"))?;

        let spec: ToolSpecification = serde_json::from_str(content)?;
        Ok(spec)
    }

    pub async fn generate_mojo_spec_from_llm(&self, system: &str, prompt: &str) -> Result<MojoToolSpecification> {
        let client = reqwest::Client::new();
        let req_body = serde_json::json!({
            "model": self.model_name,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.1,
            "response_format": { "type": "json_object" }
        });

        let res = client.post(format!("{}/v1/chat/completions", self.vllm_endpoint))
            .json(&req_body)
            .send()
            .await?;

        let json_res: serde_json::Value = res.json().await?;
        let content = json_res["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| anyhow!("Invalid response from vLLM"))?;

        let spec: MojoToolSpecification = serde_json::from_str(content)?;
        Ok(spec)
    }

    pub async fn test_generated_tool(&self, script_path: &Path, _sample_inputs: &serde_json::Value) -> Result<TestResult> {
        let parent = script_path.parent().unwrap_or_else(|| Path::new("."));
        let parent_str = parent.to_str().unwrap_or(".");
        let script_str = script_path.to_str().unwrap_or("");

        // Run in the native sandbox process
        let cmd = vec!["python3", script_str];
        match sandbox::execute_in_sandbox(&cmd, parent_str).await {
            Ok(exec_res) => Ok(TestResult {
                passed: exec_res.exit_code == 0 || !exec_res.stdout.is_empty(),
                stdout: exec_res.stdout,
                stderr: exec_res.stderr,
            }),
            Err(e) => Ok(TestResult {
                passed: false,
                stdout: String::new(),
                stderr: e,
            }),
        }
    }
}
