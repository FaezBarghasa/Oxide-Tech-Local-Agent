export type TabId =
  | 'chat'
  | 'overview'
  | 'graph'
  | 'memory'
  | 'infra'
  | 'grpc'
  | 'rag'
  | 'mcp'
  | 'catalog'
  | 'dataset'
  | 'training'
  | 'soup'
  | 'sglang'
  | 'endpoints'
  | 'verify'
  | 'doctor'
  | 'reforge'
  | 'settings';

export interface GraphNode {
  id: string;
  label: string;
  category: 'project' | 'mcu' | 'firmware' | 'hardware' | 'protocol' | 'lora';
  subLabel?: string;
  description: string;
  status?: 'deployed' | 'active' | 'prototype' | 'testing';
  drcScore?: number;
  tokens?: number;
  color?: string;
  x?: number;
  y?: number;
  vx?: number;
  vy?: number;
  radius?: number;
  tags?: string[];
  metrics?: {
    frequency?: string;
    vramMb?: number;
    voltage?: string;
    busSpeed?: string;
    passRate?: string;
  };
}

export interface GraphLink {
  source: string;
  target: string;
  label: string;
  type: 'implements' | 'uses' | 'communicates' | 'assisted_by' | 'integrates' | 'verifies';
  strength?: number;
}

export type ChatMode = 'chat' | 'code' | 'architect' | 'ask' | 'autonomous' | 'research';

export interface SwarmTaskNode {
  id: string;
  title: string;
  role: 'Architect' | 'Coder' | 'Debugger' | 'DevOps' | 'Reviewer';
  description: string;
  dependencies: string[];
  status: 'Pending' | 'Running' | 'Passed' | 'Failed' | 'Skipped';
  result?: string;
  retryCount: number;
}

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'tool' | 'system';
  content: string;
  timestamp: string;
  mode?: ChatMode;
  toolName?: string;
  toolStatus?: 'success' | 'running' | 'failed';
  toolDuration?: string;
  reasoningTrace?: string;
  thinkTokens?: number;
  complexity?: 'Routine' | 'Moderate' | 'DeepReasoning' | 'FormalProof';
  meta?: {
    model?: string;
    tokens?: number;
    mode?: ChatMode;
  };
}

export interface ModelInfo {
  id: string;
  name: string;
  fmt: 'AWQ' | 'NF4' | 'BF16' | 'GGUF' | 'Safetensors';
  params: number;
  tp: number;
  vram: number;
  dl: boolean;
  desc: string;
  tags: string[];
}

export interface LoraAdapter {
  id: string;
  name: string;
  domain: string;
  rank: number;
  weight: number;
  color: string;
  status: 'loaded' | 'standby' | 'training';
}

export interface McpTool {
  name: string;
  description: string;
  category: 'embedded' | 'cad' | 'rag' | 'debug' | 'jit';
  status: 'ready' | 'standby' | 'network' | 'active';
  latency: string;
  schemaSample: string;
}

export interface SystemTelemetry {
  gpu0Vram: number;
  gpu1Vram: number;
  gpu0Temp: number;
  gpu1Temp: number;
  cacheHit: number;
  simdThroughput: number;
  activeSessions: number;
  grpcLatencyMs: number;
}

export interface LogEntry {
  id: string;
  time: string;
  level: 'info' | 'ok' | 'warn' | 'err';
  text: string;
  subsystem: string;
}

export interface PhaseInfo {
  id: number;
  tag: string;
  title: string;
  description: string;
  status: 'COMPLETE' | 'IN PROGRESS' | 'PENDING' | 'ACTIVE';
  progress: number;
  color: 'cyan' | 'purple' | 'emerald' | 'amber' | 'rose' | 'orange';
  duration: string;
  tasksCount: number;
}

// Doctor Diagnostics
export interface DiagnosticCheck {
  name: string;
  command: string;
  required: boolean;
  passed: boolean;
  version: string;
  error: string | null;
}

