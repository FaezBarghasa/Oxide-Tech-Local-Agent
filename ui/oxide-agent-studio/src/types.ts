export type TabId =
  | 'chat'
  | 'overview'
  | 'graph'
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
  | 'verify';

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

export type ChatMode = 'chat' | 'code' | 'research' | 'scrape' | 'agent';

export interface ChatMessage {
  id: string;
  role: 'user' | 'assistant' | 'tool' | 'system';
  content: string;
  timestamp: string;
  toolName?: string;
  toolStatus?: 'success' | 'running' | 'failed';
  toolDuration?: string;
  meta?: {
    model?: string;
    tokens?: number;
    mode?: ChatMode;
  };
}

export interface ModelInfo {
  id: string;
  name: string;
  fmt: 'AWQ' | 'NF4' | 'BF16' | 'GGUF';
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
  status: 'COMPLETE' | 'IN PROGRESS' | 'PENDING';
  progress: number;
  color: 'cyan' | 'purple' | 'emerald' | 'amber' | 'rose' | 'orange';
  duration: string;
  tasksCount: number;
}
