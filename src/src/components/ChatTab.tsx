import React, { useState, useRef, useEffect, useCallback } from 'react';

let msgId = 0;
const nextId = () => `msg-${++msgId}-${Date.now()}`;
import { ChatMode, ChatMessage } from '../types';
import { desktop, ModelInfo } from '../lib/desktop';
import { ModelSelector } from './ModelSelector';
import {
  Bot,
  Send,
  Code2,
  Search,
  Wrench,
  Trash2,
  Copy,
  Check,
  Brain,
  Terminal,
  Loader2,
  Zap,
  Gauge,
  AlertTriangle,
} from 'lucide-react';

export const ChatTab: React.FC = () => {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState('');
  const [mode, setMode] = useState<ChatMode>('chat');
  const [isLoading, setIsLoading] = useState(false);
  const [stairContext, setStairContext] = useState(true);
  const [stairBudget, setStairBudget] = useState(1500);
  const [stairInfo, setStairInfo] = useState<{ tokens: number; crumbs: number } | null>(null);

  // Model & Unsloth-Style Execution Parameters
  const [selectedModel, setSelectedModel] = useState('qwen2.5-coder:7b');
  const [selectedProvider, setSelectedProvider] = useState('ollama');
  const [temperature, setTemperature] = useState(0.2);
  const [maxTokens, setMaxTokens] = useState(4096);
  const [lastLatency, setLastLatency] = useState<number | null>(null);
  const [copiedId, setCopiedId] = useState<string | null>(null);

  const messagesEndRef = useRef<HTMLDivElement>(null);

  const quickActions = [
    { label: '⚡ Run Code Gen (Unsloth)', text: 'Write a high-performance no_std Embassy async UART driver with ringbuffer for STM32F4.' },
    { label: '🧠 Plan Architecture', text: 'Analyze our microservices backend architecture and design a QUIC stream multiplexing plan.' },
    { label: '🔍 STAIR Memory Search', text: 'Search project memory for previous architecture decisions and hardware pin mappings.' },
    { label: '🩺 Subsystem Diagnostics', text: 'Inspect toolchains, probe-rs, and environment health using the Doctor engine.' },
  ];

  const scrollToBottom = useCallback(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  const handleCopy = (id: string, text: string) => {
    navigator.clipboard.writeText(text);
    setCopiedId(id);
    setTimeout(() => setCopiedId(null), 2000);
  };

  const handleModelChange = useCallback((m: ModelInfo) => {
    setSelectedModel(m.name);
    setSelectedProvider(m.provider);
  }, []);

  const handleSend = useCallback(async () => {
    const textToSend = input.trim();
    if (!textToSend || isLoading) return;

    const userMsg: ChatMessage = {
      id: nextId(),
      role: 'user',
      content: textToSend,
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
      meta: { mode, model: selectedModel },
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

      // Persona system prompt based on selected mode
      const systemPrompt =
        mode === 'plan'
          ? 'You are the Chief Architecture Thinker for Oxide-Tech. Classify the task, produce a step-by-step implementation plan, identify verification checks, and form a detailed coder prompt.'
          : mode === 'agent'
          ? 'You are the Autonomous Engineering Agent for Oxide-Tech. Implement production-grade Rust, embedded firmware, PCB schematics, and systems code with zero unwrap and strict safety.'
          : 'You are Oxide-Tech Local Agent — expert in embedded systems, Rust, reverse engineering, and Unsloth-style high throughput local model execution.';

      const result = await desktop.modelRunPrompt({
        prompt: textToSend,
        system_prompt: systemPrompt,
        model: selectedModel,
        provider: selectedProvider,
        temperature,
        max_tokens: maxTokens,
        stair_context: stairPrefix,
      });

      setLastLatency(result.latency_ms);

      if (result.error) {
        setMessages((prev) => [
          ...prev,
          {
            id: nextId(),
            role: 'assistant',
            content: `⚠️ **Model Execution Warning**\n\n${result.error}\n\n*Targeted Model:* \`${result.model}\` (${result.provider})\n*Latency:* ${result.latency_ms}ms`,
            timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
            meta: { mode, model: result.model, error: true },
          },
        ]);
      } else {
        setMessages((prev) => [
          ...prev,
          {
            id: nextId(),
            role: 'assistant',
            content: result.text || '(Model returned an empty response)',
            timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
            meta: {
              mode,
              model: result.model,
              tokens: result.tokens_used ?? 0,
              latency: result.latency_ms,
            },
          },
        ]);
      }
    } catch (err: any) {
      setMessages((prev) => [
        ...prev,
        {
          id: nextId(),
          role: 'assistant',
          content: `❌ **Agent Runtime Error**\n\n${err.message || 'Execution failed'}.\n\nEnsure selected model or Ollama/llama runner is accessible.`,
          timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
          meta: { mode, error: true },
        },
      ]);
    } finally {
      setIsLoading(false);
    }
  }, [input, isLoading, mode, stairContext, stairBudget, selectedModel, selectedProvider, temperature, maxTokens]);

  const handleQuickAction = useCallback((text: string) => {
    setInput(text);
  }, []);

  return (
    <div className="space-y-5 font-sans">
      {/* Top Unsloth-Style Model & Mode Bar */}
      <div className="bg-[#121216] border border-white/[0.07] rounded-2xl p-4 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(245,158,11,0.04)_0%,transparent_70%)]">
        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-3 pb-3 border-b border-white/[0.06]">
          <div className="flex items-center gap-2.5">
            <div className="w-7 h-7 rounded-lg bg-amber-500/10 border border-amber-500/20 flex items-center justify-center shrink-0">
              <Bot className="w-4 h-4 text-amber-400" />
            </div>
            <div>
              <h2 className="text-sm font-bold text-white tracking-wide uppercase flex items-center gap-2">
                Oxide Agent Studio
                <span className="text-[10px] px-1.5 py-0.5 rounded font-mono font-normal bg-emerald-500/10 text-emerald-300 border border-emerald-500/20">
                  Unsloth Runner
                </span>
              </h2>
              <p className="text-[10px] font-mono text-zinc-400 mt-0.5">
                Local GGUF · Ollama · SGLang High-Throughput · STAIR Code-ToC Memory
              </p>
            </div>
          </div>

          {/* Model Selector & Parameters */}
          <div className="flex items-center gap-2">
            <ModelSelector
              selectedModel={selectedModel}
              selectedProvider={selectedProvider}
              onModelChange={handleModelChange}
              temperature={temperature}
              onTemperatureChange={setTemperature}
              maxTokens={maxTokens}
              onMaxTokensChange={setMaxTokens}
            />

            {lastLatency !== null && (
              <div className="hidden sm:flex items-center gap-1 px-2.5 py-1.5 rounded-lg bg-[#18181e] border border-white/[0.06] text-[11px] font-mono text-zinc-400">
                <Gauge className="w-3 h-3 text-amber-400" />
                <span>{lastLatency}ms</span>
              </div>
            )}
          </div>
        </div>

        {/* Mode Selector & STAIR Controls */}
        <div className="flex flex-wrap items-center justify-between gap-2 mt-3 pt-1">
          <div className="flex items-center gap-1.5 overflow-x-auto">
            <span className="text-[10px] text-zinc-500 font-mono uppercase mr-1">Agent Mode:</span>
            <button
              onClick={() => setMode('chat')}
              className={`px-3 py-1 rounded-lg text-xs font-medium transition flex items-center gap-1.5 cursor-pointer ${
                mode === 'chat'
                  ? 'bg-amber-500/15 border border-amber-500/50 text-amber-400 font-semibold'
                  : 'bg-[#18181e] border border-white/[0.06] text-zinc-400 hover:text-white'
              }`}
            >
              <Zap className="w-3 h-3" />
              Direct Run
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
              Autonomous Agent
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
              Architectural Thinker
            </button>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={() => setStairContext(!stairContext)}
              title="Toggle STAIR Code-ToC context packing via oxide-embed"
              className={`px-2.5 py-1 rounded-lg text-[11px] transition shrink-0 cursor-pointer flex items-center gap-1 ${
                stairContext
                  ? 'text-amber-300 border border-amber-500/30 bg-amber-500/10'
                  : 'bg-[#18181e] border border-white/[0.06] text-zinc-400 hover:text-zinc-200'
              }`}
            >
              <Brain className="w-3 h-3" />
              STAIR Context {stairContext ? 'ON' : 'OFF'}
            </button>

            {messages.length > 0 && (
              <button
                onClick={() => setMessages([])}
                className="p-1 rounded-lg bg-[#18181e] hover:bg-[#22222a] border border-white/[0.06] text-zinc-500 hover:text-red-400 transition cursor-pointer"
                title="Clear Chat History"
              >
                <Trash2 className="w-3.5 h-3.5" />
              </button>
            )}
          </div>
        </div>
      </div>

      {/* Messages + Input Container */}
      <div className="bg-[#121216] border border-white/[0.07] rounded-2xl flex flex-col h-[540px] shadow-xl overflow-hidden">
        {/* Messages List */}
        <div className="flex-1 p-5 overflow-y-auto space-y-4">
          {/* Welcome Message */}
          <div className="flex items-start gap-3">
            <div className="w-6 h-6 rounded-lg bg-amber-500/10 border border-amber-500/20 flex items-center justify-center shrink-0 mt-0.5">
              <Bot className="w-3.5 h-3.5 text-amber-400" />
            </div>
            <div className="max-w-[90%] rounded-xl bg-[#18181e] border border-white/[0.06] p-3 text-xs text-zinc-200 leading-relaxed">
              <div className="flex items-center justify-between mb-2">
                <span className="text-[10px] font-mono text-zinc-400 uppercase tracking-widest font-semibold">
                  Oxide Agent · Model Engine Active
                </span>
                <span className="text-[10px] font-mono text-amber-400/80 bg-amber-500/10 px-1.5 py-0.5 rounded border border-amber-500/20">
                  {selectedModel} ({selectedProvider})
                </span>
              </div>
              <div className="whitespace-pre-wrap text-[13px] text-zinc-200">
                Ready to execute code generation, reverse engineering, and embedded firmware tasks.
                Select any local <code className="text-amber-300">.gguf</code> model or Ollama model above to run inference instantly.
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
          {messages.map((msg) => (
            <div key={msg.id} className={`flex flex-col ${msg.role === 'user' ? 'items-end' : 'items-start'}`}>
              <div
                className={`max-w-[90%] md:max-w-[85%] rounded-xl p-3.5 text-xs leading-relaxed relative group ${
                  msg.role === 'user'
                    ? 'bg-amber-500/10 border border-amber-500/30 text-zinc-100 rounded-br-xs'
                    : msg.meta?.error
                    ? 'bg-red-950/20 border border-red-500/30 text-zinc-200 rounded-bl-xs'
                    : 'bg-[#18181e] border border-white/[0.06] text-zinc-200 rounded-bl-xs'
                }`}
              >
                <div className="flex items-center justify-between gap-3 mb-1.5 border-b border-white/[0.04] pb-1">
                  <div className="flex items-center gap-2">
                    <span className="text-[9px] font-mono text-zinc-400 uppercase tracking-widest font-semibold">
                      {msg.role === 'user' ? 'You' : 'Assistant'}
                    </span>
                    {msg.meta?.model && (
                      <span className="text-[9px] font-mono text-zinc-500">· {msg.meta.model}</span>
                    )}
                  </div>

                  <div className="flex items-center gap-2">
                    {msg.meta?.latency && (
                      <span className="text-[9px] font-mono text-zinc-500">{msg.meta.latency}ms</span>
                    )}
                    {msg.meta?.tokens && (
                      <span className="text-[9px] font-mono text-zinc-500">~{msg.meta.tokens} tok</span>
                    )}
                    <button
                      onClick={() => handleCopy(msg.id, msg.content)}
                      className="opacity-0 group-hover:opacity-100 transition p-1 rounded hover:bg-white/[0.08] text-zinc-400 hover:text-white cursor-pointer"
                      title="Copy content"
                    >
                      {copiedId === msg.id ? (
                        <Check className="w-3 h-3 text-emerald-400" />
                      ) : (
                        <Copy className="w-3 h-3" />
                      )}
                    </button>
                  </div>
                </div>
                <div className="whitespace-pre-wrap text-[13px] text-zinc-200 font-sans leading-relaxed">
                  {msg.content}
                </div>
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
              <div className="bg-[#18181e] border border-white/[0.06] rounded-xl rounded-bl-xs p-3 flex items-center gap-2">
                <Loader2 className="w-3.5 h-3.5 text-amber-400 animate-spin" />
                <span className="text-xs font-mono text-zinc-300">
                  Executing on <span className="text-amber-400">{selectedModel}</span> ({selectedProvider})...
                </span>
              </div>
            </div>
          )}

          <div ref={messagesEndRef} />
        </div>

        {/* Quick Actions */}
        <div className="px-3 pb-2 border-t border-white/[0.06] bg-[#09090b]">
          <div className="flex items-center gap-1.5 overflow-x-auto pb-1 pt-1.5">
            {quickActions.map((action) => (
              <button
                key={action.label}
                onClick={() => handleQuickAction(action.text)}
                className="px-2.5 py-1 rounded-lg bg-[#18181e] hover:bg-[#22222a] text-zinc-300 hover:text-white border border-white/[0.06] hover:border-amber-500/30 text-[11px] font-medium transition shrink-0 cursor-pointer"
              >
                {action.label}
              </button>
            ))}
          </div>
        </div>

        {/* Input Area */}
        <div className="p-3 border-t border-white/[0.06] bg-[#09090b]">
          <div className="bg-[#18181e] border border-white/[0.08] rounded-xl p-3 focus-within:border-amber-500/60 focus-within:ring-1 focus-within:ring-amber-500/30 transition">
            <div className="flex items-center justify-between mb-2 text-zinc-400 text-[10px] font-mono">
              <div className="flex items-center gap-2">
                <Terminal className="w-3 h-3 text-amber-400" />
                <span>Selected: <span className="text-white font-medium">{selectedModel}</span></span>
              </div>
              {stairContext && (
                <span className="text-amber-400/80 bg-amber-500/10 px-1.5 py-0.5 rounded border border-amber-500/20">
                  STAIR Memory Attached
                </span>
              )}
            </div>
            <div className="flex items-center gap-2 mb-2">
              <textarea
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={(e) => {
                  if (e.key === 'Enter' && !e.shiftKey) {
                    e.preventDefault();
                    handleSend();
                  }
                }}
                placeholder={`Prompt ${selectedModel} (Enter to send, Shift+Enter for newline)...`}
                className="w-full bg-transparent text-xs text-zinc-100 placeholder:text-zinc-500 focus:outline-none resize-none font-sans"
                rows={2}
              />
            </div>
            <div className="flex items-center justify-between">
              <span className="text-[9px] font-mono text-zinc-600">Enter to run · Shift+Enter for newline</span>
              <button
                onClick={handleSend}
                disabled={isLoading || !input.trim()}
                className="px-4 py-1.5 rounded-lg bg-gradient-to-r from-amber-600 to-amber-500 hover:from-amber-500 hover:to-amber-400 disabled:opacity-40 text-zinc-950 text-xs font-bold tracking-wider uppercase transition-all flex items-center gap-1.5 cursor-pointer shadow-[0_0_12px_rgba(245,158,11,0.2)]"
              >
                <Send className="w-3 h-3" />
                Run Model
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
