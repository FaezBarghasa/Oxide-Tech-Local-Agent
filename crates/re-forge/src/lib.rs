pub mod analyzer;
pub mod cfg;
pub mod cuda;
pub mod firmware;
pub mod mir;
pub mod neural_decompiler;
pub mod sass_adapter;

pub use analyzer::{BinaryAnalyzer, BinaryFormat, DisassembledFunction, DisassembledInstruction};
pub use cfg::{BasicBlock, ControlFlowGraph};
pub use cuda::{
    CudaAnalyzer, CudaGraphBridge, CudaKernel, CudaKernelNode, CudaMemoryAccessNode,
    CudaReconstructionResult, MemoryAccessPattern, NeuralCudaLifter, PtxAnalysis, PtxParser,
    TensorCorePattern,
};
pub use firmware::{
    ArmVectorTable, EntropyChunk, EntropyScanner, RtosDetectionResult, RtosDetector,
    SvdPeripheralMap,
};
pub use mir::{MirBlock, MirFunction, MirOp};
pub use neural_decompiler::{DecompilationResult, NeuralDecompiler};
pub use sass_adapter::{SassAdapter, SassDisassembly, SassInstruction};
