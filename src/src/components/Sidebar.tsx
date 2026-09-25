import React from 'react';
import { TabId } from '../types';
import { useUI } from '../store/uiStore';
import {
  MessageSquare,
  LayoutDashboard,
  Boxes,
  Zap,
  Code2,
  Cpu,
  Database,
  Flame,
  Brain,
  Search,
  Wrench,
  Stethoscope,
  Settings,
  ChevronLeft,
  ChevronRight,
  Server,
  Network,
  Binary,
  Smartphone,
  ShieldAlert,
} from 'lucide-react';

interface SidebarProps {
  currentTab: TabId;
  onSelectTab: (tab: TabId) => void;
}

interface NavSection {
  title: string;
  items: {
    id: TabId;
    label: string;
    icon: React.ReactNode;
    desc: string;
  }[];
}

const navSections: NavSection[] = [
  {
    title: 'Overview',
    items: [
      { id: 'overview', label: 'Dashboard', icon: <LayoutDashboard className="w-4 h-4" />, desc: 'System & telemetry' },
      { id: 'chat', label: 'Playground', icon: <MessageSquare className="w-4 h-4" />, desc: 'Chat & testing' },
    ],
  },
  {
    title: 'Compute',
    items: [
      { id: 'catalog', label: 'Model Hub', icon: <Boxes className="w-4 h-4" />, desc: 'Universal loader' },
      { id: 'sglang', label: 'Engines', icon: <Zap className="w-4 h-4" />, desc: 'Candle / vLLM / SGLang' },
    ],
  },
  {
    title: 'Network',
    items: [
      { id: 'endpoints', label: 'Gateway', icon: <Code2 className="w-4 h-4" />, desc: 'OpenAI API & Keys' },
      { id: 'mcp', label: 'Agents', icon: <Wrench className="w-4 h-4" />, desc: 'MCP tool sandboxes' },
    ],
  },
  {
    title: 'Tooling',
    items: [
      { id: 'dataset', label: 'Datasets', icon: <Database className="w-4 h-4" />, desc: 'Formatting & recipes' },
      { id: 'soup', label: 'Export', icon: <Flame className="w-4 h-4" />, desc: 'GGUF & weight soups' },
      { id: 'memory', label: 'Memory', icon: <Brain className="w-4 h-4" />, desc: 'STAIR & compaction' },
      { id: 'graph', label: 'Graph', icon: <Network className="w-4 h-4" />, desc: 'Code topology & AST' },
    ],
  },
  {
    title: 'System',
    items: [
      { id: 'doctor', label: 'Diagnostics', icon: <Stethoscope className="w-4 h-4" />, desc: 'Hardware doctor' },
      { id: 'reforge', label: 'RE-Forge', icon: <Binary className="w-4 h-4" />, desc: 'Binary disassembly & PTX' },
      { id: 'settings', label: 'Settings', icon: <Settings className="w-4 h-4" />, desc: 'Preferences' },
    ],
  },
];

