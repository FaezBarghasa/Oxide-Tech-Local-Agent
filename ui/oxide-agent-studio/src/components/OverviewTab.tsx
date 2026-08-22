import React, { useState, useEffect } from 'react';
import { PhaseInfo, LogEntry } from '../types';
import {
  Activity,
  CheckCircle2,
  Clock,
  Cpu,
  Layers,
  Network,
  Play,
  RotateCcw,
  Sparkles,
  Zap,
} from 'lucide-react';

interface OverviewTabProps {
  onNavigateTab: (tab: any) => void;
}

export const OverviewTab: React.FC<OverviewTabProps> = ({ onNavigateTab }) => {
  const [telemetry, setTelemetry] = useState({
    gpu0: 18.4,
    gpu1: 18.1,
    cacheHit: 87.3,
    sessions: 3,
    simd: 847,
  });

  const [latencyHistory, setLatencyHistory] = useState<number[]>([
    132, 138, 142, 135, 129, 145, 150, 141, 137, 133, 140, 136, 144, 139, 142,
  ]);

  const [simdHistory, setSimdHistory] = useState<number[]>([
    810, 825, 840, 835, 860, 847, 855, 830, 845, 865, 870, 852, 848, 860, 847,
  ]);

  const [logs, setLogs] = useState<LogEntry[]>([
    { id: '1', time: '14:23:01', level: 'info', subsystem: 'Unsloth FastLanguageModel', text: 'FastLanguageModel.from_pretrained("Qwen/Qwen2.5-32B-Instruct", max_seq_length=16384, load_in_4bit=True)' },
    { id: '2', time: '14:23:02', level: 'ok', subsystem: 'SGLang TP=2', text: 'Request req_8f3m · TTFT=142ms · 284 tokens · TP=2 · RadixAttention hit' },
    { id: '3', time: '14:23:03', level: 'info', subsystem: 'Mojo SIMD', text: 'SIMD embedding batch · 1,240 chunks · 847 MB/s throughput' },
    { id: '4', time: '14:23:05', level: 'ok', subsystem: 'Unsloth GRPO', text: 'Step 420/1200 · loss=0.0381 · cargo check reward pass=91.2% · 5.2x speedup' },
    { id: '5', time: '14:23:07', level: 'warn', subsystem: 'GPU Watchdog', text: 'Dual RTX 3090 VRAM at 76.6% (Unsloth QDoRA saves 80% VRAM vs FP16)' },
    { id: '6', time: '14:23:08', level: 'info', subsystem: 'gRPC CAD Bridge', text: 'kicad_bridge.RunDRC completed · 0 violations · 38ms' },
    { id: '7', time: '14:23:10', level: 'ok', subsystem: 'RadixAttention', text: 'Prefix cache hit ratio 87.3% across 16K active context window' },
  ]);

  const phases: PhaseInfo[] = [
    {
      id: 0,
      tag: 'Phase 0',
      title: 'System Toolchains & Native Infra',
      description: 'Pop!_OS 24.04 toolchains, sccache+lld, SurrealDB, Qdrant, SGLang TP=2 server',
      status: 'COMPLETE',
      progress: 100,
      color: 'cyan',
      duration: 'Days 1–2',
      tasksCount: 3,
    },
    {
      id: 1,
      tag: 'Phase 1',
      title: 'gRPC Python CAD Bridge',
      description: 'Protobuf codegen, KiCad 8/9 schematic generation, Blender STEP→GLTF tessellation',
      status: 'IN PROGRESS',
      progress: 65,
      color: 'purple',
      duration: 'Days 3–5',
      tasksCount: 3,
    },
    {
      id: 2,
      tag: 'Phase 2',
      title: 'RAG & Context Compression',
      description: 'Tree-Sitter AST pruning, steno.rs token compression, Qdrant vector search',
      status: 'PENDING',
      progress: 20,
      color: 'emerald',
      duration: 'Days 6–8',
      tasksCount: 2,
    },
    {
      id: 3,
      tag: 'Phase 3',
      title: 'MCP Server Suite & Sandbox',
      description: 'STDIO JSON-RPC 2.0 protocol, JIT tool synthesizer, bubblewrap sandboxing',
      status: 'PENDING',
      progress: 10,
      color: 'amber',
      duration: 'Days 9–11',
      tasksCount: 2,
    },
    {
      id: 4,
      tag: 'Phase 4',
      title: 'ReAct Execution Engine',
      description: 'DAG orchestration in crates/optio, oscillation guard, SRAE gateway router',
      status: 'PENDING',
      progress: 5,
      color: 'cyan',
      duration: 'Days 12–14',
      tasksCount: 2,
    },
    {
      id: 5,
      tag: 'Phase 5',
      title: 'Unsloth GRPO RLVR',
      description: 'FastLanguageModel FSDP-QDoRA, cargo check verifier rewards, self-evolution loop',
      status: 'PENDING',
      progress: 0,
      color: 'orange',
      duration: 'Days 15+',
      tasksCount: 1,
    },
    {
      id: 6,
      tag: 'Phase 6',
      title: 'LoRA Model Soup & Serving',
      description: 'Model Catalog, Dataset Recipes, Training Telemetry, LoRA Soup Blender',
      status: 'PENDING',
      progress: 0,
      color: 'emerald',
      duration: 'Days 18+',
      tasksCount: 4,
    },
  ];

  // Dynamic telemetry updates
  useEffect(() => {
    const interval = setInterval(() => {
      const g0 = parseFloat((17.8 + Math.random() * 1.2).toFixed(1));
      const g1 = parseFloat((17.5 + Math.random() * 1.2).toFixed(1));
      const ch = parseFloat((85 + Math.random() * 4).toFixed(1));
      const simd = Math.round(820 + Math.random() * 50);

      setTelemetry({
        gpu0: g0,
        gpu1: g1,
        cacheHit: ch,
        sessions: 3,
        simd,
      });

      setLatencyHistory((prev) => {
        const next = [...prev.slice(1), Math.round(130 + Math.random() * 25)];
        return next;
      });

      setSimdHistory((prev) => {
        const next = [...prev.slice(1), Math.round(820 + Math.random() * 50)];
        return next;
      });
    }, 3000);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="space-y-6 font-sans">
      {/* Unsloth Hero Banner */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 md:p-7 relative overflow-hidden shadow-xl bg-[radial-gradient(ellipse_at_top_right,rgba(249,115,22,0.12)_0%,transparent_70%)]">
        <div className="flex flex-wrap items-center gap-2 mb-3">
          <span className="px-2.5 py-0.5 rounded-md text-[10px] mono font-bold bg-orange-500/10 text-orange-400 border border-orange-500/30 shadow-[0_0_8px_rgba(249,115,22,0.25)] flex items-center gap-1.5">
            <span>🦥</span> UNSLOTH FAST-RL
          </span>
          <span className="px-2.5 py-0.5 rounded-md text-[10px] mono font-bold bg-emerald-500/10 text-emerald-400 border border-emerald-500/30">
            ⚡ 5X FASTER · 80% LESS VRAM
          </span>
          <span className="px-2.5 py-0.5 rounded-md text-[10px] mono font-bold bg-[#1d1f2a] text-gray-300 border border-[#2d3040]">
            DUAL RTX 3090 (48GB) TP=2
          </span>
          <span className="px-2.5 py-0.5 rounded-md text-[10px] mono font-bold bg-[#1d1f2a] text-gray-300 border border-[#2d3040]">
            SGLANG :8080
          </span>
        </div>

        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4">
          <div>
            <h1 className="text-2xl md:text-3xl font-extrabold text-white tracking-tight mb-1 flex items-center gap-2.5">
              <span>oxide-agent-studio</span>
              <span className="text-xs font-mono font-normal px-2.5 py-0.5 rounded bg-orange-500/10 text-orange-400 border border-orange-500/20">
                v0.2.0
              </span>
            </h1>
            <p className="text-gray-400 text-xs md:text-sm max-w-3xl leading-relaxed">
              High-performance local agent studio built on Unsloth FastLanguageModel & GRPO RL fine-tuning, SGLang TP=2 serving, gRPC KiCad/FreeCAD bridge, Tree-Sitter RAG context compactor, and LoRA Model Soup blending.
            </p>
          </div>

          <div className="flex items-center gap-2.5 shrink-0">
            <div className="text-right hidden sm:block">
              <div className="text-[10px] mono uppercase text-gray-400 font-semibold">Fine-Tuning Engine</div>
              <div className="text-xs mono font-bold text-orange-400">Unsloth FastLanguageModel</div>
            </div>
            <div className="w-12 h-12 rounded-xl bg-orange-500/10 border border-orange-500/30 flex items-center justify-center text-2xl shadow-[0_0_15px_rgba(249,115,22,0.3)]">
              🦥
            </div>
          </div>
        </div>

        <div className="flex flex-wrap gap-2.5 mt-5 pt-4 border-t border-[#232530]">
          <button
            onClick={() => onNavigateTab('graph')}
            className="px-4 py-2 rounded-xl bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-gray-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-1.5 cursor-pointer shadow-[0_0_15px_rgba(249,115,22,0.35)]"
          >
            <Network className="w-3.5 h-3.5 fill-current" />
            Explore Projects Knowledge Graph
          </button>
          <button
            onClick={() => onNavigateTab('training')}
            className="px-4 py-2 rounded-xl bg-[#181a24] hover:bg-[#222432] text-gray-200 border border-[#2d3040] hover:border-orange-500/30 text-xs font-medium transition flex items-center gap-1.5 cursor-pointer"
          >
            <Zap className="w-3.5 h-3.5 text-orange-400" />
            Launch Unsloth GRPO (Phase 5)
          </button>
          <button
            onClick={() => onNavigateTab('grpc')}
            className="px-4 py-2 rounded-xl bg-[#181a24] hover:bg-[#222432] text-gray-200 border border-[#2d3040] hover:border-orange-500/30 text-xs font-medium transition flex items-center gap-1.5 cursor-pointer"
          >
            <Layers className="w-3.5 h-3.5 text-orange-400" />
            gRPC CAD Bridge (Phase 1)
          </button>
          <button
            onClick={() => onNavigateTab('soup')}
            className="px-4 py-2 rounded-xl bg-[#181a24] hover:bg-[#222432] text-gray-200 border border-[#2d3040] hover:border-orange-500/30 text-xs font-medium transition flex items-center gap-1.5 cursor-pointer"
          >
            <Sparkles className="w-3.5 h-3.5 text-amber-400" />
            LoRA Model Soup Blender
          </button>
        </div>
      </div>

      {/* Execution Plan Grid */}
      <div>
        <div className="flex items-center justify-between mb-3">
          <div>
            <h2 className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              Deep Execution Plan · 7 Phases
            </h2>
            <div className="text-[10px] mono text-gray-400">
              Pop!_OS 24.04 · Ryzen 9 9950X · 96GB DDR5 · Dual RTX 3090
            </div>
          </div>
          <span className="text-[11px] mono text-emerald-400 font-bold px-2.5 py-0.5 rounded-md bg-emerald-500/10 border border-emerald-500/30">
            Overall: 23.8% (Phase 1 active)
          </span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3">
          {phases.map((p) => {
            const isComplete = p.status === 'COMPLETE';
            const isInProgress = p.status === 'IN PROGRESS';
            const tabMap: Record<number, string> = {
              0: 'infra',
              1: 'grpc',
              2: 'rag',
              3: 'mcp',
              4: 'chat',
              5: 'training',
              6: 'catalog',
            };

            return (
              <div
                key={p.id}
                onClick={() => onNavigateTab(tabMap[p.id] || 'overview')}
                className="bg-[#111217] hover:bg-[#161822] border border-[#232530] hover:border-orange-500/40 rounded-xl p-4 transition-all duration-200 cursor-pointer flex flex-col justify-between shadow-sm"
              >
                <div>
                  <div className="flex items-center justify-between mb-2">
                    <span className="px-2 py-0.5 rounded text-[10px] mono font-bold bg-[#1a1c26] text-gray-300">
                      {p.tag}
                    </span>
                    <span
                      className={`text-[9px] mono px-2 py-0.5 rounded-full font-bold ${
                        isComplete
                          ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/30'
                          : isInProgress
                          ? 'bg-amber-500/10 text-amber-400 border border-amber-500/30'
                          : 'bg-[#181922] text-gray-500 border border-[#232530]'
                      }`}
                    >
                      {p.status}
                    </span>
                  </div>

                  <div className="text-xs font-bold text-white mb-1">{p.title}</div>
                  <div className="text-[11px] text-gray-400 leading-relaxed line-clamp-2">
                    {p.description}
                  </div>
                </div>

                <div className="mt-4 pt-3 border-t border-[#232530]">
                  <div className="h-1.5 w-full bg-[#181922] rounded-full overflow-hidden mb-1.5">
                    <div
                      className={`h-full rounded-full transition-all duration-500 ${
                        isComplete
                          ? 'bg-emerald-400 shadow-[0_0_8px_rgba(34,197,94,0.6)]'
                          : isInProgress
                          ? 'bg-gradient-to-r from-orange-500 to-amber-400 shadow-[0_0_8px_rgba(249,115,22,0.6)]'
                          : 'bg-gray-700'
                      }`}
                      style={{ width: `${p.progress}%` }}
                    />
                  </div>
                  <div className="flex items-center justify-between text-[9px] mono text-gray-400">
                    <span>{p.duration}</span>
                    <span>{p.tasksCount} tasks</span>
                  </div>
                </div>
              </div>
            );
          })}

          {/* Overall Aggregator Card */}
          <div className="bg-[#111217] border border-emerald-500/30 rounded-xl p-4 flex flex-col justify-between shadow-sm">
            <div>
              <div className="flex items-center justify-between mb-2">
                <span className="px-2 py-0.5 rounded text-[10px] mono font-bold bg-emerald-500/10 text-emerald-400">
                  Overall
                </span>
                <span className="text-[10px] mono font-bold text-emerald-400">23.8%</span>
              </div>
              <div className="text-xs font-bold text-white mb-1">Total Roadmap Progress</div>
              <div className="text-[11px] text-gray-400 leading-relaxed">
                7 phases · 17 total tasks · estimated ~18 engineering days
              </div>
            </div>
            <div className="mt-4 pt-3 border-t border-[#232530]">
              <div className="h-1.5 w-full bg-[#181922] rounded-full overflow-hidden mb-1.5">
                <div className="h-full bg-emerald-400 rounded-full shadow-[0_0_8px_rgba(34,197,94,0.6)]" style={{ width: '23.8%' }} />
              </div>
              <div className="text-[9px] mono text-emerald-400 flex items-center justify-between">
                <span>On Track</span>
                <span>Phase 1 active</span>
              </div>
            </div>
          </div>
        </div>
      </div>

      {/* Unsloth Telemetry & Hardware Gauges */}
      <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4">
          <div className="text-[9px] mono uppercase text-gray-400 font-semibold tracking-wider mb-1 flex items-center justify-between">
            <span>GPU 0 · RTX 3090</span>
            <span className="text-[8px] text-orange-400 mono">QDoRA 4-bit</span>
          </div>
          <div className="text-xl font-bold mono text-white flex items-baseline gap-1">
            {telemetry.gpu0} <span className="text-xs text-gray-400 font-normal">GB</span>
          </div>
          <div className="text-[10px] mono text-gray-400 mt-0.5">/ 24 GB · 76.6%</div>
          <div className="h-1.5 bg-[#181922] rounded-full mt-2 overflow-hidden">
            <div
              className="h-full bg-emerald-400 shadow-[0_0_6px_rgba(34,197,94,0.8)] transition-all duration-300"
              style={{ width: `${(telemetry.gpu0 / 24) * 100}%` }}
            />
          </div>
        </div>

        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4">
          <div className="text-[9px] mono uppercase text-gray-400 font-semibold tracking-wider mb-1 flex items-center justify-between">
            <span>GPU 1 · RTX 3090</span>
            <span className="text-[8px] text-orange-400 mono">TP=2</span>
          </div>
          <div className="text-xl font-bold mono text-white flex items-baseline gap-1">
            {telemetry.gpu1} <span className="text-xs text-gray-400 font-normal">GB</span>
          </div>
          <div className="text-[10px] mono text-gray-400 mt-0.5">/ 24 GB · 75.4%</div>
          <div className="h-1.5 bg-[#181922] rounded-full mt-2 overflow-hidden">
            <div
              className="h-full bg-emerald-400 shadow-[0_0_6px_rgba(34,197,94,0.8)] transition-all duration-300"
              style={{ width: `${(telemetry.gpu1 / 24) * 100}%` }}
            />
          </div>
        </div>

        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4">
          <div className="text-[9px] mono uppercase text-gray-400 font-semibold tracking-wider mb-1 flex items-center justify-between">
            <span>SGLang Cache Hit</span>
            <span className="text-[8px] text-emerald-400 mono">Radix Tree</span>
          </div>
          <div className="text-xl font-bold mono text-orange-400 flex items-baseline gap-1">
            {telemetry.cacheHit} <span className="text-xs text-gray-400 font-normal">%</span>
          </div>
          <div className="text-[10px] mono text-gray-400 mt-0.5">Prefix Cache LRU</div>
          <div className="h-1.5 bg-[#181922] rounded-full mt-2 overflow-hidden">
            <div
              className="h-full bg-orange-400 shadow-[0_0_6px_rgba(249,115,22,0.8)] transition-all duration-300"
              style={{ width: `${telemetry.cacheHit}%` }}
            />
          </div>
        </div>

        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4">
          <div className="text-[9px] mono uppercase text-gray-400 font-semibold tracking-wider mb-1 flex items-center justify-between">
            <span>Unsloth Speedup</span>
            <span className="text-[8px] text-emerald-400 mono">RLVR</span>
          </div>
          <div className="text-xl font-bold mono text-emerald-400 flex items-baseline gap-1">
            5.2x <span className="text-xs text-gray-400 font-normal">faster</span>
          </div>
          <div className="text-[10px] mono text-gray-400 mt-0.5">80% VRAM Reduction</div>
          <div className="h-1.5 bg-[#181922] rounded-full mt-2 overflow-hidden">
            <div className="h-full bg-emerald-400 shadow-[0_0_6px_rgba(34,197,94,0.8)]" style={{ width: '85%' }} />
          </div>
        </div>
      </div>

      {/* Latency & Throughput Visualizers */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Latency Chart */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4">
          <div className="flex items-center justify-between mb-2">
            <div>
              <div className="text-xs font-bold text-white uppercase tracking-wider">Inference Latency (TTFT)</div>
              <div className="text-[10px] mono text-gray-400">P50: 132ms · P90: 144ms · P99: 158ms</div>
            </div>
            <span className="text-xs mono text-orange-400 font-bold">~138 ms</span>
          </div>

          <div className="h-32 flex items-end gap-1.5 pt-3">
            {latencyHistory.map((val, idx) => {
              const heightPct = Math.min(100, Math.max(15, ((val - 100) / 70) * 100));
              return (
                <div key={idx} className="flex-1 flex flex-col items-center gap-1">
                  <div
                    className="w-full bg-orange-500/70 hover:bg-orange-400 rounded-t-sm transition-all shadow-[0_0_8px_rgba(249,115,22,0.3)]"
                    style={{ height: `${heightPct}%` }}
                    title={`${val} ms`}
                  />
                  <span className="text-[8px] mono text-gray-500">{idx + 1}</span>
                </div>
              );
            })}
          </div>
        </div>

        {/* Mojo SIMD Throughput */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4">
          <div className="flex items-center justify-between mb-2">
            <div>
              <div className="text-xs font-bold text-white uppercase tracking-wider">Mojo SIMD Throughput</div>
              <div className="text-[10px] mono text-gray-400">Cosine Distance · 512-bit AVX-512</div>
            </div>
            <span className="text-xs mono text-emerald-400 font-bold">{telemetry.simd} MB/s</span>
          </div>

          <div className="h-32 flex items-end gap-1.5 pt-3">
            {simdHistory.map((val, idx) => {
              const heightPct = Math.min(100, Math.max(15, ((val - 700) / 200) * 100));
              return (
                <div key={idx} className="flex-1 flex flex-col items-center gap-1">
                  <div
                    className="w-full bg-emerald-500/70 hover:bg-emerald-400 rounded-t-sm transition-all shadow-[0_0_8px_rgba(34,197,94,0.3)]"
                    style={{ height: `${heightPct}%` }}
                    title={`${val} MB/s`}
                  />
                  <span className="text-[8px] mono text-gray-500">{idx + 1}</span>
                </div>
              );
            })}
          </div>
        </div>
      </div>

      {/* System Activity Stream */}
      <div className="bg-[#111217] border border-[#232530] rounded-xl overflow-hidden shadow-sm">
        <div className="px-4 py-3 border-b border-[#232530] flex items-center justify-between bg-[#0e0f14]">
          <div className="flex items-center gap-2">
            <Activity className="w-4 h-4 text-orange-400" />
            <span className="text-xs font-bold text-white uppercase tracking-wider">System Activity Stream</span>
            <span className="text-[10px] mono text-gray-400">Real-time Telemetry</span>
          </div>
          <button
            onClick={() => setLogs([])}
            className="text-[10px] mono text-gray-400 hover:text-white px-2 py-0.5 rounded bg-[#181922] border border-[#232530] transition cursor-pointer"
          >
            Clear
          </button>
        </div>

        <div className="p-3.5 space-y-1.5 max-h-52 overflow-y-auto font-mono text-[11px]">
          {logs.map((log) => {
            const borderColors = {
              ok: 'border-l-emerald-500 text-emerald-200 bg-emerald-950/20',
              info: 'border-l-orange-500 text-orange-200 bg-orange-950/20',
              warn: 'border-l-amber-500 text-amber-200 bg-amber-950/20',
              err: 'border-l-rose-500 text-rose-200 bg-rose-950/20',
            };

            return (
              <div
                key={log.id}
                className={`border-l-2 pl-3 py-1 flex items-start gap-2 rounded-r-md ${
                  borderColors[log.level]
                }`}
              >
                <span className="text-gray-500 shrink-0">[{log.time}]</span>
                <span className="text-gray-300 shrink-0 font-semibold">{log.subsystem}:</span>
                <span className="text-gray-200 flex-1">{log.text}</span>
              </div>
            );
          })}
        </div>
      </div>
    </div>
  );
};
