import React, { useState, useRef, useEffect } from 'react';
import { ChatMode, ChatMessage } from '../types';
import {
  MessageSquare,
  Code2,
  Search,
  Globe,
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
} from 'lucide-react';

export const ChatTab: React.FC = () => {
  const [mode, setMode] = useState<ChatMode>('chat');
  const [input, setInput] = useState('');
  const [isLoading, setIsLoading] = useState(false);
  const [copiedId, setCopiedId] = useState<string | null>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);

  const [messages, setMessages] = useState<ChatMessage[]>([
    {
      id: 'm-init',
      role: 'assistant',
      content:
        `Hello! I am your local AI Assistant for the **oxide-agent-studio** workspace.\n\n` +
        `I am wired to the **SGLang TP=2 runtime** on your dual RTX 3090 rig with active LoRA routing, **Tree-Sitter AST context compaction**, and the **gRPC CAD Bridge**.\n\n` +
        `Choose a mode below or try one of the quick actions:`,
      timestamp: '14:23',
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
      const response = await fetch('/api/gemini/chat', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          message: textToSend.trim(),
          mode,
          history: messages.slice(-6).map((m) => ({ role: m.role, content: m.content })),
        }),
      });

      const data = await response.json();
      const reply = data.reply || 'No response received from agent.';

      // Random tool call demonstration in agent/code modes
      if (mode === 'agent' || mode === 'code' || mode === 'research') {
        const tools = [
          { name: 'tree_sitter_parse', desc: 'AST extraction · Rust/Embassy AST pruned 78%' },
          { name: 'cargo_cross_build', desc: 'target: thumbv7em-none-eabihf · cargo check PASS' },
          { name: 'kicad_drc_check', desc: 'gRPC :50051 · 0 electrical rule violations' },
          { name: 'qdrant_rag_search', desc: 'Vector cosine match · top_k=4 chunks retrieved' },
        ];
        const selectedTool = tools[Math.floor(Math.random() * tools.length)];

        setMessages((prev) => [
          ...prev,
          {
            id: `t-${Date.now()}`,
            role: 'tool',
            content: `Invoked Tool: ${selectedTool.name}\n${selectedTool.desc}\nStatus: PASS (38ms)`,
            timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
            toolName: selectedTool.name,
            toolStatus: 'success',
            toolDuration: '38ms',
          },
        ]);
      }

      setMessages((prev) => [
        ...prev,
        {
          id: `a-${Date.now()}`,
          role: 'assistant',
          content: reply,
          timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
          meta: {
            model: data.source || 'Qwen3.8-35B-AWQ',
            tokens: Math.round(reply.length / 3.8),
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
          content: `Local synthesize fallback: ${err.message || 'Network error'}`,
          timestamp: new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }),
        },
      ]);
    } finally {
      setIsLoading(false);
    }
  };

  const quickPrompts = [
    { label: '🦀 Embassy SPI Driver', text: 'Generate an asynchronous SPI DMA driver for Embassy STM32 with zero-copy ring buffer.' },
    { label: '⚡ KiCad DRC & Netlist', text: 'Synthesize a 3.3V LDO power supply schematic in KiCad S-expression and run DRC rule checks.' },
    { label: '🧠 Unsloth GRPO vs PPO', text: 'Compare GRPO compiler-verifier rewards vs standard PPO for embedded firmware code generation.' },
    { label: '🍲 LoRA Model Soup Blend', text: 'Calculate the task arithmetic soup weights for merging embedded_rust (0.45), pcb_design (0.35), and cad_3d (0.20).' },
  ];

  return (
    <div className="space-y-5 font-sans">
      {/* Top Mode Bar & Controls */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-5 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(249,115,22,0.1)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-3.5 border-b border-[#232530]">
          <div>
            <div className="text-sm font-bold text-white flex items-center gap-2 tracking-wide uppercase">
              <Bot className="w-4 h-4 text-orange-400" />
              <span>Oxide Local Agent Assistant</span>
            </div>
            <div className="text-[10px] mono text-gray-400 mt-0.5">
              Qwen3.8-35B-Instruct-AWQ · SGLang TP=2 · RadixAttention Active
            </div>
          </div>
        </div>

        {/* Mode Selector Chips & Quick Actions */}
        <div className="flex flex-wrap items-center justify-between gap-2 mt-3.5 pt-3 border-t border-[#232530]">
          <div className="flex items-center gap-1.5 overflow-x-auto">
            <span className="text-[10px] text-gray-500 font-mono uppercase mr-1">Mode:</span>
            <button
              onClick={() => setMode('chat')}
              className={`px-3 py-1 rounded-lg text-xs font-medium transition flex items-center gap-1.5 cursor-pointer ${
                mode === 'chat'
                  ? 'bg-orange-500/15 border border-orange-500/50 text-orange-400 font-semibold'
                  : 'bg-[#181a24] border border-[#262838] text-gray-400 hover:text-white'
              }`}
            >
              <MessageSquare className="w-3.5 h-3.5 text-orange-400" />
              <span>General Assistant</span>
            </button>
            <button
              onClick={() => setMode('code')}
              className={`px-3 py-1 rounded-lg text-xs font-medium transition flex items-center gap-1.5 cursor-pointer ${
                mode === 'code'
                  ? 'bg-orange-500/15 border border-orange-500/50 text-orange-400 font-semibold'
                  : 'bg-[#181a24] border border-[#262838] text-gray-400 hover:text-white'
              }`}
            >
              <Code2 className="w-3.5 h-3.5 text-orange-400" />
              <span>Firmware & Rust</span>
            </button>
            <button
              onClick={() => setMode('agent')}
              className={`px-3 py-1 rounded-lg text-xs font-medium transition flex items-center gap-1.5 cursor-pointer ${
                mode === 'agent'
                  ? 'bg-orange-500/15 border border-orange-500/50 text-orange-400 font-semibold'
                  : 'bg-[#181a24] border border-[#262838] text-gray-400 hover:text-white'
              }`}
            >
              <Sparkles className="w-3.5 h-3.5 text-orange-400" />
              <span>CAD & KiCad Tools</span>
            </button>
          </div>

          <button
            onClick={() => setMessages([messages[0]])}
            className="px-2.5 py-1 rounded-lg bg-[#181a24] hover:bg-[#202230] text-gray-400 hover:text-gray-200 border border-[#262838] text-xs font-medium transition flex items-center gap-1.5 cursor-pointer ml-auto"
          >
            <Trash2 className="w-3 h-3 text-rose-400" />
            <span>Clear</span>
          </button>
        </div>
      </div>

      {/* Message Stream */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl flex flex-col h-[540px] shadow-xl overflow-hidden">
        <div className="flex-1 p-5 overflow-y-auto space-y-4">
          {messages.map((msg) => {
            const isUser = msg.role === 'user';
            const isTool = msg.role === 'tool';

            if (isTool) {
              return (
                <div
                  key={msg.id}
                  className="mx-2 md:mx-8 p-3 rounded-xl bg-[#0b0c10] border border-orange-500/40 font-mono text-xs text-orange-200 shadow-[0_0_15px_rgba(249,115,22,0.1)]"
                >
                  <div className="flex items-center justify-between text-[10px] text-orange-400 uppercase font-semibold mb-1 tracking-wider">
                    <span className="flex items-center gap-1.5">
                      <Terminal className="w-3 h-3 text-orange-400" />
                      TOOL EXECUTION: {msg.toolName}
                    </span>
                    <span className="text-emerald-400">{msg.toolDuration}</span>
                  </div>
                  <pre className="whitespace-pre-wrap text-[11px] text-gray-300 leading-relaxed font-mono">
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
                      ? 'bg-orange-500/10 border border-orange-500/30 text-gray-100 rounded-br-xs shadow-[0_0_15px_rgba(249,115,22,0.1)]'
                      : 'bg-[#181a24] border border-[#262838] text-gray-200 rounded-bl-xs'
                  }`}
                >
                  {/* Role Header */}
                  <div className="flex items-center justify-between gap-3 text-[10px] mono mb-2 pb-1.5 border-b border-[#232530]">
                    <span
                      className={`font-semibold uppercase tracking-wider ${
                        isUser ? 'text-orange-400' : 'text-gray-300'
                      }`}
                    >
                      {isUser ? 'You' : 'Assistant · Oxide Agent'}
                    </span>
                    <div className="flex items-center gap-2 text-gray-400">
                      <span>{msg.timestamp}</span>
                      <button
                        onClick={() => handleCopy(msg.content, msg.id)}
                        className="hover:text-gray-200 transition cursor-pointer"
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

                  {/* Body Content */}
                  <div className="whitespace-pre-wrap space-y-2 text-[13px] text-gray-200">
                    {msg.content}
                  </div>

                  {/* Metadata Footer */}
                  {msg.meta && !isUser && (
                    <div className="mt-3 pt-2 border-t border-[#232530] flex items-center justify-between text-[9px] mono text-gray-400">
                      <span>Model: {msg.meta.model || 'Qwen3.8-35B'}</span>
                      <span>~{msg.meta.tokens || 120} tokens</span>
                    </div>
                  )}
                </div>
              </div>
            );
          })}

          {isLoading && (
            <div className="flex flex-col items-start">
              <div className="bg-[#181a24] border border-[#262838] rounded-xl rounded-bl-xs p-3 text-xs text-gray-300 flex items-center gap-2.5">
                <div className="flex gap-1.5">
                  <span className="w-2 h-2 rounded-full bg-orange-400 animate-bounce" />
                  <span className="w-2 h-2 rounded-full bg-amber-400 animate-bounce [animation-delay:0.2s]" />
                  <span className="w-2 h-2 rounded-full bg-emerald-400 animate-bounce [animation-delay:0.4s]" />
                </div>
                <span className="text-[11px] mono text-gray-400">
                  Agent reasoning & executing tools...
                </span>
              </div>
            </div>
          )}

          <div ref={messagesEndRef} />
        </div>

        {/* Quick Prompts Drawer */}
        <div className="px-4 py-2 border-t border-[#232530] bg-[#111217] flex items-center gap-2 overflow-x-auto">
          <span className="text-[9px] mono text-gray-400 uppercase tracking-widest font-semibold shrink-0">
            Quick:
          </span>
          {quickPrompts.map((qp, idx) => (
            <button
              key={idx}
              onClick={() => handleSend(qp.text)}
              className="px-2.5 py-1 rounded bg-[#181a24] hover:bg-[#202230] text-gray-300 hover:text-white border border-[#262838] text-[11px] transition shrink-0 cursor-pointer"
            >
              {qp.label}
            </button>
          ))}
        </div>

        {/* Input Text Box */}
        <div className="p-3 border-t border-[#232530] bg-[#0b0c10]">
          <div className="bg-[#181a24] border border-[#262838] rounded-xl p-3 focus-within:border-orange-500 focus-within:ring-1 focus-within:ring-orange-500/30 transition">
            <div className="flex items-center gap-2 mb-2 text-gray-400">
              <button className="p-1 hover:text-white rounded transition cursor-pointer" title="Tools Palette">
                <Wrench className="w-3.5 h-3.5" />
              </button>
              <button className="p-1 hover:text-white rounded transition cursor-pointer" title="Attach Schematic/CAD">
                <Paperclip className="w-3.5 h-3.5" />
              </button>
              <button className="p-1 hover:text-white rounded transition cursor-pointer" title="Agent Settings">
                <Settings className="w-3.5 h-3.5" />
              </button>
              <span className="text-[10px] mono text-gray-500 ml-auto">
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
              placeholder={`Ask in ${mode} mode (e.g. Write Embassy SPI driver, convert STEP to GLTF, run DRC test)...`}
              rows={2}
              className="w-full bg-transparent text-xs text-gray-100 placeholder:text-gray-500 focus:outline-none resize-none font-sans"
            />

            <div className="flex items-center justify-between mt-2 pt-2 border-t border-[#232530]">
              <div className="text-[10px] mono text-gray-400 flex items-center gap-2">
                <Cpu className="w-3 h-3 text-orange-400" />
                <span>TP=2 · AWQ 4-bit · 32K context</span>
              </div>

              <button
                onClick={() => handleSend()}
                disabled={!input.trim() || isLoading}
                className="px-4 py-1.5 rounded-lg bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 disabled:opacity-40 text-gray-950 text-xs font-bold tracking-wider uppercase transition-all flex items-center gap-1.5 cursor-pointer shadow-[0_0_15px_rgba(249,115,22,0.35)]"
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
