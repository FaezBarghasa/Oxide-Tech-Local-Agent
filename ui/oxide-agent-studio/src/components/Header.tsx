import React from 'react';
import { TabId } from '../types';
import { Play, Plus, Zap, Boxes, Database, Flame, Layers, Search, Wrench, Cpu, Server, Code2, CheckCircle2 } from 'lucide-react';

interface HeaderProps {
  currentTab: TabId;
  onSelectTab: (tab: TabId) => void;
  onNewSession: () => void;
  onQuickDeploy: () => void;
}

const tabInfo: Record<TabId, { title: string; subtitle: string; category: string }> = {
  chat: { title: 'AI Assistant', subtitle: 'Native agent with STAIR Code-ToC AST retrieval', category: 'Chat' },
  overview: { title: 'Workspace Overview', subtitle: 'Verified crates, desktop architecture & system telemetry', category: 'Overview' },
  graph: { title: 'Knowledge Graph', subtitle: 'Interactive relational topology of hardware & firmware crates', category: 'Projects' },
  memory: { title: 'Project Memory', subtitle: 'oxide-embed semantic memory: search, recall, remember & context packs', category: 'Projects' },
  training: { title: 'Unsloth Tuning', subtitle: 'FastLanguageModel fine-tuning with verifiable compiler rewards', category: 'Tuning' },
  catalog: { title: 'Model Catalog & VRAM', subtitle: 'Model memory profiling & 4-bit / 16-bit fit calculator', category: 'Tuning' },
  dataset: { title: 'Dataset Recipes', subtitle: 'Visual recipe studio for synthetic multi-source data', category: 'Tuning' },
  soup: { title: 'LoRA Model Soup', subtitle: 'Task arithmetic weight blending & zero-loss export', category: 'Tuning' },
  grpc: { title: 'KiCad & DRC Bridge', subtitle: 'Protobuf gRPC schematics, netlists & design rule verification', category: 'CAD' },
  rag: { title: 'AST Token Compactor', subtitle: 'Tree-Sitter AST scope pruner saving up to 78% tokens', category: 'CAD' },
  mcp: { title: 'MCP Sandbox & Tools', subtitle: 'STDIO JSON-RPC 2.0 tool execution in safe sandbox', category: 'CAD' },
  sglang: { title: 'SGLang Serving Engine', subtitle: 'Inference runtime with RadixAttention prefix caching', category: 'Serving' },
  infra: { title: 'System Daemons', subtitle: 'SurrealDB, Qdrant & native LLVM toolchain health', category: 'Serving' },
  endpoints: { title: 'API Specifications', subtitle: 'Interactive API runner for Trainer, Runner & Nexus endpoints', category: 'Serving' },
  verify: { title: 'Verification Matrix', subtitle: 'End-to-end subsystem latency & precision tests', category: 'Serving' },
  doctor: { title: 'System Diagnostics & Doctor', subtitle: 'Deterministic host environment, compiler & GPU checks', category: 'System' },
  reforge: { title: 'RE-Forge Studio', subtitle: 'Binary disassembly, ARM IVT parsing, PTX GPU analysis & safe Rust decompiler', category: 'Reverse Engineering' },
  settings: { title: 'Settings & Profiles', subtitle: 'Operating profiles, hardware limits & config.toml editor', category: 'System' },
};

