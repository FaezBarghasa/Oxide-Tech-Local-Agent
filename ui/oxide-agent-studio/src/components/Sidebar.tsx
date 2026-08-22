import React from 'react';
import { TabId } from '../types';
import {
  MessageSquare,
  LayoutDashboard,
  Network,
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
} from 'lucide-react';

interface SidebarProps {
  currentTab: TabId;
  onSelectTab: (tab: TabId) => void;
}

export const Sidebar: React.FC<SidebarProps> = ({ currentTab, onSelectTab }) => {
  // Map granular tabs to primary workspaces
  const isTrainingSection = ['training', 'catalog', 'dataset', 'soup'].includes(currentTab);
  const isCadSection = ['grpc', 'rag', 'mcp'].includes(currentTab);
  const isServingSection = ['sglang', 'infra', 'endpoints', 'verify'].includes(currentTab);

  const mainNavigation = [
    {
      id: 'chat' as TabId,
      label: 'AI Assistant',
      desc: 'Chat, code & firmware',
      icon: MessageSquare,
      active: currentTab === 'chat',
      badge: 'Fast',
    },
    {
      id: 'overview' as TabId,
      label: 'Overview & Plan',
      desc: '7-phase roadmap & telemetry',
      icon: LayoutDashboard,
      active: currentTab === 'overview',
    },
    {
      id: 'graph' as TabId,
      label: 'Knowledge Graph',
      desc: 'All projects & hardware map',
      icon: Network,
      active: currentTab === 'graph',
      badge: '8 Projects',
      badgeColor: 'text-orange-400 bg-orange-500/10 border-orange-500/30',
    },
    {
      id: 'training' as TabId,
      label: 'Unsloth Tuning',
      desc: 'GRPO, QDoRA & Model Soup',
      icon: Zap,
      active: isTrainingSection,
      badge: '5x Turbo',
      badgeColor: 'text-orange-400 bg-orange-500/10 border-orange-500/30',
      subItems: [
        { id: 'training' as TabId, label: 'GRPO RLVR Trainer', icon: Zap },
        { id: 'catalog' as TabId, label: 'Model Catalog & VRAM', icon: Boxes },
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
        { id: 'grpc' as TabId, label: 'KiCad & DRC Bridge', icon: Layers },
        { id: 'rag' as TabId, label: 'AST Token Compactor', icon: Search },
        { id: 'mcp' as TabId, label: 'MCP Sandbox & Tools', icon: Wrench },
      ],
    },
    {
      id: 'sglang' as TabId,
      label: 'Serving & System',
      desc: 'SGLang TP=2, APIs & health',
      icon: Cpu,
      active: isServingSection,
      subItems: [
        { id: 'sglang' as TabId, label: 'SGLang TP=2 Engine', icon: Cpu },
        { id: 'infra' as TabId, label: 'System Daemons', icon: Server },
        { id: 'endpoints' as TabId, label: 'API Specifications', icon: Code2 },
        { id: 'verify' as TabId, label: 'Verification Suite', icon: CheckCircle2 },
      ],
    },
  ];

  return (
    <aside className="w-64 bg-[#0c0d12]/95 backdrop-blur-2xl border-r border-[#232530] flex flex-col h-screen sticky top-0 shrink-0 z-30 font-sans">
      {/* Brand Header */}
      <div className="p-4 border-b border-[#232530] flex items-center justify-between bg-[#111217]">
        <div className="flex items-center gap-3">
          <div className="w-8 h-8 rounded-lg bg-gradient-to-br from-orange-500 to-amber-500 flex items-center justify-center font-bold text-gray-950 text-base shadow-[0_0_15px_rgba(249,115,22,0.4)]">
            🦥
          </div>
          <div>
            <div className="text-xs font-bold text-white tracking-tight flex items-center gap-1.5">
              oxide-agent-studio
            </div>
            <div className="text-[10px] text-gray-400 font-mono">
              Unsloth · SGLang TP=2
            </div>
          </div>
        </div>
        <span className="text-[9px] mono px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 font-semibold">
          LIVE
        </span>
      </div>

      {/* Main Navigation */}
      <div className="flex-1 overflow-y-auto p-3 space-y-1.5">
        <div className="text-[9px] mono uppercase font-bold text-gray-400 tracking-[0.2em] px-2 mb-2">
          Workspaces
        </div>

        {mainNavigation.map((item) => {
          const Icon = item.icon;
          const isActive = item.active;

          return (
            <div key={item.id} className="space-y-1">
              <button
                onClick={() => onSelectTab(item.id)}
                className={`w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-xs font-medium transition-all text-left relative cursor-pointer ${
                  isActive
                    ? 'bg-orange-500/10 border border-orange-500/40 text-white font-semibold shadow-[0_0_15px_rgba(249,115,22,0.15)]'
                    : 'text-gray-400 hover:text-gray-200 hover:bg-[#151722]'
                }`}
              >
                {isActive && (
                  <span className="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-5 bg-gradient-to-b from-orange-400 to-amber-500 rounded-r-full shadow-[0_0_8px_rgba(249,115,22,0.8)]" />
                )}
                <div
                  className={`w-7 h-7 rounded-lg flex items-center justify-center shrink-0 ${
                    isActive
                      ? 'bg-orange-500/20 text-orange-400'
                      : 'bg-[#161824] text-gray-400'
                  }`}
                >
                  <Icon className="w-3.5 h-3.5" />
                </div>
                <div className="min-w-0 flex-1">
                  <div className="flex items-center justify-between">
                    <span className="truncate">{item.label}</span>
                    {item.badge && (
                      <span
                        className={`text-[8px] mono font-bold px-1.5 py-0.2 rounded border ${
                          item.badgeColor || 'text-emerald-400 bg-emerald-500/10 border-emerald-500/20'
                        }`}
                      >
                        {item.badge}
                      </span>
                    )}
                  </div>
                  <div className="text-[10px] text-gray-400 truncate mt-0.5">
                    {item.desc}
                  </div>
                </div>
              </button>

              {/* Collapsible Sub-items when parent workspace is active */}
              {isActive && item.subItems && (
                <div className="pl-6 pr-1 py-1 space-y-0.5 border-l border-orange-500/20 ml-4">
                  {item.subItems.map((sub) => {
                    const SubIcon = sub.icon;
                    const isSubActive = currentTab === sub.id;
                    return (
                      <button
                        key={sub.id}
                        onClick={() => onSelectTab(sub.id)}
                        className={`w-full flex items-center gap-2 px-2.5 py-1.5 rounded-lg text-[11px] font-medium transition text-left cursor-pointer ${
                          isSubActive
                            ? 'text-orange-300 font-semibold bg-orange-500/10 border border-orange-500/20'
                            : 'text-gray-400 hover:text-gray-200 hover:bg-[#14151e]'
                        }`}
                      >
                        <SubIcon className={`w-3 h-3 ${isSubActive ? 'text-orange-400' : 'text-gray-500'}`} />
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
      <div className="p-3 border-t border-[#232530] bg-[#111217]">
        <div className="flex items-center justify-between text-[10px] mono text-gray-300">
          <div className="flex items-center gap-2">
            <span className="w-2 h-2 rounded-full bg-emerald-400 shadow-[0_0_6px_rgba(34,197,94,0.6)] animate-pulse" />
            <span className="font-semibold text-white">Dual RTX 3090</span>
          </div>
          <span className="text-orange-400 font-bold">48GB VRAM</span>
        </div>
        <div className="mt-1.5 flex items-center justify-between text-[9px] mono text-gray-400">
          <span>SGLang :8080</span>
          <span className="text-emerald-400">gRPC :50051</span>
          <span className="text-gray-400">TP=2</span>
        </div>
      </div>
    </aside>
  );
};
