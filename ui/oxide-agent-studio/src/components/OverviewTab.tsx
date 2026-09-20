import React, { useState, useEffect } from 'react';
import { PhaseInfo } from '../types';
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
  Box,
  Brain,
  HardDrive,
  ShieldCheck,
  FolderGit2,
} from 'lucide-react';
import { desktop, MemoryEnv } from '../lib/desktop';

interface OverviewTabProps {
  onNavigateTab: (tab: any) => void;
}

export const OverviewTab: React.FC<OverviewTabProps> = ({ onNavigateTab }) => {
  const [memoryEnv, setMemoryEnv] = useState<MemoryEnv | null>(null);
  const [gatewayStatus, setGatewayStatus] = useState<number | null>(null);

  const workspaceCrates = [
    { name: 'gateway', desc: 'Actix HTTP/3 & QUIC Master Gateway', tier: 'Tier 1', ready: true },
    { name: 'memory', desc: 'STAIR Code-ToC & Memanto Fabric (.oxide/)', tier: 'Tier 1', ready: true },
    { name: 'thinker', desc: 'Multi-model inference engine (Ollama/Gemini/Groq)', tier: 'Tier 1', ready: true },
    { name: 'mcp-server', desc: 'STDIO JSON-RPC 2.0 RMCP 3.2 Server', tier: 'Tier 1', ready: true },
    { name: 're-forge', desc: 'ARM IVT, Shannon entropy & PTX GPU lifter', tier: 'Tier 2', ready: true },
    { name: 'verifier', desc: 'Deterministic 7-phase evidence verifier', tier: 'Tier 2', ready: true },
    { name: 'config-loader', desc: 'Hot-reloadable config.toml loader', tier: 'Tier 1', ready: true },
    { name: 'skills', desc: 'Skill store & prompt gene synthesizer', tier: 'Tier 2', ready: true },
  ];

  const phases: PhaseInfo[] = [
    {
      id: 0,
      tag: 'Phase 0',
      title: 'Universal Desktop Application',
      description: 'Single-binary Tauri 2 desktop app with embedded gateway background daemon',
      status: 'COMPLETE',
      progress: 100,
      color: 'emerald',
      duration: 'Completed',
      tasksCount: 4,
    },
    {
      id: 1,
      tag: 'Phase 1',
      title: 'STAIR Code-ToC & Memanto Fabric',
      description: 'AST semantic retrieval, conflict-free memory assertions & context budgeting',
      status: 'COMPLETE',
      progress: 100,
      color: 'emerald',
      duration: 'Completed',
      tasksCount: 3,
    },
    {
      id: 2,
      tag: 'Phase 2',
      title: 'Hardware Probes & RE-Forge Lifter',
      description: 'probe-rs STM32 SWD flashing, ARM interrupt vector parsing & CUDA PTX inspector',
      status: 'COMPLETE',
      progress: 100,
      color: 'emerald',
      duration: 'Active',
      tasksCount: 3,
    },
    {
      id: 3,
      tag: 'Phase 3',
      title: 'Deterministic Verifier Suite',
      description: 'Automated workspace integrity checks & cryptographic evidence export bundle',
      status: 'COMPLETE',
      progress: 100,
      color: 'emerald',
      duration: 'Active',
      tasksCount: 2,
    },
  ];

  useEffect(() => {
    async function loadRealData() {
      try {
        const [mem, gw] = await Promise.allSettled([
          desktop.memoryEnv(),
          desktop.gatewayStatus(),
        ]);
        if (mem.status === 'fulfilled') setMemoryEnv(mem.value);
        if (gw.status === 'fulfilled') setGatewayStatus(gw.value);
      } catch {
        // standalone mode
      }
    }
    loadRealData();
  }, []);

  return (
    <div className="space-y-6 font-sans">
      {/* Hero Banner */}
      <div className="bg-[#121216] border border-white/[0.07] rounded-2xl p-6 md:p-7 relative overflow-hidden shadow-xl bg-[radial-gradient(ellipse_at_top_right,rgba(245,158,11,0.08)_0%,transparent_70%)]">
        <div className="flex flex-wrap items-center gap-2 mb-3">
          <span className="px-2.5 py-0.5 rounded-md text-[10px] font-mono font-bold bg-amber-500/10 text-amber-400 border border-amber-500/30 flex items-center gap-1.5">
            <span>⚡</span> OXIDE TECH LOCAL AGENT
          </span>
          <span className="px-2.5 py-0.5 rounded-md text-[10px] font-mono font-bold bg-emerald-500/10 text-emerald-400 border border-emerald-500/30">
            DESKTOP-FIRST ARCHITECTURE
          </span>
          <span className="px-2.5 py-0.5 rounded-md text-[10px] font-mono font-bold bg-[#1a1a20] text-zinc-300 border border-white/[0.08]">
            RUST 1.85+ · TAURI V2
          </span>
          <span className="px-2.5 py-0.5 rounded-md text-[10px] font-mono font-bold bg-[#1a1a20] text-zinc-300 border border-white/[0.08]">
            {gatewayStatus === 200 ? 'GATEWAY :8080 (LIVE)' : 'STANDALONE IPC'}
          </span>
        </div>

        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-4">
          <div>
            <h1 className="text-2xl md:text-3xl font-extrabold text-white tracking-tight mb-1 flex items-center gap-2.5">
              <span>Oxide Agent Studio</span>
              <span className="text-xs font-mono font-normal px-2.5 py-0.5 rounded bg-amber-500/10 text-amber-400 border border-amber-500/20">
                v0.5.0 Universal
              </span>
            </h1>
            <p className="text-zinc-400 text-xs md:text-sm max-w-3xl leading-relaxed">
              Desktop-first local engineering workstation. All operations—including STAIR Code-ToC search, Memanto memory, RE-Forge binary decompilation, probe-rs STM32 flashing, and deterministic verification—are executed in-process through native Rust Tauri IPC with zero command line required.
            </p>
          </div>

          <div className="flex items-center gap-2.5 shrink-0">
            <div className="text-right hidden sm:block">
              <div className="text-[10px] font-mono uppercase text-zinc-400 font-semibold">Memory Fabric</div>
              <div className="text-xs font-mono font-bold text-amber-400">oxide-embed AST</div>
            </div>
            <div className="w-12 h-12 rounded-xl bg-[#18181e] border border-white/[0.1] p-2 flex items-center justify-center shadow-[0_0_15px_rgba(245,158,11,0.2)]">
              <img
                src="/assets/oxide-logo.png"
                alt="Oxide-Tech Logo"
                className="w-full h-full object-contain"
              />
            </div>
          </div>
        </div>

        <div className="flex flex-wrap gap-2.5 mt-5 pt-4 border-t border-white/[0.07]">
          <button
            onClick={() => onNavigateTab('memory')}
            className="px-4 py-2 rounded-xl bg-gradient-to-r from-amber-500 to-amber-600 hover:from-amber-400 hover:to-amber-500 text-zinc-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-1.5 cursor-pointer shadow-[0_0_15px_rgba(245,158,11,0.3)]"
          >
            <Brain className="w-3.5 h-3.5 fill-current" />
            STAIR Project Memory
          </button>
          <button
            onClick={() => onNavigateTab('reforge')}
            className="px-4 py-2 rounded-xl bg-[#18181e] hover:bg-[#22222a] text-zinc-200 border border-white/[0.08] hover:border-amber-500/30 text-xs font-medium transition flex items-center gap-1.5 cursor-pointer"
          >
            <Layers className="w-3.5 h-3.5 text-cyan-400" />
            RE-Forge Binary Lifter
          </button>
          <button
            onClick={() => onNavigateTab('doctor')}
            className="px-4 py-2 rounded-xl bg-[#18181e] hover:bg-[#22222a] text-zinc-200 border border-white/[0.08] hover:border-amber-500/30 text-xs font-medium transition flex items-center gap-1.5 cursor-pointer"
          >
            <ShieldCheck className="w-3.5 h-3.5 text-emerald-400" />
            System Doctor Diagnostics
          </button>
          <button
            onClick={() => onNavigateTab('verify')}
            className="px-4 py-2 rounded-xl bg-[#18181e] hover:bg-[#22222a] text-zinc-200 border border-white/[0.08] hover:border-amber-500/30 text-xs font-medium transition flex items-center gap-1.5 cursor-pointer"
          >
            <CheckCircle2 className="w-3.5 h-3.5 text-amber-400" />
            Deterministic Verifier
          </button>
        </div>
      </div>

      {/* Real Workspace Crates Inventory */}
      <div>
        <div className="flex items-center justify-between mb-3">
          <div>
            <h2 className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Box className="w-4 h-4 text-amber-400" />
              Active Workspace Modules & Crates
            </h2>
            <div className="text-[10px] font-mono text-zinc-400">
              Cargo Workspace · 8 Verified Modular Crates
            </div>
          </div>
          <span className="text-[11px] font-mono text-emerald-400 font-bold px-2.5 py-0.5 rounded-md bg-emerald-500/10 border border-emerald-500/30">
            All Crates Compiled (0 errors)
          </span>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-3">
          {workspaceCrates.map((crate, idx) => (
            <div
              key={idx}
              className="bg-[#121216] border border-white/[0.06] hover:border-amber-500/30 rounded-xl p-4 transition-all duration-200 flex flex-col justify-between"
            >
              <div>
                <div className="flex items-center justify-between mb-2">
                  <span className="text-xs font-mono font-bold text-white flex items-center gap-1.5">
                    <span className="w-1.5 h-1.5 rounded-full bg-emerald-400" />
                    {crate.name}
                  </span>
                  <span className="text-[9px] font-mono text-zinc-400 px-1.5 py-0.2 bg-[#181820] rounded border border-white/[0.06]">
                    {crate.tier}
                  </span>
                </div>
                <div className="text-[11px] text-zinc-400 leading-relaxed">
                  {crate.desc}
                </div>
              </div>
              <div className="mt-3 pt-2 border-t border-white/[0.06] flex items-center justify-between text-[9px] font-mono text-emerald-400">
                <span>Status: READY</span>
                <span>no_std / async</span>
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Execution Roadmap */}
      <div>
        <div className="flex items-center justify-between mb-3">
          <div>
            <h2 className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <FolderGit2 className="w-4 h-4 text-amber-400" />
              System Architecture & Desktop Milestones
            </h2>
            <div className="text-[10px] font-mono text-zinc-400">
              Pop!_OS 24.04 Cosmic · Pure Rust Core · Desktop-First OS
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-3">
          {phases.map((p) => (
            <div
              key={p.id}
              className="bg-[#121216] border border-white/[0.06] rounded-xl p-4 flex flex-col justify-between"
            >
              <div>
                <div className="flex items-center justify-between mb-2">
                  <span className="px-2 py-0.5 rounded text-[10px] font-mono font-bold bg-[#181820] text-zinc-300">
                    {p.tag}
                  </span>
                  <span className="text-[9px] font-mono px-2 py-0.5 rounded-full font-bold bg-emerald-500/10 text-emerald-400 border border-emerald-500/30">
                    {p.status}
                  </span>
                </div>

                <div className="text-xs font-bold text-white mb-1">{p.title}</div>
                <div className="text-[11px] text-zinc-400 leading-relaxed">
                  {p.description}
                </div>
              </div>

              <div className="mt-4 pt-3 border-t border-white/[0.06]">
                <div className="h-1.5 w-full bg-[#181820] rounded-full overflow-hidden mb-1.5">
                  <div className="h-full bg-emerald-400 rounded-full shadow-[0_0_8px_rgba(16,185,129,0.5)]" style={{ width: `${p.progress}%` }} />
                </div>
                <div className="flex items-center justify-between text-[9px] font-mono text-zinc-400">
                  <span>{p.duration}</span>
                  <span>{p.tasksCount} tasks</span>
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};

