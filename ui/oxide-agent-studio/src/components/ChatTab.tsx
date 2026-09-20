import React, { useState, useRef, useEffect, useCallback } from 'react';
import { v4 as uuidv4 } from 'crypto';

let msgId = 0;
const nextId = () => `msg-${++msgId}-${Date.now()}`;
import { ChatMode, ChatMessage } from '../types';
import { desktop } from '../lib/desktop';
import {
  Bot,
  Send,
  Code2,
  Search,
  Wrench,
  Settings,
  Trash2,
  Copy,
  Check,
  Sparkles,
  Brain,
  Terminal,
  Loader2,
} from 'lucide-react';

export const ChatTab: React.FC = () => {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [mode, setMode] = useState<ChatMode>('chat');
  const [isLoading, setIsLoading] = useState(false);
  const [stairContext, setStairContext] = useState(true);
  const [stairBudget, setStairBudget] = useState(1500);
  const [stairInfo, setStairInfo] = useState<{ tokens: number; crumbs: number } | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const quickActions = [
    { label: '⚡ STAIR Memory Recall', text: 'Search project memory for previous architecture decisions and hardware pin mappings.' },
    { label: '🩺 Run Subsystem Diagnostics', text: 'Inspect toolchains, probe-rs, and environment health using the Doctor engine.' },
    { label: '🛡️ Deterministic Verification', text: 'Run the complete 7-phase workspace verification matrix and generate evidence.' },
  ];

  const scrollToBottom = useCallback(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleSend = useCallback(async () => {
    const textToSend = input.trim();
    if (!textToSend || isLoading) return;

    const userMsg: ChatMessage = {
      role: 'user',
      content: textToSend,
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
      meta: { mode },
    };

    setMessages((prev) => [...prev, userMsg]);
    setInput('');
    setIsLoading(true);

    try {
      let stairPrefix = '';
      if (stairContext) {
        try {
          const packed = await desktop.injectStairContext(textToSend.trim(), { budget: stairBudget });
          stairPrefix = packed.prefix;
          setStairInfo({ tokens: packed.tokens, crumbs: packed.breadcrumbs.length });
        } catch {
          setStairInfo(null);
        }
      }

      const response = await fetch('/api/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ message: textToSend, mode, stair_prefix: stairPrefix }),
      });

      if (!response.ok) throw new Error(`HTTP ${response.status}`);
      const data = await response.json();
      const reply = data.reply || 'No response received from agent backend.';

      setMessages((prev) => [
        ...prev,
        {
          role: 'assistant',
          content: reply,
          timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
          meta: { mode, tokens: data.tokens || 0, source: data.source || 'Local Thinker Engine' },
        },
      ]);
    } catch (err: any) {
      setMessages((prev) => [
        ...prev,
        {
          role: 'assistant',
          content: `Agent runtime response: ${err.message || 'Standalone mode'}.\n\n(Ensure gateway is running on :8080 or use desktop native commands).`,
          timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
          meta: { mode, source: 'Standalone' },
        },
      ]);
    } finally {
      setIsLoading(false);
    }
  }, [input, isLoading, mode, stairContext, stairBudget]);

  const handleQuickAction = useCallback((text: string) => {
    setInput(text);
  }, []);

  return (
    <div className="space-y-5 font-sans">
      {/* Top Mode Bar */}
      <div className="bg-[#121216] border border-white/[0.07] rounded-2xl p-5 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(245,158,11,0.04)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-3.5 border-b border-white/[0.06]">
          <div className="flex items-center gap-2">
            <Bot className="w-4 h-4 text-amber-400" />
            <h2 className="text-sm font-bold text-white flex items-center gap-2 tracking-wide uppercase">
              Oxide Engineering Assistant
            </h2>
          </div>
          <p className="text-[10px] font-mono text-zinc-400 mt-0.5">
            AST-Aware Context Compaction · Local First · Zero Command Line Needed
          </p>
        </div>

        {/* Mode Selector */}
        <div className="flex flex-wrap items-center justify-between gap-2 mt-3.5 pt-3 border-t border-white/[0.06]">
          <div className="flex items-center gap-1.5 overflow-x-auto">
            <span className="text-[10px] text-zinc-500 font-mono uppercase mr-1">Mode:</span>
            <button
              onClick={() => setMode('chat')}
              className={`px-3 py-1 rounded-lg text-xs font-medium transition flex items-center gap-1.5 cursor-pointer ${
                mode === 'chat'
                  ? 'bg-amber-500/15 border border-amber-500/50 text-amber-400 font-semibold'
                  : 'bg-[#18181e] border border-white/[0.06] text-zinc-400 hover:text-white'
              }`}
            >
              <Search className="w-3 h-3" />
              Chat
            </button>
            <button
              onClick={() => setMode('agent')}
              className={`px-3 py-1 rounded-lg text-xs font-medium transition flex items-center gap-1.5 cursor-pointer ${
                mode === 'agent'
                  ? 'bg-amber-500/15 border border-amber-500/50 text-amber-400 font-semibold'
                  : 'bg-[#18181e] border border-white/[0.06] text-zinc-400 hover:text-white'
              }`}
            >
              <Wrench className="w-3 h-3" />
              Agent
            </button>
            <button
              onClick={() => setMode('plan')}
              className={`px-3 py-1 rounded-lg text-xs font-medium transition flex items-center gap-1.5 cursor-pointer ${
                mode === 'plan'
                  ? 'bg-amber-500/15 border border-amber-500/50 text-amber-400 font-semibold'
                  : 'bg-[#18181e] border border-white/[0.06] text-zinc-400 hover:text-white'
              }`}
            >
              <Code2 className="w-3 h-3" />
              Plan
            </button>
          </div>
          <div className="flex items-center gap-1.5">
            <button
              onClick={() => setStairContext(!stairContext)}
              title="Toggle STAIR Code-ToC context packing via oxide-embed"
              className={`px-2.5 py-1 rounded-lg text-[11px] transition shrink-0 cursor-pointer ${
                stairContext
                  ? 'text-amber-300 border-amber-500/30 bg-amber-500/10'
                  : 'bg-[#18181e] border border-white/[0.06] text-zinc-400 hover:text-zinc-200'
              }`}
            >
              <Brain className="w-3 h-3 inline mr-1" />
              STAIR Context
            </button>
          </div>
        </div>
      </div>

      {/* Messages + Input */}
      <div className="bg-[#121216] border border-white/[0.07] rounded-2xl flex flex-col h-[540px] shadow-xl overflow-hidden">
        {/* Messages */}
        <div className="flex-1 p-5 overflow-y-auto space-y-4">
          {/* Initial assistant message */}
          <div className="flex items-start gap-3">
            <div className="w-6 h-6 rounded-lg bg-amber-500/10 border border-amber-500/20 flex items-center justify-center shrink-0 mt-0.5">
              <Bot className="w-3.5 h-3.5 text-amber-400" />
            </div>
            <div className="max-w-[88%] rounded-xl bg-[#18181e] border border-white/[0.06] p-3 text-xs text-zinc-200 leading-relaxed">
              <div className="flex items-center gap-2 mb-2">
                <span className="text-[10px] font-mono text-zinc-400 uppercase tracking-widest font-semibold">Assistant · Oxide Agent</span>
              </div>
              <div className="whitespace-pre-wrap text-[13px] text-zinc-200">
                Directly integrated with <span className="text-amber-400/80">STAIR Code-ToC</span> via <code className="text-cyan-400/80">oxide-embed</code>.
                All telemetry, hardware probes, and workspace data are sourced from real runtime inspection.
              </div>
              {stairInfo && (
                <div className="mt-2 pt-2 border-t border-white/[0.06] flex items-center gap-3 text-[9px] font-mono text-zinc-500">
                  <span>STAIR tokens: {stairInfo.tokens}</span>
                  <span>Breadcrumbs: {stairInfo.crumbs}</span>
                </div>
              )}
            </div>
          </div>

          {/* Messages list */}
          {messages.map((msg, idx) => (
            <div key={idx} className={`flex flex-col ${msg.role === 'user' ? 'items-end' : 'items-start'}`}>
              <div className={`max-w-[88%] md:max-w-[80%] rounded-xl p-3 text-xs leading-relaxed ${
                msg.role === 'user'
                  ? 'bg-amber-500/10 border border-amber-500/30 text-zinc-100 rounded-br-xs'
                  : 'bg-[#18181e] border border-white/[0.06] text-zinc-200 rounded-bl-xs'
              }`}>
                <div className="flex items-center gap-2 mb-1">
                  <span className="text-[9px] font-mono text-zinc-500 uppercase tracking-widest font-semibold">
                    {msg.role === 'user' ? 'You' : 'Assistant · Oxide Agent'}
                  </span>
                  {msg.meta?.tokens && (
                    <span className="text-[9px] font-mono text-zinc-600">~{msg.meta.tokens} tokens</span>
                  )}
                </div>
                <div className="whitespace-pre-wrap text-[13px] text-zinc-200">{msg.content}</div>
              </div>
              <span className="text-[9px] font-mono text-zinc-600 mt-1 px-1">
                {msg.timestamp}
              </span>
            </div>
          ))}

          {/* Loading indicator */}
          {isLoading && (
            <div className="flex items-start gap-3">
              <div className="w-6 h-6 rounded-lg bg-amber-500/10 border border-amber-500/20 flex items-center justify-center shrink-0 mt-0.5">
                <Bot className="w-3.5 h-3.5 text-amber-400" />
              </div>
              <div className="bg-[#18181e] border border-white/[0.06] rounded-xl rounded-bl-xs p-3 flex items-center gap-1.5">
                <Loader2 className="w-3 h-3 text-amber-400 animate-spin" />
                <span className="text-[11px] font-mono text-zinc-400">Processing via oxide-embed...</span>
              </div>
            </div>
          )}

          <div ref={messagesEndRef} />
        </div>

        {/* Quick Actions */}
        <div className="px-3 pb-2 border-t border-white/[0.06] bg-[#09090b]">
          <div className="flex items-center gap-1.5 overflow-x-auto pb-2">
            {quickActions.map((action) => (
              <button
                key={action.label}
                onClick={() => handleQuickAction(action.text)}
                className="px-2.5 py-1 rounded bg-[#18181e] hover:bg-[#22222a] text-zinc-300 hover:text-white border border-white/[0.06] text-[11px] transition shrink-0 cursor-pointer"
              >
                {action.label}
              </button>
            ))}
          </div>
        </div>

        {/* Input */}
        <div className="p-3 border-t border-white/[0.06] bg-[#09090b]">
          <div className="bg-[#18181e] border border-white/[0.08] rounded-xl p-3 focus-within:border-amber-500/60 focus-within:ring-1 focus-within:ring-amber-500/30 transition">
            <div className="flex items-center gap-2 mb-2 text-zinc-400">
              <Terminal className="w-3 h-3" />
              <span className="text-[9px] font-mono uppercase tracking-wider">Live STAIR Context</span>
              {stairContext && <span className="text-amber-400/70 text-[9px]">Active</span>}
            </div>
            <div className="flex items-center gap-2 mb-2">
              <textarea
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={(e) => { if (e.key === 'Enter' && !e.shiftKey) { e.preventDefault(); handleSend(); } }}
                placeholder="Send a message to the agent..."
                className="w-full bg-transparent text-xs text-zinc-100 placeholder:text-zinc-500 focus:outline-none resize-none font-sans"
                rows={2}
              />
            </div>
            <div className="flex items-center justify-between">
              <span className="text-[9px] font-mono text-zinc-600">Enter to send · Shift+Enter for new line</span>
              <button
                onClick={handleSend}
                disabled={isLoading || !input.trim()}
                className="px-4 py-1.5 rounded-lg bg-gradient-to-r from-amber-600 to-amber-500 hover:from-amber-500 hover:to-amber-400 disabled:opacity-40 text-zinc-950 text-xs font-bold tracking-wider uppercase transition-all flex items-center gap-1.5 cursor-pointer shadow-[0_0_12px_rgba(245,158,11,0.2)]"
              >
                <Send className="w-3 h-3" />
                Send
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
