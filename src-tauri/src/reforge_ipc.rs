use anyhow::{Context, Result};
use re_forge::{
    ArmVectorTable, BinaryAnalyzer, DisassembledFunction, DisassembledInstruction,
    EntropyScanner, RtosDetector,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReforgeRequest {
    pub file_path: String,
    pub arch: Option<String>,
    pub summary: Option<bool>,
    pub decompile: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArmVectorTableDto {
    pub initial_sp: u32,
    pub reset_handler: u32,
    pub hardfault_handler: u32,
    pub systick_handler: u32,
    pub external_irqs_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RtosDetectionDto {
    pub detected_rtos: Option<String>,
    pub confidence: f32,
    pub signatures_found: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntropyChunkDto {
    pub offset: usize,
    pub entropy: f64,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisassembledInstructionDto {
    pub address: u64,
    pub mnemonic: String,
    pub length: usize,
    pub is_call: bool,
    pub is_branch: bool,
    pub is_return: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DisassembledFunctionDto {
    pub name: String,
    pub start_address: u64,
    pub instructions: Vec<DisassembledInstructionDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtxMemoryPatternDto {
    pub shared_memory_bytes: usize,
    pub uses_async_copy: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TensorCorePatternDto {
    pub instruction: String,
    pub shape: String,
    pub precision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PtxAnalysisDto {
    pub target_arch: String,
    pub kernel_name: String,
    pub memory_pattern: PtxMemoryPatternDto,
    pub inferred_operation: String,
    pub tensor_core_patterns: Vec<TensorCorePatternDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecompiledFunctionDto {
    pub name: String,
    pub rust_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReforgeResult {
    pub domain: String,
    pub file_size: usize,
    pub binary_format: Option<String>,
    pub entry_point: Option<u64>,
    pub arm_vector_table: Option<ArmVectorTableDto>,
    pub rtos_detection: Option<RtosDetectionDto>,
    pub entropy_chunks: Vec<EntropyChunkDto>,
    pub avg_entropy: f64,
    pub disassembled_functions: Vec<DisassembledFunctionDto>,
    pub total_instructions: usize,
    pub ptx_analysis: Option<PtxAnalysisDto>,
    pub decompiled_functions: Vec<DecompiledFunctionDto>,
    pub error: Option<String>,
}

fn instruction_to_dto(inst: &DisassembledInstruction) -> DisassembledInstructionDto {
    DisassembledInstructionDto {
        address: inst.address,
        mnemonic: inst.mnemonic.clone(),
        length: inst.length,
        is_call: inst.is_call,
        is_branch: inst.is_branch,
        is_return: inst.is_return,
    }
}

fn function_to_dto(func: &DisassembledFunction) -> DisassembledFunctionDto {
    DisassembledFunctionDto {
        name: func.name.clone(),
        start_address: func.start_address,
        instructions: func.instructions.iter().map(instruction_to_dto).collect(),
    }
}

pub fn analyze_file(request: ReforgeRequest) -> Result<ReforgeResult> {
    let file_path = PathBuf::from(&request.file_path);
    if !file_path.exists() {
        anyhow::bail!("Target file '{}' does not exist", file_path.display());
    }

    let ext = file_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_lowercase();

    let arch = request.arch.unwrap_or_else(|| "auto".to_string());
    let summary = request.summary.unwrap_or(false);
    let decompile = request.decompile.unwrap_or(false);

    if ext == "bin" || ext == "hex" || arch == "arm" || arch == "cortex-m" {
        let buffer = std::fs::read(&file_path).context("Failed to read firmware image")?;

        let mut result = ReforgeResult {
            domain: "Embedded Firmware / Microcontroller".to_string(),
            file_size: buffer.len(),
            binary_format: None,
            entry_point: None,
            arm_vector_table: None,
            rtos_detection: None,
            entropy_chunks: Vec::new(),
            avg_entropy: 0.0,
            disassembled_functions: Vec::new(),
            total_instructions: 0,
            ptx_analysis: None,
            decompiled_functions: Vec::new(),
            error: None,
        };

        if let Some(ivt) = ArmVectorTable::parse(&buffer, 0x0800_0000) {
            result.arm_vector_table = Some(ArmVectorTableDto {
                initial_sp: ivt.initial_sp,
                reset_handler: ivt.reset_handler,
                hardfault_handler: ivt.hardfault_handler,
                systick_handler: ivt.systick_handler,
                external_irqs_count: ivt.external_irqs.len(),
            });
        }

        let rtos = RtosDetector::detect(&buffer);
        result.rtos_detection = Some(RtosDetectionDto {
            detected_rtos: rtos.detected_rtos,
            confidence: rtos.confidence,
            signatures_found: rtos.signatures_found,
        });

        let entropy_chunks = EntropyScanner::scan(&buffer, 4096);
        result.entropy_chunks = entropy_chunks
            .iter()
            .map(|c| EntropyChunkDto {
                offset: c.offset,
                entropy: c.entropy,
                size: c.size,
            })
            .collect();
        result.avg_entropy = if !entropy_chunks.is_empty() {
            entropy_chunks.iter().map(|c| c.entropy).sum::<f64>() / entropy_chunks.len() as f64
        } else {
            0.0
        };

        return Ok(result);
    }

    if ext == "ptx" || arch == "cuda" {
        let ptx_content = std::fs::read_to_string(&file_path).context("Failed to read PTX source file")?;
        let analysis = re_forge::PtxParser::analyze(&ptx_content);

        return Ok(ReforgeResult {
            domain: "GPU PTX Kernel".to_string(),
            file_size: ptx_content.len(),
            binary_format: None,
            entry_point: None,
            arm_vector_table: None,
            rtos_detection: None,
            entropy_chunks: Vec::new(),
            avg_entropy: 0.0,
            disassembled_functions: Vec::new(),
            total_instructions: 0,
            ptx_analysis: Some(PtxAnalysisDto {
                target_arch: analysis.target_arch,
                kernel_name: analysis.kernel_name,
                memory_pattern: PtxMemoryPatternDto {
                    shared_memory_bytes: analysis.memory_pattern.shared_memory_bytes,
                    uses_async_copy: analysis.memory_pattern.uses_async_copy,
                },
                inferred_operation: analysis.inferred_operation,
                tensor_core_patterns: analysis
                    .tensor_core_patterns
                    .iter()
                    .map(|tcp| TensorCorePatternDto {
                        instruction: tcp.instruction.clone(),
                        shape: tcp.shape.clone(),
                        precision: tcp.precision.clone(),
                    })
                    .collect(),
            }),
            decompiled_functions: Vec::new(),
            error: None,
        });
    }

    let analyzer = BinaryAnalyzer::analyze_file(&file_path)
        .context("Failed to inspect binary with Goblin/Yaxpeax")?;

    let mut total_instructions = 0;
    let mut disassembled_functions = Vec::new();
    for func in &analyzer.functions {
        total_instructions += func.instructions.len();
        if !summary {
            disassembled_functions.push(function_to_dto(func));
        }
    }

    let decompiled_functions = Vec::new();
    if decompile {
        // NeuralDecompiler requires an LLM provider which is not available in the desktop context
        // Decompilation would need to be triggered via the gateway API instead
    }

    Ok(ReforgeResult {
        domain: "CPU Binary (ELF/PE)".to_string(),
        file_size: std::fs::metadata(&file_path)?.len() as usize,
        binary_format: Some(format!("{:?}", analyzer.format)),
        entry_point: Some(analyzer.entry_point),
        arm_vector_table: None,
        rtos_detection: None,
        entropy_chunks: Vec::new(),
        avg_entropy: 0.0,
        disassembled_functions,
        total_instructions,
        ptx_analysis: None,
        decompiled_functions,
        error: None,
    })
}