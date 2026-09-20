import React from 'react';
import { TabId } from '../types';
import { Play, Search, Zap, Settings } from 'lucide-react';

interface HeaderProps {
  currentTab: TabId;
  onSelectTab: (tab: TabId) => void;
  onNewSession: () => void;
  onQuickDeploy: () => void;
}

const tabInfo: Record<TabId, { title: string; subtitle: string }> = {
  chat: { title: 'AI Assistant', subtitle: 'STAIR Code-ToC AST retrieval' },
  overview: { title: 'Workspace Overview', subtitle: 'Verified crates & system telemetry' },
  graph: { title: 'Knowledge Graph', subtitle: 'Relational topology' },
  memory: { title: 'Project Memory', subtitle: 'oxide-embed semantic memory' },
  training: { title: 'Unsloth Tuning', subtitle: 'FastLanguageModel fine-tuning' },
  catalog: { title: 'Model Catalog', subtitle: 'Memory profiling & fit calculator' },
  dataset: { title: 'Dataset Recipes', subtitle: 'Visual recipe studio' },
  soup: { title: 'LoRA Model Soup', subtitle: 'Task arithmetic weight blending' },
  grpc: { title: 'KiCad & DRC Bridge', subtitle: 'Protobuf gRPC schematics' },
  rag: { title: 'AST Token Compactor', subtitle: 'Tree-Sitter scope pruner' },
  mcp: { title: 'MCP Sandbox', subtitle: 'STDIO JSON-RPC 2.0' },
  sglang: { title: 'SGLang Engine', subtitle: 'RadixAttention prefix caching' },
  endpoints: { title: 'Endpoints', subtitle: 'API configuration' },
  verify: { title: 'Verification', subtitle: '7-phase evidence matrix' },
  doctor: { title: 'Doctor', subtitle: 'Diagnostics & health' },
  reforge: { title: 'RE-Forge', subtitle: 'Binary disassembly & PTX' },
  settings: { title: 'Settings', subtitle: 'Configuration' },
  infra: { title: 'Infrastructure', subtitle: 'System resources' },
};

export const Header: React.FC<HeaderProps> = ({ currentTab, onSelectTab, onNewSession, onQuickDeploy }) => {
  const current = tabInfo[currentTab] || { title: 'Oxide Agent Studio', subtitle: '' };

  return (
    <header className="h-12 border-b border-white/[0.05] flex items-center justify-between px-4 bg-[#0A0A0A]/80 backdrop-blur-sm shrink-0">
      <div className="flex items-center gap-2.5 min-w-0">
        <span className="text-[10px] font-mono uppercase font-bold text-[#10B981]/80 px-1.5 py-0.5 rounded bg-[#10B981]/[0.08] border border-[#10B981]/20">
          {currentTab}
        </span>
        <div className="min-w-0">
          <h1 className="text-xs font-bold text-white tracking-tight font-display truncate">{current.title}</h1>
          <p className="text-[10px] text-zinc-500 font-mono truncate">{current.subtitle}</p>
        </div>
      </div>

      <div className="flex items-center gap-1.5">
        <button
          onClick={onNewSession}
          className="px-3 py-1.5 rounded-lg bg-[#141418] hover:bg-[#1c1c22] text-zinc-300 border border-white/[0.08] hover:border-[#10B981]/40 text-[11px] font-medium transition flex items-center gap-1.5 cursor-pointer"
        >
          <Play className="w-3 h-3" />
          New Session
        </button>
        <button
          onClick={onQuickDeploy}
          className="px-3 py-1.5 rounded-lg bg-[#141418] hover:bg-[#1c1c22] text-zinc-300 border border-white/[0.08] hover:border-[#10B981]/40 text-[11px] font-medium transition flex items-center gap-1.5 cursor-pointer"
        >
          <Zap className="w-3 h-3" />
          Deploy
        </button>
        <button className="p-1.5 rounded-lg hover:bg-[#18181b] text-zinc-500 hover:text-zinc-300 transition cursor-pointer">
          <Search className="w-3.5 h-3.5" />
        </button>
      </div>
    </header>
  );
};