export const Sidebar: React.FC<SidebarProps> = ({ currentTab, onSelectTab }) => {
  const { sidebarCollapsed, toggleSidebar, setMobileCompanionOpen, setHitlOpen, hitlPendingCount } = useUI();

  return (
    <aside
      className={`${
        sidebarCollapsed ? 'w-16' : 'w-64'
      } bg-[#0A0A0A] border-r border-[#27272A] flex flex-col h-screen sticky top-0 shrink-0 z-30 font-sans transition-all duration-200 select-none`}
    >
      {/* Brand Header */}
      <div className="p-3.5 border-b border-[#27272A] flex items-center justify-between bg-[#111113]">
        <div className="flex items-center gap-2.5 min-w-0">
          <div className="w-7 h-7 rounded-lg bg-[#18181b] border border-[#27272A] p-1 flex items-center justify-center shrink-0">
            <img
              src="/assets/oxide-logo.png"
              alt="Oxide-Tech"
              className="w-full h-full object-contain"
              onError={(e) => {
                // Fallback icon if image not found
                e.currentTarget.style.display = 'none';
              }}
            />
            <Cpu className="w-4 h-4 text-[#10B981]" />
          </div>
          {!sidebarCollapsed && (
            <div className="min-w-0">
              <div className="text-xs font-bold text-[#FAFAFA] tracking-tight flex items-center gap-1.5">
                Oxide-Tech
                <span className="text-[9px] px-1.5 py-0.2 rounded-full font-mono bg-[#8B5CF6]/15 text-[#8B5CF6] border border-[#8B5CF6]/30">
                  OS
                </span>
              </div>
              <div className="text-[9px] text-[#A1A1AA] font-mono truncate">Local AI Studio</div>
            </div>
          )}
        </div>
        <button
          onClick={toggleSidebar}
          title={sidebarCollapsed ? 'Expand sidebar' : 'Collapse sidebar'}
          className="p-1 rounded-md text-zinc-400 hover:text-white hover:bg-[#18181b] transition cursor-pointer"
        >
          {sidebarCollapsed ? <ChevronRight className="w-3.5 h-3.5" /> : <ChevronLeft className="w-3.5 h-3.5" />}
        </button>
      </div>

      {/* Navigation Sections */}
      <nav className="flex-1 overflow-y-auto p-2 space-y-4 scrollbar-thin">
        {navSections.map((sec) => (
          <div key={sec.title} className="space-y-1">
            {!sidebarCollapsed && (
              <div className="text-[9px] font-mono uppercase font-bold text-zinc-500 tracking-[0.18em] px-2.5 mb-1">
                {sec.title}
              </div>
            )}
            {sec.items.map((item) => {
              const isActive = currentTab === item.id;
              return (
                <button
                  key={item.id}
                  onClick={() => onSelectTab(item.id)}
                  title={sidebarCollapsed ? `${item.label} — ${item.desc}` : undefined}
                  className={`w-full flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-xs font-medium transition-all text-left relative cursor-pointer group ${
                    isActive
                      ? 'bg-[#10B981]/[0.12] text-[#FAFAFA] border border-[#10B981]/30 shadow-sm'
                      : 'text-[#A1A1AA] hover:text-[#FAFAFA] hover:bg-[#18181b] border border-transparent'
                  }`}
                >
                  {isActive && (
                    <span className="absolute left-0 top-1/2 -translate-y-1/2 w-1 h-4 bg-[#10B981] rounded-r-full" />
                  )}
                  <span
                    className={`shrink-0 transition-colors ${
                      isActive ? 'text-[#10B981]' : 'text-zinc-400 group-hover:text-zinc-200'
                    }`}
                  >
                    {item.icon}
                  </span>
                  {!sidebarCollapsed && (
                    <>
                      <span className="truncate">{item.label}</span>
                      <span className="ml-auto text-[9px] text-zinc-500 font-mono truncate">{item.desc}</span>
                    </>
                  )}
                </button>
              );
            })}
          </div>
        ))}
      </nav>

      {/* Quick Action Controls */}
      <div className="p-2 border-t border-[#27272A] bg-[#0c0d10] space-y-1">
        <button
          onClick={() => setMobileCompanionOpen(true)}
          title="Pair Mobile Companion (Zero-Trust P2P WebRTC)"
          className="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs font-medium text-cyan-400 hover:text-cyan-300 hover:bg-cyan-950/30 border border-cyan-900/40 transition-all cursor-pointer group"
        >
          <Smartphone className="w-4 h-4 shrink-0 text-cyan-400 group-hover:scale-105 transition-transform" />
          {!sidebarCollapsed && (
            <>
              <span className="truncate">Mobile Companion</span>
              <span className="ml-auto text-[9px] font-mono px-1 rounded bg-cyan-500/10 text-cyan-400">P2P</span>
            </>
          )}
        </button>
        <button
          onClick={() => setHitlOpen(true)}
          title="Inspect Human-in-the-Loop (HITL) Execution Gate"
          className="w-full flex items-center gap-2.5 px-2.5 py-1.5 rounded-lg text-xs font-medium text-amber-400 hover:text-amber-300 hover:bg-amber-950/30 border border-amber-900/40 transition-all cursor-pointer group"
        >
          <ShieldAlert className="w-4 h-4 shrink-0 text-amber-400 group-hover:scale-105 transition-transform" />
          {!sidebarCollapsed && (
            <>
              <span className="truncate">HITL Gate</span>
              <span className="ml-auto text-[9px] font-mono px-1.5 py-0.2 rounded-full bg-amber-500/20 text-amber-300 border border-amber-500/30 animate-pulse">{hitlPendingCount} Pending</span>
            </>
          )}
        </button>
      </div>

      {/* Bottom Status / Local Workstation info */}
      <div className="p-3 border-t border-[#27272A] bg-[#111113]">
        <div className="flex items-center gap-2">
          <span className="w-2 h-2 rounded-full bg-[#10B981] animate-pulse shrink-0" />
          {!sidebarCollapsed && (
            <div className="min-w-0">
              <div className="text-[10px] font-mono text-zinc-300 font-medium">Cosmic Workstation</div>
              <div className="text-[9px] font-mono text-zinc-500">Offline-First · Pop!_OS</div>
            </div>
          )}
        </div>
      </div>
    </aside>
  );
};
