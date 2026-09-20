use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReforgeRequest {
    pub file_path: String,
    pub arch: Option<String>,
    pub summary: Option<bool>,
    pub decompile: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArmVectorTableDto {
    pub initial_sp: String,
    pub reset_handler: String,
    pub hardfault_handler: String,
    pub systick_handler: String,
    pub external_irqs_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtosDetectionDto {
    pub detected_rtos: Option<String>,
    pub confidence: f32,
    pub signatures_found: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyChunkDto {
    pub offset: usize,
    pub entropy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TensorCorePatternDto {
    pub instruction: String,
    pub shape: String,
    pub precision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PtxAnalysisDto {
    pub target_arch: String,
    pub kernel_name: String,
    pub shared_memory_bytes: usize,
    pub uses_async_copy: bool,
    pub inferred_operation: String,
    pub tensor_core_patterns: Vec<TensorCorePatternDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisassembledInstructionDto {
    pub address: String,
    pub mnemonic: String,
    pub length: usize,
    pub is_call: bool,
    pub is_branch: bool,
    pub is_return: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisassembledFunctionDto {
    pub name: String,
    pub start_address: String,
    pub instructions: Vec<DisassembledInstructionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReforgeResult {
    pub domain: String,
    pub file_size: usize,
    pub format: String,
    pub entry_point: Option<String>,
    pub arm_vector_table: Option<ArmVectorTableDto>,
    pub rtos: Option<RtosDetectionDto>,
    pub avg_entropy: f64,
    pub entropy_chunks: Vec<EntropyChunkDto>,
    pub ptx_analysis: Option<PtxAnalysisDto>,
    pub functions: Vec<DisassembledFunctionDto>,
    pub total_instructions: usize,
    pub decompiled_code: Option<String>,
}

pub fn analyze_file(req: ReforgeRequest) -> anyhow::Result<ReforgeResult> {
    let file_path = PathBuf::from(&req.file_path);
    if !file_path.exists() {
        anyhow::bail!("Target file '{}' does not exist", file_path.display());
    }

    let arch = req.arch.unwrap_or_else(|| "auto".to_string());
    let ext = file_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_lowercase();

    if ext == "bin" || ext == "hex" || arch == "arm" || arch == "cortex-m" {
        let buffer = std::fs::read(&file_path).context("Failed to read firmware image")?;
        let file_size = buffer.len();

        let arm_vector_table = re_forge::ArmVectorTable::parse(&buffer, 0x0800_0000).map(|ivt| {
            ArmVectorTableDto {
                initial_sp: format!("0x{:08X}", ivt.initial_sp),
                reset_handler: format!("0x{:08X}", ivt.reset_handler),
                hardfault_handler: format!("0x{:08X}", ivt.hardfault_handler),
                systick_handler: format!("0x{:08X}", ivt.systick_handler),
                external_irqs_count: ivt.external_irqs.len(),
            }
        });

        let rtos_raw = re_forge::RtosDetector::detect(&buffer);
        let rtos = Some(RtosDetectionDto {
            detected_rtos: rtos_raw.detected_rtos,
            confidence: rtos_raw.confidence,
            signatures_found: rtos_raw.signatures_found,
        });

        let chunks_raw = re_forge::EntropyScanner::scan(&buffer, 4096);
        let avg_entropy: f64 = if !chunks_raw.is_empty() {
            chunks_raw.iter().map(|c| c.entropy).sum::<f64>() / chunks_raw.len() as f64
        } else {
            0.0
        };

        let entropy_chunks = chunks_raw
            .into_iter()
            .map(|c| EntropyChunkDto {
                offset: c.offset,
                entropy: c.entropy,
            })
            .collect();

        return Ok(ReforgeResult {
            domain: "Embedded Firmware / Microcontroller".to_string(),
            file_size,
            format: "Raw Binary / Hex".to_string(),
            entry_point: arm_vector_table
                .as_ref()
                .map(|a| a.reset_handler.clone()),
            arm_vector_table,
            rtos,
            avg_entropy,
            entropy_chunks,
            ptx_analysis: None,
            functions: Vec::new(),
            total_instructions: 0,
            decompiled_code: None,
        });
    }

    if ext == "ptx" || arch == "cuda" {
        let ptx_content =
            std::fs::read_to_string(&file_path).context("Failed to read PTX source file")?;
        let analysis = re_forge::PtxParser::analyze(&ptx_content);

        let ptx_dto = PtxAnalysisDto {
            target_arch: analysis.target_arch,
            kernel_name: analysis.kernel_name,
            shared_memory_bytes: analysis.memory_pattern.shared_memory_bytes,
            uses_async_copy: analysis.memory_pattern.uses_async_copy,
            inferred_operation: analysis.inferred_operation,
            tensor_core_patterns: analysis
                .tensor_core_patterns
                .into_iter()
                .map(|t| TensorCorePatternDto {
                    instruction: t.instruction,
                    shape: t.shape,
                    precision: t.precision,
                })
                .collect(),
        };

        return Ok(ReforgeResult {
            domain: "CUDA GPU PTX".to_string(),
            file_size: ptx_content.len(),
            format: "NVIDIA PTX Assembly".to_string(),
            entry_point: Some(pttx_entry(&ptx_dto.kernel_name)),
            arm_vector_table: None,
            rtos: None,
            avg_entropy: 0.0,
            entropy_chunks: Vec::new(),
            ptx_analysis: Some(ptx_dto),
            functions: Vec::new(),
            total_instructions: 0,
            decompiled_code: None,
        });
    }

    // CPU Binary Analysis (ELF / PE)
    let analyzer = re_forge::BinaryAnalyzer::analyze_file(&file_path)
        .context("Failed to inspect binary with Goblin/Yaxpeax")?;

    let file_size = std::fs::metadata(&file_path)
        .map(|m| m.len() as usize)
        .unwrap_or(0);
    let mut total_instructions = 0;
    let mut functions = Vec::new();

    for func in &analyzer.functions {
        total_instructions += func.instructions.len();
        let instrs = func
            .instructions
            .iter()
            .map(|i| DisassembledInstructionDto {
                address: format!("0x{:08x}", i.address),
                mnemonic: i.mnemonic.clone(),
                length: i.length,
                is_call: i.is_call,
                is_branch: i.is_branch,
                is_return: i.is_return,
            })
            .collect();

        functions.push(DisassembledFunctionDto {
            name: func.name.clone(),
            start_address: format!("0x{:08x}", func.start_address),
            instructions: instrs,
        });
    }

    let decompiled_code = if req.decompile.unwrap_or(false) {
        let mut code = String::new();
        for func in &analyzer.functions {
            code.push_str(&format!("// ── Recovered Function: {} ──\n", func.name));
            code.push_str(&format!(
                "pub fn {}() -> Result<(), Box<dyn std::error::Error>> {{\n",
                func.name
            ));
            code.push_str(&format!(
                "    // Recovered from 0x{:08x} ({} instructions)\n",
                func.start_address,
                func.instructions.len()
            ));
            code.push_str("    Ok(())\n}\n\n");
        }
        Some(code)
    } else {
        None
    };

    Ok(ReforgeResult {
        domain: "Native Executable / Shared Object".to_string(),
        file_size,
        format: format!("{:?}", analyzer.format),
        entry_point: Some(format!("0x{:08x}", analyzer.entry_point)),
        arm_vector_table: None,
        rtos: None,
        avg_entropy: 0.0,
        entropy_chunks: Vec::new(),
        ptx_analysis: None,
        functions,
        total_instructions,
        decompiled_code,
    })
}

fn pttx_entry(k: &str) -> String {
    format!("Kernel: {}", k)
}

#[tauri::command]
pub async fn reforge_analyze_file(request: ReforgeRequest) -> Result<ReforgeResult, String> {
    tokio::task::spawn_blocking(move || analyze_file(request))
        .await
        .map_err(|e| e.to_string())?
        .map_err(|e| e.to_string())
}