export interface DoctorResult {
  checks: DiagnosticCheck[];
  passed: number;
  warnings: number;
  failed: number;
  ready: boolean;
  timestamp: string;
}

export interface UdevInstallResult {
  success: boolean;
  message: string;
}

// RE-Forge Binary/PTX Analysis
export interface ArmVectorTableDto {
  initial_sp: number;
  reset_handler: number;
  hardfault_handler: number;
  systick_handler: number;
  external_irqs_count: number;
}

export interface RtosDetectionDto {
  detected_rtos: string | null;
  confidence: number;
  signatures_found: string[];
}

export interface EntropyChunkDto {
  offset: number;
  entropy: number;
  size: number;
}

export interface DisassembledInstructionDto {
  address: number;
  mnemonic: string;
  length: number;
  is_call: boolean;
  is_branch: boolean;
  is_return: boolean;
}

export interface DisassembledFunctionDto {
  name: string;
  start_address: number;
  instructions: DisassembledInstructionDto[];
}

export interface PtxMemoryPatternDto {
  shared_memory_bytes: number;
  uses_async_copy: boolean;
}

export interface TensorCorePatternDto {
  instruction: string;
  shape: string;
  precision: string;
}

export interface PtxAnalysisDto {
  target_arch: string;
  kernel_name: string;
  memory_pattern: PtxMemoryPatternDto;
  inferred_operation: string;
  tensor_core_patterns: TensorCorePatternDto[];
}

export interface DecompiledFunctionDto {
  name: string;
  rust_code: string;
}

export interface ReforgeRequest {
  file_path: string;
  arch?: string;
  summary?: boolean;
  decompile?: boolean;
}

export interface ReforgeResult {
  domain: string;
  file_size: number;
  binary_format: string | null;
  entry_point: number | null;
  arm_vector_table: ArmVectorTableDto | null;
  rtos_detection: RtosDetectionDto | null;
  entropy_chunks: EntropyChunkDto[];
  avg_entropy: number;
  disassembled_functions: DisassembledFunctionDto[];
  total_instructions: number;
  ptx_analysis: PtxAnalysisDto | null;
  decompiled_functions: DecompiledFunctionDto[];
  error: string | null;
}

// Verifier Suite
export interface VerifierReportDto {
  stage: string;
  passed: boolean;
  stdout: string;
  stderr: string;
  duration_ms: number;
}

export interface EvidenceBundleDto {
  bundle_id: string;
  task_id: string;
  git_diff: string;
  reports: VerifierReportDto[];
  verified_success: boolean;
  hitl_decision: string | null;
  timestamp: string;
}

export interface VerifierRequest {
  workspace_path?: string;
  export_path?: string;
}

export interface VerifierResult {
  evidence_bundle: EvidenceBundleDto | null;
  overall_passed: boolean;
  error: string | null;
}

// Hardware / probe-rs
export interface ProbeDeviceDto {
  identifier: string;
  vendor_id: number;
  product_id: number;
  serial_number: string | null;
  product_name: string | null;
  manufacturer: string | null;
}

export interface ProbeDevicesResult {
  devices: ProbeDeviceDto[];
  error: string | null;
}

export interface CoreInfoDto {
  name: string;
  core_type: string;
}

export interface MemoryRegionDto {
  name: string;
  range_start: number;
  range_end: number;
  is_flash: boolean;
  is_ram: boolean;
}

export interface ChipInfoDto {
  name: string;
  part: string;
  cores: CoreInfoDto[];
  memory_regions: MemoryRegionDto[];
}

export interface FlashRequest {
  device_identifier: string;
  firmware_path: string;
  chip_name?: string;
  verify?: boolean;
}

export interface FlashResult {
  success: boolean;
  message: string;
  bytes_written: number | null;
  duration_ms: number | null;
}

// Gateway Daemon
export interface GatewayLogEntry {
  id: string;
  time: string;
  level: 'info' | 'warn' | 'error';
  text: string;
}

// Config
export interface ConfigFile {
  content: string;
}
