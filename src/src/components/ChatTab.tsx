import React, { useState, useRef, useEffect, useCallback } from 'react';
import { ChatMode, ChatMessage } from '../types';
import { desktop, ModelInfo, audioStartRecording, audioStopAndTranscribe, audioSynthesizeSpeech } from '../lib/desktop';
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
  Mic,
  MicOff,
  Volume2,
} from 'lucide-react';

let msgId = 0;
const nextId = () => `msg-${++msgId}-${Date.now()}`;

export const ChatTab: React.FC = () => {
  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      id: 'welcome-1',
      role: 'assistant',
      content: 'Welcome to **Oxide-Tech Playground**. Connected to local CUDA inference engine. Ready for embedded firmware tasks, systems code generation, audio/media synthesis, and low-latency testing.',
      timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
    },
  ]);
  const [input, setInput] = useState('');
  const [isStreaming, setIsStreaming] = useState(false);
  const [streamingContent, setStreamingContent] = useState('');
  const [showParameters, setShowParameters] = useState(true);
  const [isRecording, setIsRecording] = useState(false);
  const [playingTtsId, setPlayingTtsId] = useState<string | null>(null);

  // Model & Sampling Parameters
  const [selectedModel, setSelectedModel] = useState('Qwen2.5-Coder-7B');
  const [selectedProvider, setSelectedProvider] = useState('candle');
  const [temperature, setTemperature] = useState(0.2);
  const [topP, setTopP] = useState(0.95);
  const [presencePenalty, setPresencePenalty] = useState(0.0);
  const [frequencyPenalty, setFrequencyPenalty] = useState(0.0);
  const [repetitionPenalty, setRepetitionPenalty] = useState(1.1);
  const [mirostatMode, setMirostatMode] = useState<0 | 1 | 2>(0);
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

  const handleToggleRecording = async () => {
    if (!isRecording) {
      try {
        await audioStartRecording();
        setIsRecording(true);
      } catch (e) {
        console.error('Audio recording start error:', e);
      }
    } else {
      setIsRecording(false);
      try {
        const res = await audioStopAndTranscribe();
        if (res.text && res.text.trim()) {
          setInput((prev) => (prev ? `${prev} ${res.text}` : res.text));
        }
      } catch (e) {
        console.error('Audio transcribe error:', e);
      }
    }
  };

  const handlePlayTts = async (msgId: string, text: string) => {
    setPlayingTtsId(msgId);
    try {
      const res = await audioSynthesizeSpeech(text);
      if (res.audio_base64) {
        const audio = new Audio(`data:audio/wav;base64,${res.audio_base64}`);
        audio.onended = () => setPlayingTtsId(null);
        audio.onerror = () => setPlayingTtsId(null);
        await audio.play();
      } else {
        setPlayingTtsId(null);
      }
    } catch (e) {
      console.error('TTS playback error:', e);
      setPlayingTtsId(null);
    }
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
      const result = await desktop.modelRunPrompt({
        prompt: textToSend,
        system_prompt: systemPrompt,
        model: selectedModel,
        provider: selectedProvider,
        temperature,
        max_tokens: maxTokens,
      });

      const responseText = result.text || result.error || 'Execution finished.';
      
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
                tokens: result.tokens_used ?? undefined,
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
              <h3 className="text-xs font-bold uppercase tracking-wider text-[#FAFAFA]">Sampling Matrix</h3>
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
              rows={3}
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
              <span>Exact (0.0)</span>
              <span>Creative (1.5)</span>
            </div>
          </div>

          {/* Top-P Slider */}
          <div className="space-y-1.5">
            <div className="flex justify-between text-xs font-mono">
              <span className="text-zinc-400">Top-P (Nucleus)</span>
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

          {/* Repetition Penalty Slider */}
          <div className="space-y-1.5">
            <div className="flex justify-between text-xs font-mono">
              <span className="text-zinc-400">Repetition Penalty</span>
              <span className="text-[#FAFAFA] font-bold">{repetitionPenalty.toFixed(2)}</span>
            </div>
            <input
              type="range"
              min="0.8"
              max="1.5"
              step="0.05"
              value={repetitionPenalty}
              onChange={(e) => setRepetitionPenalty(parseFloat(e.target.value))}
              className="w-full accent-[#8B5CF6] bg-[#18181b] h-1.5 rounded-lg cursor-pointer"
            />
          </div>

          {/* Mirostat Mode Selector */}
          <div className="space-y-1.5">
            <label className="text-[11px] font-mono uppercase text-zinc-400 font-semibold">Mirostat Sampling</label>
            <div className="grid grid-cols-3 gap-1.5 text-[10px] font-mono">
              {[
                { id: 0, label: 'Off' },
                { id: 1, label: 'v1.0' },
                { id: 2, label: 'v2.0' },
              ].map((m) => (
                <button
                  key={m.id}
                  onClick={() => setMirostatMode(m.id as 0 | 1 | 2)}
                  className={`py-1.5 px-2 rounded-lg border transition ${
                    mirostatMode === m.id
                      ? 'bg-[#8B5CF6]/20 border-[#8B5CF6] text-white font-bold'
                      : 'bg-[#18181b] border-[#27272A] text-zinc-400 hover:text-zinc-200'
                  }`}
                >
                  {m.label}
                </button>
              ))}
            </div>
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
              setRepetitionPenalty(1.1);
              setMirostatMode(0);
              setMaxTokens(4096);
            }}
            className="text-[11px] font-mono text-zinc-400 hover:text-white flex items-center gap-1 transition cursor-pointer"
          >
            <RotateCcw className="w-3 h-3" />
            Reset Defaults
          </button>
          <span className="text-[10px] font-mono text-zinc-500">Audio Forge Active</span>
        </div>
      </div>

      {/* 2. Main Chat & Playground Area */}
      <div className="flex-1 bg-[#111113] border border-[#27272A] rounded-xl flex flex-col shadow-lg overflow-hidden relative">
        {/* Chat Header */}
        <div className="p-3.5 border-b border-[#27272A] bg-[#0A0A0A] flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="w-2 h-2 rounded-full bg-[#10B981] animate-pulse" />
            <span className="text-xs font-bold text-[#FAFAFA]">{selectedModel}</span>
            <span className="text-[10px] font-mono text-zinc-500">· Playground & Audio-Forge</span>
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
                    <div className="flex items-center gap-1.5">
                      <button
                        onClick={() => handlePlayTts(m.id, m.content)}
                        className={`hover:text-zinc-300 transition cursor-pointer ${
                          playingTtsId === m.id ? 'text-[#10B981] animate-pulse' : ''
                        }`}
                        title="TTS Voice Playback (Kokoro/Piper)"
                      >
                        <Volume2 className="w-3.5 h-3.5" />
                      </button>
                      <button
                        onClick={() => handleCopy(m.id, m.content)}
                        className="hover:text-zinc-300 transition cursor-pointer"
                        title="Copy content"
                      >
                        {copiedId === m.id ? <Check className="w-3 h-3 text-[#10B981]" /> : <Copy className="w-3 h-3" />}
                      </button>
                    </div>
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
            <button
              onClick={handleToggleRecording}
              title={isRecording ? 'Stop Recording & Transcribe' : 'Voice Input (STT)'}
              className={`p-2 rounded-lg transition cursor-pointer shrink-0 ${
                isRecording
                  ? 'bg-red-500/20 text-red-400 border border-red-500 animate-pulse'
                  : 'text-zinc-400 hover:text-[#FAFAFA] hover:bg-[#18181b]'
              }`}
            >
              {isRecording ? <MicOff className="w-4 h-4" /> : <Mic className="w-4 h-4" />}
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
              placeholder={
                isRecording
                  ? 'Listening to microphone… speak now.'
                  : 'Ask a question, test prompts, generate Rust kernels… (Enter to send)'
              }
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
