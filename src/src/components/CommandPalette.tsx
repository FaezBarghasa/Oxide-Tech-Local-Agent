import React, { useEffect, useState, useRef } from 'react';
import { useUI } from '../store/uiStore';
import {
  Search,
  Cpu,
  Key,
  Database,
  Globe,
  LayoutDashboard,
  MessageSquare,
  Boxes,
  Zap,
  Flame,
  Brain,
  Wrench,
  Stethoscope,
  Settings,
  ArrowRight,
  Network,
  Binary,
  Smartphone,
  ShieldAlert,
} from 'lucide-react';

interface CommandAction {
  id: string;
  label: string;
  category: 'Actions' | 'Navigation' | 'Engines' | 'Tooling';
  icon: React.ReactNode;
  shortcut?: string;
}

const ACTIONS: CommandAction[] = [
  { id: 'deploy', label: 'Deploy Qwen3-8B with Candle…', category: 'Actions', icon: <Cpu className="w-4 h-4 text-[#10B981]" />, shortcut: '↵' },
  { id: 'pair-mobile', label: 'Pair Mobile Companion (Zero-Trust P2P WebRTC)', category: 'Actions', icon: <Smartphone className="w-4 h-4 text-cyan-400" /> },
  { id: 'hitl-gate', label: 'Inspect HITL Execution Gate (Safety Authorizations)', category: 'Actions', icon: <ShieldAlert className="w-4 h-4 text-amber-400" /> },
  { id: 'gen-key', label: 'Generate new API key for Gateway', category: 'Actions', icon: <Key className="w-4 h-4 text-[#8B5CF6]" /> },
  { id: 'tunnel', label: 'Start Cloudflare Tunnel', category: 'Actions', icon: <Globe className="w-4 h-4 text-cyan-400" /> },
  { id: 'dataset', label: 'Open Dataset Formatter (JSONL/CSV)', category: 'Tooling', icon: <Database className="w-4 h-4 text-amber-400" /> },
  { id: 'export-gguf', label: 'Export fine-tuned model to GGUF', category: 'Tooling', icon: <Flame className="w-4 h-4 text-orange-400" /> },
  { id: 'compact-memory', label: 'Run STAIR Context Compaction', category: 'Tooling', icon: <Brain className="w-4 h-4 text-pink-400" /> },
  { id: 'nav-overview', label: 'Go to Dashboard', category: 'Navigation', icon: <LayoutDashboard className="w-4 h-4 text-zinc-400" /> },
  { id: 'nav-chat', label: 'Go to Playground', category: 'Navigation', icon: <MessageSquare className="w-4 h-4 text-zinc-400" /> },
  { id: 'nav-catalog', label: 'Go to Model Hub', category: 'Navigation', icon: <Boxes className="w-4 h-4 text-zinc-400" /> },
  { id: 'nav-engines', label: 'Go to Inference Engines', category: 'Navigation', icon: <Zap className="w-4 h-4 text-zinc-400" /> },
  { id: 'nav-gateway', label: 'Go to Gateway & API Keys', category: 'Navigation', icon: <Key className="w-4 h-4 text-zinc-400" /> },
  { id: 'nav-mcp', label: 'Go to MCP Agents & Tools', category: 'Navigation', icon: <Wrench className="w-4 h-4 text-zinc-400" /> },
  { id: 'nav-graph', label: 'Go to Knowledge Graph (Code AST & Topology)', category: 'Navigation', icon: <Network className="w-4 h-4 text-zinc-400" /> },
  { id: 'nav-doctor', label: 'Run Hardware Doctor Diagnostics', category: 'Navigation', icon: <Stethoscope className="w-4 h-4 text-zinc-400" /> },
  { id: 'nav-reforge', label: 'Go to RE-Forge (Binary Disassembler & PTX)', category: 'Navigation', icon: <Binary className="w-4 h-4 text-zinc-400" /> },
  { id: 'nav-settings', label: 'Open Settings', category: 'Navigation', icon: <Settings className="w-4 h-4 text-zinc-400" /> },
];

