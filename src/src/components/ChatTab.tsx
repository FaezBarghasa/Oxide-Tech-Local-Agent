import React, { useState, useRef, useEffect, useCallback } from 'react';
import { ChatMode, ChatMessage } from '../types';
import { desktop, ModelInfo } from '../lib/desktop';
import { ModelSelector } from './ModelSelector';
import {
  Bot,
  Send,
  Sliders,
  Paperclip,
  Copy,
  Check,
  Brain,
  Zap,
  RotateCcw,
  Sparkles,
  ChevronDown,
  ChevronUp,
  Cpu,
} from 'lucide-react';

let msgId = 0;
const nextId = () => `msg-${++msgId}-${Date.now()}`;

export const ChatTab: React.FC = () => {
  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      id: 'welcome-1',
      role: 'assistant',
      content: 'Welcome to **Oxide-Tech Playground**. Connected to local CUDA inference engine. Ready for embedded firmware tasks, systems code generation, and low-latency testing.',
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
    },
  ]);
  const [input, setInput] = useState('');
  const [isStreaming, setIsStreaming] = useState(false);
  const [streamingContent, setStreamingContent] = useState('');
  const [showParameters, setShowParameters] = useState(true);

  // Model & Sampling Parameters
  const [selectedModel, setSelectedModel] = useState('Qwen2.5-Coder-7B');
  const [selectedProvider, setSelectedProvider] = useState('candle');
  const [temperature, setTemperature] = useState(0.2);
  const [topP, setTopP] = useState(0.95);
  const [maxTokens, setMaxTokens] = useState(4096);
  const [systemPrompt, setSystemPrompt] = useState(
    'You are Oxide-Tech Local Agent — expert in embedded systems, bare-metal no_std Rust, PCB design, and high-performance local AI computing.'
  );

  const [copiedId, setCopiedId] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);

  const scrollToBottom = useCallback(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, []);

  useEffect(() => {
    scrollToBottom();
  }, [messages, streamingContent]);

  const handleCopy = (id: string, text: string) => {
    navigator.clipboard.writeText(text);
    setCopiedId(id);
    setTimeout(() => setCopiedId(null), 2000);
  };

  const handleSend = async () => {
    const textToSend = input.trim();
    if (!textToSend || isStreaming) return;

    const userMsg: ChatMessage = {
      id: nextId(),
      role: 'user',
      content: textToSend,
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
    };

    setMessages((prev) => [...prev, userMsg]);
    setInput('');
    setIsStreaming(true);
    setStreamingContent('');

    try {
      // Simulate real-time streaming or call backend
      const result = await desktop.modelRunPrompt({
        prompt: textToSend,
        system_prompt: systemPrompt,
        model: selectedModel,
        provider: selectedProvider,
        temperature,
        max_tokens: maxTokens,
      });

      const responseText = result.text || result.error || 'Execution finished.';
      
      // Animate streaming tokens smoothly
      let currentLen = 0;
      const step = Math.max(1, Math.floor(responseText.length / 30));
      const interval = setInterval(() => {
        currentLen += step;
        if (currentLen >= responseText.length) {
          clearInterval(interval);
          setStreamingContent('');
          setIsStreaming(false);
          setMessages((prev) => [
            ...prev,
            {
              id: nextId(),
              role: 'assistant',
              content: responseText,
              timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
              meta: {
                model: selectedModel,
                tokens: result.tokens_used,
              },
            },
          ]);
        } else {
          setStreamingContent(responseText.slice(0, currentLen));
        }
      }, 25);
    } catch (err: unknown) {
      const errMsg = err instanceof Error ? err.message : 'Execution error';
      setIsStreaming(false);
      setMessages((prev) => [
        ...prev,
        {
          id: nextId(),
          role: 'assistant',
          content: `⚠️ **Runtime Error**: ${errMsg}`,
          timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        },
      ]);
    }
  };

  return (
    <div className="flex flex-col lg:flex-row gap-5 h-[calc(100vh-140px)] min-h-[540px] font-sans">
      {/* 1. Left Panel (Sampling Parameters & System Prompt) */}
      <div
        className={`w-full lg:w-80 shrink-0 bg-[#111113] border border-[#27272A] rounded-xl p-5 shadow-lg flex flex-col justify-between overflow-y-auto scrollbar-thin transition-all ${
          showParameters ? 'block' : 'hidden lg:block'
        }`}
      >
        <div className="space-y-5">
          <div className="flex items-center justify-between pb-3 border-b border-[#27272A]">
            <div className="flex items-center gap-2">
              <Sliders className="w-4 h-4 text-[#8B5CF6]" />
              <h3 className="text-xs font-bold uppercase tracking-wider text-[#FAFAFA]">Parameters</h3>
            </div>
            <span className="text-[10px] font-mono px-2 py-0.5 rounded bg-[#10B981]/10 text-[#10B981] border border-[#10B981]/25">
              {selectedModel}
            </span>
          </div>

          {/* System Prompt */}
          <div className="space-y-1.5">
            <label className="text-[11px] font-mono uppercase text-zinc-400 font-semibold">System Prompt</label>
            <textarea
              value={systemPrompt}
              onChange={(e) => setSystemPrompt(e.target.value)}
              rows={4}
              className="w-full bg-[#18181b] border border-[#27272A] rounded-lg p-2.5 text-xs text-zinc-200 focus:border-[#8B5CF6] focus:outline-none font-mono resize-none"
              placeholder="System prompt context…"
            />
          </div>

          {/* Temperature Slider */}
          <div className="space-y-1.5">
            <div className="flex justify-between text-xs font-mono">
              <span className="text-zinc-400">Temperature</span>
              <span className="text-[#FAFAFA] font-bold">{temperature.toFixed(2)}</span>
            </div>
            <input
              type="range"
              min="0.0"
              max="1.5"
              step="0.05"
              value={temperature}
              onChange={(e) => setTemperature(parseFloat(e.target.value))}
              className="w-full accent-[#8B5CF6] bg-[#18181b] h-1.5 rounded-lg cursor-pointer"
            />
            <div className="flex justify-between text-[9px] font-mono text-zinc-500">
              <span>Deterministic (0.0)</span>
              <span>Creative (1.5)</span>
            </div>
          </div>

          {/* Top-P Slider */}
          <div className="space-y-1.5">
            <div className="flex justify-between text-xs font-mono">
              <span className="text-zinc-400">Top-P</span>
              <span className="text-[#FAFAFA] font-bold">{topP.toFixed(2)}</span>
            </div>
            <input
              type="range"
              min="0.1"
              max="1.0"
              step="0.05"
              value={topP}
              onChange={(e) => setTopP(parseFloat(e.target.value))}
              className="w-full accent-[#8B5CF6] bg-[#18181b] h-1.5 rounded-lg cursor-pointer"
            />
          </div>

          {/* Max Tokens Slider */}
          <div className="space-y-1.5">
            <div className="flex justify-between text-xs font-mono">
              <span className="text-zinc-400">Max Tokens</span>
              <span className="text-[#FAFAFA] font-bold">{maxTokens}</span>
            </div>
            <input
              type="range"
              min="512"
              max="16384"
              step="512"
              value={maxTokens}
              onChange={(e) => setMaxTokens(parseInt(e.target.value))}
              className="w-full accent-[#8B5CF6] bg-[#18181b] h-1.5 rounded-lg cursor-pointer"
            />
          </div>
        </div>

        {/* Quick Reset */}
        <div className="pt-4 border-t border-[#27272A] flex items-center justify-between">
          <button
            onClick={() => {
              setTemperature(0.2);
              setTopP(0.95);
              setMaxTokens(4096);
            }}
            className="text-[11px] font-mono text-zinc-400 hover:text-white flex items-center gap-1 transition cursor-pointer"
          >
            <RotateCcw className="w-3 h-3" />
            Reset Defaults
          </button>
          <span className="text-[10px] font-mono text-zinc-500">Candle Runner</span>
        </div>
      </div>

      {/* 2. Main Chat & Playground Area */}
      <div className="flex-1 bg-[#111113] border border-[#27272A] rounded-xl flex flex-col shadow-lg overflow-hidden relative">
        {/* Chat Header */}
        <div className="p-3.5 border-b border-[#27272A] bg-[#0A0A0A] flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="w-2 h-2 rounded-full bg-[#10B981] animate-pulse" />
            <span className="text-xs font-bold text-[#FAFAFA]">{selectedModel}</span>
            <span className="text-[10px] font-mono text-zinc-500">· Playground Mode</span>
          </div>
          <button
            onClick={() => setMessages([])}
            className="text-xs text-zinc-400 hover:text-white px-2 py-1 rounded hover:bg-[#18181b] transition font-mono cursor-pointer"
          >
            Clear Session
          </button>
        </div>

        {/* Message Stream */}
        <div className="flex-1 overflow-y-auto p-5 space-y-6 scrollbar-thin">
          {messages.map((m) => {
            const isUser = m.role === 'user';
            return (
              <div
                key={m.id}
                className={`flex flex-col ${isUser ? 'items-end' : 'items-start'}`}
              >
                {/* Message Header / Timestamp */}
                <div className="flex items-center gap-2 mb-1.5 text-[10px] font-mono text-zinc-500">
                  <span>{isUser ? 'You' : selectedModel}</span>
                  <span>{m.timestamp}</span>
                  {!isUser && (
                    <button
                      onClick={() => handleCopy(m.id, m.content)}
                      className="hover:text-zinc-300 transition cursor-pointer"
                      title="Copy content"
                    >
                      {copiedId === m.id ? <Check className="w-3 h-3 text-[#10B981]" /> : <Copy className="w-3 h-3" />}
                    </button>
                  )}
                </div>

                {/* Message Bubble / Clean Markdown */}
                {isUser ? (
                  <div className="max-w-[85%] rounded-xl bg-[#18181b] border border-[#27272A] px-4 py-3 text-sm text-[#FAFAFA] leading-relaxed shadow-sm">
                    {m.content}
                  </div>
                ) : (
                  <div className="max-w-[95%] text-sm text-[#FAFAFA] leading-relaxed font-sans space-y-2 prose-invert">
                    <div className="whitespace-pre-wrap">{m.content}</div>
                  </div>
                )}
              </div>
            );
          })}

          {/* Real-time Streaming State with Cursor */}
          {isStreaming && (
            <div className="flex flex-col items-start">
              <div className="flex items-center gap-2 mb-1.5 text-[10px] font-mono text-zinc-500">
                <span>{selectedModel}</span>
                <span className="text-[#10B981] animate-pulse">Streaming…</span>
              </div>
              <div className="max-w-[95%] text-sm text-[#FAFAFA] leading-relaxed font-sans">
                <span className="whitespace-pre-wrap">{streamingContent}</span>
                <span className="inline-block text-[#10B981] font-mono font-bold animate-pulse ml-0.5">
                  ▋
                </span>
              </div>
            </div>
          )}

          <div ref={messagesEndRef} />
        </div>

        {/* 3. Floating Context Input Bar */}
        <div className="p-4 border-t border-[#27272A] bg-[#0A0A0A]">
          <div className="bg-[#111113] border border-[#27272A] focus-within:border-[#8B5CF6]/60 rounded-xl p-2.5 shadow-xl transition flex items-end gap-2.5">
            <button
              title="Attach context file or prompt"
              className="p-2 rounded-lg text-zinc-400 hover:text-[#FAFAFA] hover:bg-[#18181b] transition cursor-pointer shrink-0"
            >
              <Paperclip className="w-4 h-4" />
            </button>
            <textarea
              ref={inputRef}
              rows={1}
              value={input}
              onChange={(e) => setInput(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter' && !e.shiftKey) {
                  e.preventDefault();
                  handleSend();
                }
              }}
              placeholder="Ask a question, test prompts, generate Rust kernels… (Enter to send)"
              className="flex-1 bg-transparent text-sm text-[#FAFAFA] placeholder-zinc-500 outline-none resize-none py-1.5 max-h-32 font-sans"
            />
            <button
              onClick={handleSend}
              disabled={!input.trim() || isStreaming}
              className="p-2.5 rounded-lg bg-[#FAFAFA] text-black hover:bg-white hover:-translate-y-px active:translate-y-0 transition disabled:opacity-30 cursor-pointer shrink-0 shadow-md"
            >
              <Send className="w-4 h-4 fill-current" />
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