export const Header: React.FC<HeaderProps> = ({ currentTab, onSelectTab, onNewSession, onQuickDeploy }) => {
  const current = tabInfo[currentTab] || { title: 'Oxide Agent Studio', subtitle: '', category: 'Workspace' };

  // Sub-navigation configurations
  const tuningTabs = [
    { id: 'training' as TabId, label: 'GRPO Trainer', icon: Zap },
    { id: 'catalog' as TabId, label: 'Models & VRAM', icon: Boxes },
    { id: 'dataset' as TabId, label: 'Dataset Recipes', icon: Database },
    { id: 'soup' as TabId, label: 'LoRA Soup', icon: Flame },
  ];

  const cadTabs = [
    { id: 'grpc' as TabId, label: 'KiCad & DRC', icon: Layers },
    { id: 'rag' as TabId, label: 'AST Compactor', icon: Search },
    { id: 'mcp' as TabId, label: 'MCP Sandbox', icon: Wrench },
  ];

  const servingTabs = [
    { id: 'sglang' as TabId, label: 'SGLang Serving', icon: Cpu },
    { id: 'infra' as TabId, label: 'Daemons', icon: Server },
    { id: 'endpoints' as TabId, label: 'API Specs', icon: Code2 },
    { id: 'verify' as TabId, label: 'Verification', icon: CheckCircle2 },
  ];

  const getSubTabs = () => {
    if (['training', 'catalog', 'dataset', 'soup'].includes(currentTab)) return tuningTabs;
    if (['grpc', 'rag', 'mcp'].includes(currentTab)) return cadTabs;
    if (['sglang', 'infra', 'endpoints', 'verify'].includes(currentTab)) return servingTabs;
    return null;
  };

  const activeSubTabs = getSubTabs();

  return (
    <header className="sticky top-0 z-20 bg-[#09090b]/95 backdrop-blur-xl border-b border-white/[0.07] px-4 md:px-6 py-2.5 flex flex-col gap-2">
      <div className="flex items-center justify-between gap-4">
        {/* Title & Subtitle */}
        <div className="min-w-0">
          <div className="flex items-center gap-2">
            <span className="text-[10px] font-mono uppercase font-bold text-amber-400 px-1.5 py-0.5 rounded bg-amber-500/10 border border-amber-500/20">
              {current.category}
            </span>
            <h1 className="text-sm md:text-base font-bold text-white tracking-tight truncate">
              {current.title}
            </h1>
            <span className="text-xs text-zinc-400 hidden xl:inline truncate font-normal">
              — {current.subtitle}
            </span>
          </div>
        </div>

        {/* Action Controls */}
        <div className="flex items-center gap-2.5 shrink-0">
          <button
            onClick={onNewSession}
            className="px-3 py-1.5 rounded-lg bg-[#141418] hover:bg-[#1c1c22] text-zinc-200 border border-white/[0.08] hover:border-amber-500/40 text-xs font-medium transition flex items-center gap-1.5 cursor-pointer"
            title="Start new chat session"
          >
            <Plus className="w-3.5 h-3.5 text-amber-400" />
            <span className="hidden sm:inline">New Session</span>
          </button>

          <button
            onClick={onQuickDeploy}
            className="px-3.5 py-1.5 rounded-lg bg-gradient-to-r from-amber-500 to-amber-600 hover:from-amber-400 hover:to-amber-500 text-zinc-950 text-xs font-bold tracking-wider uppercase transition-all flex items-center gap-1.5 shadow-[0_0_15px_rgba(245,158,11,0.25)] cursor-pointer"
          >
            <Play className="w-3 h-3 fill-current" />
            <span>Deploy</span>
          </button>
        </div>
      </div>

      {/* Sub-Navigation Bar if in multi-tab category */}
      {activeSubTabs && (
        <div className="flex items-center gap-1.5 overflow-x-auto pt-1 pb-0.5 border-t border-white/[0.05]">
          <span className="text-[9px] font-mono text-zinc-500 uppercase font-semibold mr-1 shrink-0">
            Sub-view:
          </span>
          {activeSubTabs.map((tab) => {
            const Icon = tab.icon;
            const isActive = currentTab === tab.id;
            return (
              <button
                key={tab.id}
                onClick={() => onSelectTab(tab.id)}
                className={`px-2.5 py-1 rounded-md text-xs font-medium transition flex items-center gap-1.5 shrink-0 cursor-pointer ${
                  isActive
                    ? 'bg-amber-500/15 border border-amber-500/40 text-amber-300 font-semibold'
                    : 'text-zinc-400 hover:text-zinc-200 hover:bg-[#141418]'
                }`}
              >
                <Icon className={`w-3 h-3 ${isActive ? 'text-amber-400' : 'text-zinc-500'}`} />
                <span>{tab.label}</span>
              </button>
            );
          })}
        </div>
      )}
    </header>
  );
};

