import React, { useState, useRef, useEffect } from 'react';
import { ChatMode, ChatMessage } from '../types';
import { desktop } from '../lib/desktop';
import {
  MessageSquare,
  Code2,
  Search,
  Bot,
  Send,
  Wrench,
  Paperclip,
  Settings,
  Trash2,
  Copy,
  Check,
  Sparkles,
  Terminal,
  Cpu,
  BrainCircuit,
  ChevronDown,
  ChevronRight,
} from 'lucide-react';

export const ChatTab: React.FC = () => {
  const [mode, setMode] = useState<ChatMode>('chat');
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [copiedId, setCopiedId] = useState<string | null>(null);
  const [stairContext, setStairContext] = useState(true);
  const [stairBudget] = useState(1500);
  const [stairInfo, setStairInfo] = useState<{ tokens: number; crumbs: number } | null>(null);
  const [expandedThoughts, setExpandedThoughts] = useState<Set<string>>(new Set());
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const toggleThoughts = (id: string) => {
    setExpandedThoughts((prev) => {
      const next = new Set(prev);
      if (next.has(id)) {
        next.delete(id);
      } else {
        next.add(id);
      }
      return next;
    });
  };

  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      id: 'm-init',
      role: 'assistant',
      content:
        `Hello! I am your local AI Assistant for **Oxide Agent Studio**.\n\n` +
        `Directly integrated with **STAIR Code-ToC AST search** via \`oxide-embed\`, local config profiles, hardware probes, and the deterministic verifier engine.\n\n` +
        `Select a mode below or type a query to begin:`,
      timestamp: 'Ready',
    },
  ]);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages, isLoading]);

  const handleCopy = (text: string, id: string) => {
    navigator.clipboard.writeText(text);
    setCopiedId(id);
    setTimeout(() => setCopiedId(null), 2000);
  };

  const handleSend = async (userText?: string) => {
    const textToSend = userText || input;
    if (!textToSend.trim() || isLoading) return;

    const userMsgId = `u-${Date.now()}`;
    const userMsg: ChatMessage = {
      id: userMsgId,
      role: 'user',
      content: textToSend.trim(),
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
      meta: { mode },
    };

    setMessages((prev) => [...prev, userMsg]);
    if (!userText) setInput('');
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

      // Live backend execution
      const response = await fetch('/api/gemini/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          message: `${stairPrefix}${textToSend.trim()}`,
          mode,
          history: messages.slice(-6).map((m) => ({ role: m.role, content: m.content })),
        }),
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}`);
      }

      const data = await response.json();
      const reply = data.reply || 'No response received from agent backend.';

      setMessages((prev) => [
        ...prev,
        {
          id: `a-${Date.now()}`,
          role: 'assistant',
          content: reply,
          timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
          meta: {
            model: data.source || 'Local Thinker Engine',
            tokens: Math.round(reply.length / 4),
            mode,
          },
        },
      ]);
    } catch (err: any) {
      setMessages((prev) => [
        ...prev,
        {
          id: `err-${Date.now()}`,
          role: 'assistant',
          content: `Agent runtime response: ${err.message || 'Standalone mode'}.\n\n(Ensure gateway is running on :8080 or use desktop native commands).`,
          timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        },
      ]);
    } finally {
      setIsLoading(false);
    }
  };

  const quickPrompts = [
    { label: '🦀 Embassy Async SPI Driver', text: 'Generate an asynchronous SPI DMA driver for Embassy STM32 with zero-copy ring buffer.' },
    { label: '⚡ STAIR Memory Recall', text: 'Search project memory for previous architecture decisions and hardware pin mappings.' },
    { label: '🩺 Run Subsystem Diagnostics', text: 'Inspect toolchains, probe-rs, and environment health using the Doctor engine.' },
    { label: '🛡️ Deterministic Verification', text: 'Run the complete 7-phase workspace verification matrix and generate evidence.' },
  ];

  return (
    <div className="space-y-5 font-sans">
      {/* Top Mode Bar & Controls */}
      <div className="bg-[#121216] border border-white/[0.07] rounded-2xl p-5 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(245,158,11,0.06)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-3.5 border-b border-white/[0.06]">
          <div>
            <div className="text-sm font-bold text-white flex items-center gap-2 tracking-wide uppercase">
              <Bot className="w-4 h-4 text-amber-400" />
              <span>Oxide Engineering Assistant</span>
            </div>
            <div className="text-[10px] font-mono text-zinc-400 mt-0.5">
              AST-Aware Context Compaction · Local First · Zero Command Line Needed
            </div>
          </div>
        </div>

        {/* Mode Selector Chips & Quick Actions */}
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
              <MessageSquare className="w-3.5 h-3.5 text-amber-400" />
              <span>General Assistant</span>
            </button>
            <button
              onClick={() => setMode('code')}
              className={`px-3 py-1 rounded-lg text-xs font-medium transition flex items-center gap-1.5 cursor-pointer ${
                mode === 'code'
                  ? 'bg-amber-500/15 border border-amber-500/50 text-amber-400 font-semibold'
                  : 'bg-[#18181e] border border-white/[0.06] text-zinc-400 hover:text-white'
              }`}
            >
              <Code2 className="w-3.5 h-3.5 text-amber-400" />
              <span>Firmware & Rust</span>
            </button>
            <button
              onClick={() => setMode('agent')}
              className={`px-3 py-1 rounded-lg text-xs font-medium transition flex items-center gap-1.5 cursor-pointer ${
                mode === 'agent'
                  ? 'bg-amber-500/15 border border-amber-500/50 text-amber-400 font-semibold'
                  : 'bg-[#18181e] border border-white/[0.06] text-zinc-400 hover:text-white'
              }`}
            >
              <Sparkles className="w-3.5 h-3.5 text-amber-400" />
              <span>Hardware & EDA Tools</span>
            </button>
          </div>

          <button
            onClick={() => setMessages([messages[0]])}
            className="px-2.5 py-1 rounded-lg bg-[#18181e] hover:bg-[#22222a] text-zinc-400 hover:text-zinc-200 border border-white/[0.06] text-xs font-medium transition flex items-center gap-1.5 cursor-pointer ml-auto"
          >
            <Trash2 className="w-3 h-3 text-rose-400" />
            <span>Clear</span>
          </button>
        </div>
      </div>

      {/* Message Stream */}
      <div className="bg-[#121216] border border-white/[0.07] rounded-2xl flex flex-col h-[540px] shadow-xl overflow-hidden">
        <div className="flex-1 p-5 overflow-y-auto space-y-4">
          {messages.map((msg) => {
            const isUser = msg.role === 'user';
            const isTool = msg.role === 'tool';

            if (isTool) {
              return (
                <div
                  key={msg.id}
                  className="mx-2 md:mx-8 p-3 rounded-xl bg-[#09090b] border border-amber-500/30 font-mono text-xs text-amber-200"
                >
                  <div className="flex items-center justify-between text-[10px] text-amber-400 uppercase font-semibold mb-1 tracking-wider">
                    <span className="flex items-center gap-1.5">
                      <Terminal className="w-3 h-3 text-amber-400" />
                      TOOL EXECUTION: {msg.toolName}
                    </span>
                    <span className="text-emerald-400">{msg.toolDuration}</span>
                  </div>
                  <pre className="whitespace-pre-wrap text-[11px] text-zinc-300 leading-relaxed font-mono">
                    {msg.content}
                  </pre>
                </div>
              );
            }

            return (
              <div
                key={msg.id}
                className={`flex flex-col ${isUser ? 'items-end' : 'items-start'}`}
              >
                <div
                  className={`max-w-[88%] md:max-w-[80%] rounded-xl p-4 text-xs leading-relaxed ${
                    isUser
                      ? 'bg-amber-500/10 border border-amber-500/30 text-zinc-100 rounded-br-xs'
                      : 'bg-[#18181e] border border-white/[0.06] text-zinc-200 rounded-bl-xs'
                  }`}
                >
                  {/* Role Header */}
                  <div className="flex items-center justify-between gap-3 text-[10px] font-mono mb-2 pb-1.5 border-b border-white/[0.06]">
                    <span
                      className={`font-semibold uppercase tracking-wider ${
                        isUser ? 'text-amber-400' : 'text-zinc-300'
                      }`}
                    >
                      {isUser ? 'You' : 'Assistant · Oxide Agent'}
                    </span>
                    <div className="flex items-center gap-2 text-zinc-400">
                      <span>{msg.timestamp}</span>
                      <button
                        onClick={() => handleCopy(msg.content, msg.id)}
                        className="hover:text-zinc-200 transition cursor-pointer"
                        title="Copy text"
                      >
                        {copiedId === msg.id ? (
                          <Check className="w-3 h-3 text-emerald-400" />
                        ) : (
                          <Copy className="w-3 h-3" />
                        )}
                      </button>
                    </div>
                  </div>

                  {/* Collapsible Reasoning Thought Trace */}
                  {msg.reasoningTrace && (
                    <div className="mb-3 rounded-lg bg-amber-500/5 border border-amber-500/20 overflow-hidden">
                      <button
                        onClick={() => toggleThoughts(msg.id)}
                        className="w-full px-3 py-2 flex items-center justify-between text-left text-[11px] font-medium text-amber-400/90 hover:bg-amber-500/10 transition-colors"
                      >
                        <div className="flex items-center gap-1.5">
                          <BrainCircuit className="w-3.5 h-3.5 text-amber-400 animate-pulse" />
                          <span>Thought Trace</span>
                          {msg.thinkTokens && (
                            <span className="text-[10px] px-1.5 py-0.5 rounded bg-amber-500/20 text-amber-300 font-mono">
                              {msg.thinkTokens} tokens
                            </span>
                          )}
                        </div>
                        {expandedThoughts.has(msg.id) ? (
                          <ChevronDown className="w-3.5 h-3.5 text-amber-400" />
                        ) : (
                          <ChevronRight className="w-3.5 h-3.5 text-amber-400" />
                        )}
                      </button>
                      {expandedThoughts.has(msg.id) && (
                        <div className="px-3 py-2.5 bg-black/40 border-t border-amber-500/15 text-[11px] font-mono text-zinc-300 leading-relaxed whitespace-pre-wrap max-h-60 overflow-y-auto">
                          {msg.reasoningTrace}
                        </div>
                      )}
                    </div>
                  )}

                  {/* Body Content */}
                  <div className="whitespace-pre-wrap space-y-2 text-[13px] text-zinc-200">
                    {msg.content}
                  </div>

                  {/* Metadata Footer */}
                  {msg.meta && !isUser && (
                    <div className="mt-3 pt-2 border-t border-white/[0.06] flex items-center justify-between text-[9px] font-mono text-zinc-400">
                      <span>Model: {msg.meta.model || 'Thinker'}</span>
                      <span>~{msg.meta.tokens || 80} tokens</span>
                    </div>
                  )}
                </div>
              </div>
            );
          })}

          {isLoading && (
            <div className="flex flex-col items-start">
              <div className="bg-[#18181e] border border-white/[0.06] rounded-xl rounded-bl-xs p-3 text-xs text-zinc-300 flex items-center gap-2.5">
                <div className="flex gap-1.5">
                  <span className="w-2 h-2 rounded-full bg-amber-400 animate-bounce" />
                  <span className="w-2 h-2 rounded-full bg-amber-500 animate-bounce [animation-delay:0.2s]" />
                  <span className="w-2 h-2 rounded-full bg-emerald-400 animate-bounce [animation-delay:0.4s]" />
                </div>
                <span className="text-[11px] font-mono text-zinc-400">
                  Retrieving context & synthesizing response...
                </span>
              </div>
            </div>
          )}

          <div ref={messagesEndRef} />
        </div>

        {/* Quick Prompts Drawer */}
        <div className="px-4 py-2 border-t border-white/[0.06] bg-[#121216] flex items-center gap-2 overflow-x-auto">
          <span className="text-[9px] font-mono text-zinc-400 uppercase tracking-widest font-semibold shrink-0">
            Quick:
          </span>
          {quickPrompts.map((qp, idx) => (
            <button
              key={idx}
              onClick={() => handleSend(qp.text)}
              className="px-2.5 py-1 rounded bg-[#18181e] hover:bg-[#22222a] text-zinc-300 hover:text-white border border-white/[0.06] text-[11px] transition shrink-0 cursor-pointer"
            >
              {qp.label}
            </button>
          ))}
        </div>

        {/* Input Text Box */}
        <div className="p-3 border-t border-white/[0.06] bg-[#09090b]">
          <div className="bg-[#18181e] border border-white/[0.08] rounded-xl p-3 focus-within:border-amber-500/60 focus-within:ring-1 focus-within:ring-amber-500/30 transition">
            <div className="flex items-center gap-2 mb-2 text-zinc-400">
              <button
                onClick={() => setStairContext((v) => !v)}
                title="Toggle STAIR Code-ToC context packing via oxide-embed"
                className={`flex items-center gap-1.5 px-2 py-0.5 rounded border text-[10px] font-mono transition cursor-pointer ${
                  stairContext
                    ? 'text-amber-300 border-amber-500/30 bg-amber-500/10'
                    : 'text-zinc-500 border-white/[0.06] bg-transparent'
                }`}
              >
                <BrainCircuit className="w-3 h-3" />
                STAIR Context {stairContext ? 'ON' : 'OFF'}
                {stairInfo && stairContext && (
                  <span className="text-zinc-400">
                    · ~{stairInfo.tokens} tok · {stairInfo.crumbs} crumbs
                  </span>
                )}
              </button>
              <span className="text-[10px] font-mono text-zinc-500 ml-auto">
                Shift + Enter for new line
              </span>
            </div>

            <textarea
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && !e.shiftKey) {
                  e.preventDefault();
                  handleSend();
                }
              }}
              placeholder={`Ask in ${mode} mode (e.g. Write Embassy SPI driver, decompile binary, run verification)...`}
              rows={2}
              className="w-full bg-transparent text-xs text-zinc-100 placeholder:text-zinc-500 focus:outline-none resize-none font-sans"
            />

            <div className="flex items-center justify-between mt-2 pt-2 border-t border-white/[0.06]">
              <div className="text-[10px] font-mono text-zinc-400 flex items-center gap-2">
                <Cpu className="w-3 h-3 text-amber-400" />
                <span>Native Rust Execution · Zero CLI Required</span>
              </div>

              <button
                onClick={() => handleSend()}
                disabled={!input.trim() || isLoading}
                className="px-4 py-1.5 rounded-lg bg-gradient-to-r from-amber-500 to-amber-600 hover:from-amber-400 hover:to-amber-500 disabled:opacity-40 text-zinc-950 text-xs font-bold tracking-wider uppercase transition-all flex items-center gap-1.5 cursor-pointer shadow-[0_0_15px_rgba(245,158,11,0.25)]"
              >
                <Send className="w-3 h-3" />
                <span>Send</span>
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

