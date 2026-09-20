import React from 'react';
import { TabId } from '../types';
import {
  MessageSquare,
  LayoutDashboard,
  Network,
  Brain,
  Zap,
  Layers,
  Cpu,
  Boxes,
  Database,
  Flame,
  Search,
  Wrench,
  Server,
  Code2,
  CheckCircle2,
  Stethoscope,
  Binary,
  Settings,
} from 'lucide-react';

interface SidebarProps {
  currentTab: TabId;
  onSelectTab: (tab: TabId) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ currentTab, onSelectTab }) => {
  const isTrainingSection = ['training', 'catalog', 'dataset', 'soup'].includes(currentTab);
  const isCadSection = ['grpc', 'rag', 'mcp'].includes(currentTab);
  const isServingSection = ['sglang', 'infra', 'endpoints', 'verify'].includes(currentTab);

  const mainNavigation = [
    {
      id: 'chat' as TabId,
      label: 'AI Assistant',
      desc: 'Native engineering chat & code',
      icon: MessageSquare,
      active: currentTab === 'chat',
    },
    {
      id: 'overview' as TabId,
      label: 'Overview & Crates',
      desc: 'Workspace crates & architecture',
      icon: LayoutDashboard,
      active: currentTab === 'overview',
    },
    {
      id: 'doctor' as TabId,
      label: 'Doctor & System',
      desc: 'Toolchains, probes & GPU checks',
      icon: Stethoscope,
      active: currentTab === 'doctor',
      badge: 'Diagnostics',
      badgeColor: 'text-emerald-400 bg-emerald-500/10 border-emerald-500/30',
    },
    {
      id: 'reforge' as TabId,
      label: 'RE-Forge Studio',
      desc: 'Binary, ARM IVT & GPU PTX lifter',
      icon: Binary,
      active: currentTab === 'reforge',
      badge: 'Disasm',
      badgeColor: 'text-cyan-400 bg-cyan-500/10 border-cyan-500/30',
    },
    {
      id: 'graph' as TabId,
      label: 'Graph Topology',
      desc: 'Relational crate & node map',
      icon: Network,
      active: currentTab === 'graph',
    },
    {
      id: 'memory' as TabId,
      label: 'Project Memory',
      desc: 'oxide-embed STAIR & Memanto',
      icon: Brain,
      active: currentTab === 'memory',
      badge: '.oxide',
      badgeColor: 'text-amber-400 bg-amber-500/10 border-amber-500/30',
    },
    {
      id: 'training' as TabId,
      label: 'Unsloth Tuning',
      desc: 'GRPO, QDoRA & Model Soup',
      icon: Zap,
      active: isTrainingSection,
      subItems: [
        { id: 'training' as TabId, label: 'GRPO Trainer', icon: Zap },
        { id: 'catalog' as TabId, label: 'Model Catalog', icon: Boxes },
        { id: 'dataset' as TabId, label: 'Dataset Recipes', icon: Database },
        { id: 'soup' as TabId, label: 'LoRA Model Soup', icon: Flame },
      ],
    },
    {
      id: 'grpc' as TabId,
      label: 'CAD & Schematics',
      desc: 'KiCad netlists, DRC & AST',
      icon: Layers,
      active: isCadSection,
      subItems: [
        { id: 'grpc' as TabId, label: 'KiCad Bridge', icon: Layers },
        { id: 'rag' as TabId, label: 'AST Compactor', icon: Search },
        { id: 'mcp' as TabId, label: 'MCP Sandbox', icon: Wrench },
      ],
    },
    {
      id: 'sglang' as TabId,
      label: 'Serving & System',
      desc: 'Inference runtime & verification',
      icon: Cpu,
      active: isServingSection,
      subItems: [
        { id: 'sglang' as TabId, label: 'SGLang Engine', icon: Cpu },
        { id: 'infra' as TabId, label: 'System Daemons', icon: Server },
        { id: 'endpoints' as TabId, label: 'API Specs', icon: Code2 },
        { id: 'verify' as TabId, label: 'Verifier Matrix', icon: CheckCircle2 },
      ],
    },
    {
      id: 'settings' as TabId,
      label: 'Settings & Config',
      desc: 'Profiles, limits & config.toml',
      icon: Settings,
      active: currentTab === 'settings',
    },
  ];

  return (
    <aside className="w-64 bg-[#09090b]/95 backdrop-blur-2xl border-r border-white/[0.07] flex flex-col h-screen sticky top-0 shrink-0 z-30 font-sans">
      {/* Brand Header */}
      <div className="p-4 border-b border-white/[0.07] flex items-center justify-between bg-[#121216]">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-amber-500 to-amber-600 flex items-center justify-center font-bold text-zinc-950 text-base shadow-[0_0_12px_rgba(245,158,11,0.3)]">
            🦀
          </div>
          <div>
            <div className="text-xs font-bold text-white tracking-tight flex items-center gap-1.5">
              Oxide-Tech Agent
            </div>
            <div className="text-[10px] text-zinc-400 font-mono">
              Desktop-First OS
            </div>
          </div>
        </div>
        <span className="text-[9px] font-mono px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 font-semibold">
          DESKTOP
        </span>
      </div>

      {/* Main Navigation */}
      <div className="flex-1 overflow-y-auto p-3 space-y-1">
        <div className="text-[9px] font-mono uppercase font-bold text-zinc-400 tracking-[0.2em] px-2 mb-2">
          Workspaces
        </div>

        {mainNavigation.map((item) => {
          const Icon = item.icon;
          const isActive = item.active;

          return (
            <div key={item.id} className="space-y-0.5">
              <button
                onClick={() => onSelectTab(item.id)}
                className={`w-full flex items-center gap-3 px-3 py-2 rounded-xl text-xs font-medium transition-all text-left relative cursor-pointer ${
                  isActive
                    ? 'bg-amber-500/10 border border-amber-500/40 text-white font-semibold shadow-[0_0_12px_rgba(245,158,11,0.12)]'
                    : 'text-zinc-400 hover:text-zinc-200 hover:bg-[#141418]'
                }`}
              >
                {isActive && (
                  <span className="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-5 bg-gradient-to-b from-amber-400 to-amber-500 rounded-r-full shadow-[0_0_8px_rgba(245,158,11,0.6)]" />
                )}
                <div
                  className={`w-7 h-7 rounded-lg flex items-center justify-center shrink-0 ${
                    isActive
                      ? 'bg-amber-500/20 text-amber-400'
                      : 'bg-[#18181e] text-zinc-400'
                  }`}
                >
                  <Icon className="w-3.5 h-3.5" />
                </div>
                <div className="min-w-0 flex-1">
                  <div className="flex items-center justify-between">
                    <span className="truncate">{item.label}</span>
                    {item.badge && (
                      <span
                        className={`text-[8px] font-mono font-bold px-1.5 py-0.2 rounded border ${
                          item.badgeColor || 'text-emerald-400 bg-emerald-500/10 border-emerald-500/20'
                        }`}
                      >
                        {item.badge}
                      </span>
                    )}
                  </div>
                  <div className="text-[10px] text-zinc-400 truncate mt-0.5">
                    {item.desc}
                  </div>
                </div>
              </button>

              {/* Collapsible Sub-items */}
              {isActive && item.subItems && (
                <div className="pl-6 pr-1 py-1 space-y-0.5 border-l border-amber-500/20 ml-4">
                  {item.subItems.map((sub) => {
                    const SubIcon = sub.icon;
                    const isSubActive = currentTab === sub.id;
                    return (
                      <button
                        key={sub.id}
                        onClick={() => onSelectTab(sub.id)}
                        className={`w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-[11px] font-medium transition text-left cursor-pointer ${
                          isSubActive
                            ? 'text-amber-300 font-semibold bg-amber-500/10 border border-amber-500/20'
                            : 'text-zinc-400 hover:text-zinc-200 hover:bg-[#141418]'
                        }`}
                      >
                        <SubIcon className={`w-3 h-3 ${isSubActive ? 'text-amber-400' : 'text-zinc-400'}`} />
                        <span className="truncate">{sub.label}</span>
                      </button>
                    );
                  })}
                </div>
              )}
            </div>
          );
        })}
      </div>

      {/* Clean Cluster Status Footer */}
      <div className="p-3 border-t border-white/[0.07] bg-[#121216]">
        <div className="flex items-center justify-between text-[10px] font-mono text-zinc-300">
          <div className="flex items-center gap-2">
            <span className="w-2 h-2 rounded-full bg-emerald-400 shadow-[0_0_6px_rgba(16,185,129,0.6)] animate-pulse" />
            <span className="font-semibold text-white">Local Workstation</span>
          </div>
          <span className="text-amber-400 font-bold">Pop!_OS</span>
        </div>
        <div className="mt-1 flex items-center justify-between text-[9px] font-mono text-zinc-400">
          <span>Rust 1.85+</span>
          <span className="text-emerald-400">Tauri v2</span>
          <span>In-Process</span>
        </div>
      </div>
    </aside>
  );
};

