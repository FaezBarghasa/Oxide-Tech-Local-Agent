import React, { useState } from 'react';
import {
  Swords,
  Play,
  Zap,
  Clock,
  Gauge,
  Trophy,
  CheckCircle2,
  Copy,
  Check,
  Cpu,
  Layers,
  Sparkles,
} from 'lucide-react';
import { arenaRunComparison } from '../lib/desktop';
import { ModelArenaResponse } from '../types';

export const ModelArenaTab: React.FC = () => {
  const [prompt, setPrompt] = useState(
    'Write a production-grade `#![no_std]` Rust circular ring buffer for high-frequency ADC telemetry using const generics.'
  );
  const [modelA, setModelA] = useState('qwen2.5-coder:7b');
  const [providerA, setProviderA] = useState('ollama');
  const [modelB, setModelB] = useState('deepseek-r1:8b');
  const [providerB, setProviderB] = useState('ollama');
  const [temperature, setTemperature] = useState(0.2);
  const [maxTokens, setMaxTokens] = useState(2048);

  const [running, setRunning] = useState(false);
  const [result, setResult] = useState<ModelArenaResponse | null>(null);
  const [copiedA, setCopiedA] = useState(false);
  const [copiedB, setCopiedB] = useState(false);

  const handleRunComparison = async () => {
    if (!prompt.trim()) return;
    setRunning(true);
    setResult(null);

    try {
      const res = await arenaRunComparison({
        prompt,
        model_a: modelA,
        provider_a: providerA,
        model_b: modelB,
        provider_b: providerB,
        temperature,
        max_tokens: maxTokens,
      });
      setResult(res);
    } catch (err) {
      console.error('Model Arena error', err);
    } finally {
      setRunning(false);
    }
  };

  const copyToClipboard = (text: string, isA: boolean) => {
    navigator.clipboard.writeText(text);
    if (isA) {
      setCopiedA(true);
      setTimeout(() => setCopiedA(false), 2000);
    } else {
      setCopiedB(true);
      setTimeout(() => setCopiedB(false), 2000);
    }
  };

  return (
    <div className="flex flex-col gap-6 max-w-7xl mx-auto pb-12">
      {/* Header */}
      <div className="flex flex-col md:flex-row items-start md:items-center justify-between gap-4 bg-[#111113] p-5 rounded-xl border border-[#27272A]">
        <div className="flex items-center gap-3.5">
          <div className="w-10 h-10 rounded-lg bg-amber-500/10 border border-amber-500/30 flex items-center justify-center text-amber-400">
            <Swords className="w-5 h-5" />
          </div>
          <div>
            <h1 className="text-lg font-semibold text-zinc-100 flex items-center gap-2">
              Model Arena & Evaluation Studio
              <span className="text-[11px] px-2 py-0.5 rounded bg-amber-500/20 text-amber-300 font-mono border border-amber-500/30">
                Parallel Dual-Inference
              </span>
            </h1>
            <p className="text-xs text-zinc-400">
              Benchmark base vs. fine-tuned models side-by-side with latency, TTFT, and token-per-second diffing.
            </p>
          </div>
        </div>

        <button
          onClick={handleRunComparison}
          disabled={running || !prompt.trim()}
          className="px-5 py-2 rounded-lg bg-amber-600 hover:bg-amber-500 disabled:opacity-50 text-xs font-semibold text-white flex items-center gap-2 transition-colors shadow-sm cursor-pointer"
        >
          <Play className={`w-4 h-4 ${running ? 'animate-spin' : ''}`} />
          {running ? 'Evaluating Dual Models...' : 'Run Dual Arena'}
        </button>
      </div>

      {/* Input Control Box */}
      <div className="bg-[#111113] border border-[#27272A] rounded-xl p-5 flex flex-col gap-4">
        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* Model A Selector */}
          <div className="flex flex-col gap-2 p-3 bg-[#0A0A0A] rounded-lg border border-[#27272A]">
            <div className="flex items-center justify-between">
              <span className="text-xs font-semibold text-blue-400 flex items-center gap-1.5">
                <Cpu className="w-3.5 h-3.5" /> Model A (Left)
              </span>
              <span className="text-[10px] text-zinc-500 font-mono">Reference</span>
            </div>
            <div className="grid grid-cols-2 gap-2">
              <select
                value={modelA}
                onChange={(e) => setModelA(e.target.value)}
                className="px-2.5 py-1.5 bg-[#18181B] border border-[#27272A] rounded text-xs text-zinc-200 focus:outline-none"
              >
                <option value="qwen2.5-coder:7b">Qwen 2.5 Coder 7B</option>
                <option value="deepseek-r1:8b">DeepSeek R1 8B</option>
                <option value="llama3.2:3b">Llama 3.2 3B</option>
                <option value="gemini-2.5-flash">Gemini 2.5 Flash</option>
              </select>
              <select
                value={providerA}
                onChange={(e) => setProviderA(e.target.value)}
                className="px-2.5 py-1.5 bg-[#18181B] border border-[#27272A] rounded text-xs text-zinc-200 focus:outline-none"
              >
                <option value="ollama">Ollama</option>
                <option value="sglang">SGLang</option>
                <option value="local_gguf">Local GGUF</option>
                <option value="cloud">Cloud API</option>
              </select>
            </div>
          </div>

          {/* Model B Selector */}
          <div className="flex flex-col gap-2 p-3 bg-[#0A0A0A] rounded-lg border border-[#27272A]">
            <div className="flex items-center justify-between">
              <span className="text-xs font-semibold text-purple-400 flex items-center gap-1.5">
                <Cpu className="w-3.5 h-3.5" /> Model B (Right)
              </span>
              <span className="text-[10px] text-zinc-500 font-mono">Challenger / Adapter</span>
            </div>
            <div className="grid grid-cols-2 gap-2">
              <select
                value={modelB}
                onChange={(e) => setModelB(e.target.value)}
                className="px-2.5 py-1.5 bg-[#18181B] border border-[#27272A] rounded text-xs text-zinc-200 focus:outline-none"
              >
                <option value="deepseek-r1:8b">DeepSeek R1 8B</option>
                <option value="qwen2.5-coder:7b">Qwen 2.5 Coder 7B</option>
                <option value="llama3.2:3b">Llama 3.2 3B</option>
                <option value="gemini-2.5-flash">Gemini 2.5 Flash</option>
              </select>
              <select
                value={providerB}
                onChange={(e) => setProviderB(e.target.value)}
                className="px-2.5 py-1.5 bg-[#18181B] border border-[#27272A] rounded text-xs text-zinc-200 focus:outline-none"
              >
                <option value="ollama">Ollama</option>
                <option value="sglang">SGLang</option>
                <option value="local_gguf">Local GGUF</option>
                <option value="cloud">Cloud API</option>
              </select>
            </div>
          </div>
        </div>

        <div>
          <label className="block text-[11px] font-medium text-zinc-400 mb-1">Shared Evaluation Prompt</label>
          <textarea
            rows={3}
            value={prompt}
            onChange={(e) => setPrompt(e.target.value)}
            className="w-full p-3 bg-[#0A0A0A] border border-[#27272A] rounded-lg font-mono text-xs text-zinc-200 focus:outline-none focus:border-amber-500/50"
            placeholder="Enter benchmark prompt..."
          />
        </div>

        <div className="flex items-center gap-6 text-xs text-zinc-400">
          <div className="flex items-center gap-2">
            <span>Temperature: {temperature}</span>
            <input
              type="range"
              min="0"
              max="1"
              step="0.05"
              value={temperature}
              onChange={(e) => setTemperature(parseFloat(e.target.value))}
              className="w-24 accent-amber-500"
            />
          </div>
          <div className="flex items-center gap-2">
            <span>Max Tokens: {maxTokens}</span>
            <input
              type="range"
              min="256"
              max="4096"
              step="256"
              value={maxTokens}
              onChange={(e) => setMaxTokens(parseInt(e.target.value))}
              className="w-24 accent-amber-500"
            />
          </div>
        </div>
      </div>

      {/* Winner Recommendation Banner */}
      {result?.winner_recommendation && (
        <div className="p-4 rounded-xl bg-gradient-to-r from-amber-950/40 via-[#111113] to-purple-950/40 border border-amber-500/30 flex items-center justify-between gap-4">
          <div className="flex items-center gap-3">
            <Trophy className="w-6 h-6 text-amber-400 shrink-0" />
            <div>
              <div className="text-xs font-semibold text-zinc-200">
                Evaluation Verdict: <span className="text-amber-400">{result.winner_recommendation}</span>
              </div>
              <div className="text-[11px] text-zinc-400">
                Calculated from real-time TTFT, token throughput (tok/s), and error-free completion.
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Side-by-Side Comparison Panels */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        {/* Panel A */}
        <div className="bg-[#111113] border border-[#27272A] rounded-xl p-5 flex flex-col gap-4">
          <div className="flex items-center justify-between border-b border-[#27272A] pb-3">
            <div className="flex items-center gap-2">
              <span className="w-2.5 h-2.5 rounded-full bg-blue-500" />
              <span className="text-xs font-bold text-zinc-200">{modelA}</span>
            </div>
            {result && (
              <button
                onClick={() => copyToClipboard(result.model_a_output, true)}
                className="p-1 hover:bg-[#18181B] text-zinc-400 rounded text-xs flex items-center gap-1"
              >
                {copiedA ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                {copiedA ? 'Copied' : 'Copy'}
              </button>
            )}
          </div>

          {/* Metrics Bar A */}
          {result && (
            <div className="grid grid-cols-3 gap-2 p-2.5 bg-[#0A0A0A] rounded-lg border border-[#1F1F23] text-center">
              <div>
                <div className="text-[10px] text-zinc-500">TTFT</div>
                <div className="text-xs font-mono font-bold text-blue-400">{result.model_a_ttft_ms}ms</div>
              </div>
              <div>
                <div className="text-[10px] text-zinc-500">Throughput</div>
                <div className="text-xs font-mono font-bold text-zinc-200">{result.model_a_tok_per_sec} tok/s</div>
              </div>
              <div>
                <div className="text-[10px] text-zinc-500">Total Latency</div>
                <div className="text-xs font-mono font-bold text-zinc-200">{result.model_a_total_ms}ms</div>
              </div>
            </div>
          )}

          {/* Output A */}
          <div className="min-h-[300px] max-h-[500px] overflow-y-auto p-3.5 bg-[#0A0A0A] border border-[#1F1F23] rounded-lg text-xs font-mono text-zinc-300 whitespace-pre-wrap leading-relaxed scrollbar-thin">
            {running ? (
              <div className="flex items-center justify-center h-48 text-zinc-500 gap-2">
                <Zap className="w-4 h-4 animate-bounce text-blue-400" />
                Generating tokens...
              </div>
            ) : result ? (
              result.model_a_output || (result.model_a_error ? `Error: ${result.model_a_error}` : 'No output')
            ) : (
              <span className="text-zinc-600 italic">Run comparison to stream model A response...</span>
            )}
          </div>
        </div>

        {/* Panel B */}
        <div className="bg-[#111113] border border-[#27272A] rounded-xl p-5 flex flex-col gap-4">
          <div className="flex items-center justify-between border-b border-[#27272A] pb-3">
            <div className="flex items-center gap-2">
              <span className="w-2.5 h-2.5 rounded-full bg-purple-500" />
              <span className="text-xs font-bold text-zinc-200">{modelB}</span>
            </div>
            {result && (
              <button
                onClick={() => copyToClipboard(result.model_b_output, false)}
                className="p-1 hover:bg-[#18181B] text-zinc-400 rounded text-xs flex items-center gap-1"
              >
                {copiedB ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <Copy className="w-3.5 h-3.5" />}
                {copiedB ? 'Copied' : 'Copy'}
              </button>
            )}
          </div>

          {/* Metrics Bar B */}
          {result && (
            <div className="grid grid-cols-3 gap-2 p-2.5 bg-[#0A0A0A] rounded-lg border border-[#1F1F23] text-center">
              <div>
                <div className="text-[10px] text-zinc-500">TTFT</div>
                <div className="text-xs font-mono font-bold text-purple-400">{result.model_b_ttft_ms}ms</div>
              </div>
              <div>
                <div className="text-[10px] text-zinc-500">Throughput</div>
                <div className="text-xs font-mono font-bold text-zinc-200">{result.model_b_tok_per_sec} tok/s</div>
              </div>
              <div>
                <div className="text-[10px] text-zinc-500">Total Latency</div>
                <div className="text-xs font-mono font-bold text-zinc-200">{result.model_b_total_ms}ms</div>
              </div>
            </div>
          )}

          {/* Output B */}
          <div className="min-h-[300px] max-h-[500px] overflow-y-auto p-3.5 bg-[#0A0A0A] border border-[#1F1F23] rounded-lg text-xs font-mono text-zinc-300 whitespace-pre-wrap leading-relaxed scrollbar-thin">
            {running ? (
              <div className="flex items-center justify-center h-48 text-zinc-500 gap-2">
                <Zap className="w-4 h-4 animate-bounce text-purple-400" />
                Generating tokens...
              </div>
            ) : result ? (
              result.model_b_output || (result.model_b_error ? `Error: ${result.model_b_error}` : 'No output')
            ) : (
              <span className="text-zinc-600 italic">Run comparison to stream model B response...</span>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
