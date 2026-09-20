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

const navItems: { id: TabId; label: string; icon: React.ReactNode; desc: string }[] = [
  { id: 'chat', label: 'Assistant', icon: <MessageSquare className="w-3.5 h-3.5" />, desc: 'AI engineering agent' },
  { id: 'overview', label: 'Overview', icon: <LayoutDashboard className="w-3.5 h-3.5" />, desc: 'Workspace & telemetry' },
  { id: 'graph', label: 'Graph', icon: <Network className="w-3.5 h-3.5" />, desc: 'Relational topology' },
  { id: 'memory', label: 'Memory', icon: <Brain className="w-3.5 h-3.5" />, desc: 'STAIR & Memanto' },
  { id: 'infra', label: 'Infra', icon: <Server className="w-3.5 h-3.5" />, desc: 'System resources' },
  { id: 'grpc', label: 'gRPC', icon: <Layers className="w-3.5 h-3.5" />, desc: 'KiCad & DRC bridge' },
  { id: 'rag', label: 'RAG', icon: <Search className="w-3.5 h-3.5" />, desc: 'AST token compactor' },
  { id: 'mcp', label: 'MCP', icon: <Wrench className="w-3.5 h-3.5" />, desc: 'Tool sandbox' },
  { id: 'catalog', label: 'Catalog', icon: <Boxes className="w-3.5 h-3.5" />, desc: 'Model & VRAM' },
  { id: 'sglang', label: 'SGLang', icon: <Zap className="w-3.5 h-3.5" />, desc: 'Inference engine' },
  { id: 'training', label: 'Tune', icon: <Cpu className="w-3.5 h-3.5" />, desc: 'Fine-tuning' },
  { id: 'soup', label: 'Model Soup', icon: <Flame className="w-3.5 h-3.5" />, desc: 'Weight blending' },
  { id: 'endpoints', label: 'API', icon: <Code2 className="w-3.5 h-3.5" />, desc: 'Endpoints' },
  { id: 'verify', label: 'Verify', icon: <CheckCircle2 className="w-3.5 h-3.5" />, desc: 'Deterministic suite' },
  { id: 'doctor', label: 'Doctor', icon: <Stethoscope className="w-3.5 h-3.5" />, desc: 'Diagnostics' },
  { id: 'reforge', label: 'RE-Forge', icon: <Binary className="w-3.5 h-3.5" />, desc: 'Binary analysis' },
  { id: 'settings', label: 'Settings', icon: <Settings className="w-3.5 h-3.5" />, desc: 'Configuration' },
];

export const Sidebar: React.FC<SidebarProps> = ({ currentTab, onSelectTab }) => {
  return (
    <aside className="w-60 bg-[#0A0A0A]/95 backdrop-blur-2xl border-r border-white/[0.07] flex flex-col h-screen sticky top-0 shrink-0 z-30 font-sans">
      {/* Logo */}
      <div className="p-3.5 border-b border-white/[0.07] flex items-center gap-2.5 bg-[#111113]">
        <div className="w-7 h-7 rounded-lg bg-[#18181e] border border-white/[0.1] p-1 flex items-center justify-center shrink-0">
          <img
            src="/assets/oxide-logo.png"
            alt="Oxide-Tech"
            className="w-full h-full object-contain"
          />
        </div>
        <div className="min-w-0">
          <div className="text-xs font-bold text-white tracking-tight font-display flex items-center gap-1.5">
            Oxide-Tech
          </div>
          <div className="text-[9px] text-zinc-500 font-mono">Agent Studio</div>
        </div>
      </div>

      {/* Navigation */}
      <nav className="flex-1 overflow-y-auto p-2 space-y-0.5">
        <div className="text-[9px] font-mono uppercase font-bold text-zinc-500 tracking-[0.2em] px-2 mb-2">Navigation</div>
        {navItems.map((item) => (
          <button
            key={item.id}
            onClick={() => onSelectTab(item.id)}
            className={`w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-[11px] font-medium transition-all text-left relative cursor-pointer ${
              currentTab === item.id
                ? 'bg-[#10B981]/[0.10] text-[#10B981] border border-[#10B981]/25'
                : 'text-zinc-400 hover:text-zinc-200 hover:bg-[#18181b]'
            }`}
          >
            {currentTab === item.id && (
              <span className="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-4 bg-[#10B981] rounded-r-full" />
            )}
            <span className="shrink-0 opacity-70">{item.icon}</span>
            <span className="truncate">{item.label}</span>
            <span className="ml-auto text-[9px] text-zinc-600 font-mono truncate">{item.desc}</span>
          </button>
        ))}
      </nav>

      {/* Status footer */}
      <div className="p-3 border-t border-white/[0.07] bg-[#111113]">
        <div className="flex items-center gap-2">
          <span className="w-1.5 h-1.5 rounded-full bg-emerald-400" />
          <span className="text-[10px] font-mono text-zinc-400">Local Workstation</span>
        </div>
      </div>
    </aside>
  );
};
