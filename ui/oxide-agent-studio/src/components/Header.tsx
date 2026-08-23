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
  chat: { title: 'AI Assistant', subtitle: 'Fast local agent with tools & firmware synthesis', category: 'Chat' },
  overview: { title: 'Overview & Plan', subtitle: '7-phase deep plan & dual RTX 3090 telemetry', category: 'Overview' },
  graph: { title: 'Knowledge Graph', subtitle: 'Interactive relational topology of all hardware & firmware projects', category: 'Projects' },
  training: { title: 'Unsloth GRPO RLVR', subtitle: '5x faster reinforcement learning with verifiable compiler rewards', category: 'Tuning' },
  catalog: { title: 'Model Catalog & VRAM', subtitle: 'Model memory profiling & 4-bit / 16-bit fit calculator', category: 'Tuning' },
  dataset: { title: 'Dataset Recipes', subtitle: 'Visual recipe studio for synthetic multi-source data', category: 'Tuning' },
  soup: { title: 'LoRA Model Soup', subtitle: 'Task arithmetic weight blending & zero-loss export', category: 'Tuning' },
  grpc: { title: 'KiCad & DRC Bridge', subtitle: 'Protobuf gRPC schematics, netlists & design rule verification', category: 'CAD' },
  rag: { title: 'AST Token Compactor', subtitle: 'Tree-Sitter AST scope pruner saving up to 78% tokens', category: 'CAD' },
  mcp: { title: 'MCP Sandbox & Tools', subtitle: 'STDIO JSON-RPC 2.0 tool execution in safe sandbox', category: 'CAD' },
  sglang: { title: 'SGLang Serving Engine', subtitle: 'TP=2 dual RTX 3090 cluster with RadixAttention prefix caching', category: 'Serving' },
  infra: { title: 'System Daemons', subtitle: 'SurrealDB, Qdrant & native LLVM toolchain health', category: 'Serving' },
  endpoints: { title: 'API Specifications', subtitle: 'Interactive API runner for Trainer, Runner & Nexus endpoints', category: 'Serving' },
  verify: { title: 'Verification Matrix', subtitle: 'End-to-end subsystem latency & precision tests', category: 'Serving' },
};

export const Header: React.FC<HeaderProps> = ({ currentTab, onSelectTab, onNewSession, onQuickDeploy }) => {
  const current = tabInfo[currentTab] || { title: 'oxide-agent-studio', subtitle: '', category: 'Workspace' };

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
    { id: 'sglang' as TabId, label: 'SGLang TP=2', icon: Cpu },
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
    <header className="sticky top-0 z-20 bg-[#0c0d12]/95 backdrop-blur-xl border-b border-[#232530] px-4 md:px-6 py-2.5 flex flex-col gap-2">
      <div className="flex items-center justify-between gap-4">
        {/* Title & Subtitle */}
        <div className="min-w-0">
          <div className="flex items-center gap-2">
            <span className="text-[10px] mono uppercase font-bold text-orange-400 px-1.5 py-0.5 rounded bg-orange-500/10 border border-orange-500/20">
              {current.category}
            </span>
            <h1 className="text-sm md:text-base font-bold text-white tracking-tight truncate">
              {current.title}
            </h1>
            <span className="text-xs text-gray-400 hidden xl:inline truncate font-normal">
              — {current.subtitle}
            </span>
          </div>
        </div>

        {/* Action Controls */}
        <div className="flex items-center gap-2.5 shrink-0">
          <div className="hidden sm:flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-[#14151e] border border-[#232530] text-[10px] mono text-gray-300">
            <span className="w-1.5 h-1.5 rounded-full bg-orange-400 shadow-[0_0_6px_rgba(249,115,22,0.7)] animate-pulse" />
            <span>Rust Gateway :8080 (HTTP/3)</span>
          </div>

          <div className="hidden lg:flex items-center gap-2 px-2.5 py-1 rounded-lg bg-[#14151e] border border-[#232530] text-[10px] mono text-gray-300">
            <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 shadow-[0_0_6px_rgba(34,197,94,0.7)] animate-pulse" />
            <span>Dual RTX 3090 (48GB)</span>
          </div>

          <button
            onClick={onNewSession}
            className="px-3 py-1.5 rounded-lg bg-[#151722] hover:bg-[#1f212e] text-gray-200 border border-[#2d3040] hover:border-orange-500/40 text-xs font-medium transition flex items-center gap-1.5 cursor-pointer"
            title="Start new chat session"
          >
            <Plus className="w-3.5 h-3.5 text-orange-400" />
            <span className="hidden sm:inline">New Session</span>
          </button>

          <button
            onClick={onQuickDeploy}
            className="px-3.5 py-1.5 rounded-lg bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-gray-950 text-xs font-bold tracking-wider uppercase transition-all flex items-center gap-1.5 shadow-[0_0_15px_rgba(249,115,22,0.35)] cursor-pointer"
          >
            <Play className="w-3 h-3 fill-current" />
            <span>Deploy</span>
          </button>
        </div>
      </div>

      {/* Sub-Navigation Bar if in multi-tab category */}
      {activeSubTabs && (
        <div className="flex items-center gap-1.5 overflow-x-auto pt-1 pb-0.5 border-t border-[#1e202b]">
          <span className="text-[9px] mono text-gray-500 uppercase font-semibold mr-1 shrink-0">
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
                    ? 'bg-orange-500/15 border border-orange-500/40 text-orange-300 font-semibold'
                    : 'text-gray-400 hover:text-gray-200 hover:bg-[#161824]'
                }`}
              >
                <Icon className={`w-3 h-3 ${isActive ? 'text-orange-400' : 'text-gray-500'}`} />
                <span>{tab.label}</span>
              </button>
            );
          })}
        </div>
      )}
    </header>
  );
};