export function CommandPalette({ onAction }: { onAction: (id: string) => void }) {
  const { paletteOpen, setPalette } = useUI();
  const [query, setQuery] = useState('');
  const [selectedIndex, setSelectedIndex] = useState(0);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'k') {
        e.preventDefault();
        setPalette(!paletteOpen);
      }
      if (e.key === 'Escape' && paletteOpen) {
        setPalette(false);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [paletteOpen, setPalette]);

  useEffect(() => {
    if (paletteOpen) {
      setQuery('');
      setSelectedIndex(0);
      setTimeout(() => inputRef.current?.focus(), 50);
    }
  }, [paletteOpen]);

  if (!paletteOpen) return null;

  const filtered = ACTIONS.filter(
    (a) =>
      a.label.toLowerCase().includes(query.toLowerCase()) ||
      a.category.toLowerCase().includes(query.toLowerCase())
  );

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'ArrowDown') {
      e.preventDefault();
      setSelectedIndex((prev) => (prev + 1) % Math.max(1, filtered.length));
    } else if (e.key === 'ArrowUp') {
      e.preventDefault();
      setSelectedIndex((prev) => (prev - 1 + filtered.length) % Math.max(1, filtered.length));
    } else if (e.key === 'Enter') {
      e.preventDefault();
      if (filtered[selectedIndex]) {
        setPalette(false);
        onAction(filtered[selectedIndex].id);
      }
    }
  };

  return (
    <div
      className="fixed inset-0 z-[80] bg-black/70 backdrop-blur-md flex justify-center pt-[14vh] px-4 animate-fade-in"
      onClick={() => setPalette(false)}
      role="dialog"
      aria-modal="true"
      aria-label="command palette"
    >
      <div
        className="w-[560px] max-w-full h-fit max-h-[70vh] rounded-xl bg-[#111113] border border-[#27272A] shadow-2xl overflow-hidden flex flex-col animate-scale-in"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Search Header */}
        <div className="flex items-center px-4 py-3.5 border-b border-[#27272A] gap-3">
          <Search className="w-4 h-4 text-zinc-400 shrink-0" />
          <input
            ref={inputRef}
            type="text"
            value={query}
            onChange={(e) => {
              setQuery(e.target.value);
              setSelectedIndex(0);
            }}
            onKeyDown={handleKeyDown}
            placeholder="Type a command or search… (↑↓ to navigate, Esc to close)"
            aria-label="command search"
            className="w-full bg-transparent text-sm text-[#FAFAFA] placeholder-zinc-500 outline-none font-mono"
          />
          <kbd className="hidden sm:inline-block px-1.5 py-0.5 text-[10px] font-mono text-zinc-400 bg-[#18181b] border border-[#27272A] rounded">
            ESC
          </kbd>
        </div>

        {/* Command List */}
        <div className="overflow-y-auto max-h-[380px] p-2 space-y-1 scrollbar-thin">
          {filtered.length === 0 ? (
            <div className="px-4 py-8 text-center text-xs text-zinc-500 font-mono">
              No commands found matching "{query}"
            </div>
          ) : (
            filtered.map((item, idx) => {
              const isSelected = idx === selectedIndex;
              return (
                <button
                  key={item.id}
                  onClick={() => {
                    setPalette(false);
                    onAction(item.id);
                  }}
                  onMouseEnter={() => setSelectedIndex(idx)}
                  className={`w-full flex items-center justify-between px-3 py-2.5 rounded-lg text-xs transition cursor-pointer text-left ${
                    isSelected
                      ? 'bg-[#10B981]/15 text-[#FAFAFA] border border-[#10B981]/30'
                      : 'text-zinc-300 hover:bg-[#18181b] border border-transparent'
                  }`}
                >
                  <div className="flex items-center gap-3 min-w-0">
                    <span className="shrink-0">{item.icon}</span>
                    <span className="truncate font-medium">{item.label}</span>
                  </div>
                  <div className="flex items-center gap-2 shrink-0 ml-2">
                    <span className="text-[10px] font-mono text-zinc-500">{item.category}</span>
                    {isSelected && <ArrowRight className="w-3.5 h-3.5 text-[#10B981]" />}
                  </div>
                </button>
              );
            })
          )}
        </div>

        {/* Footer info */}
        <div className="px-4 py-2 border-t border-[#27272A] bg-[#0A0A0A] flex items-center justify-between text-[10px] font-mono text-zinc-500">
          <span>Oxide Command Dispatch</span>
          <span>{filtered.length} actions available</span>
        </div>
      </div>
    </div>
  );
}
