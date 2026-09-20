import React, { useState } from 'react';
import { Cpu, RefreshCw, Layers, CheckCircle2, Play, HardDrive, Zap } from 'lucide-react';

export const SglangTab: React.FC = () => {
  const [cacheHit, setCacheHit] = useState(87.3);
  const [activeEndpoint, setActiveEndpoint] = useState('/generate');
  const [payload, setPayload] = useState('{"prompt": "fn test_spi()", "temperature": 0.2, "max_tokens": 128}');
  const [apiResponse, setApiResponse] = useState<string | null>(null);
  const [isCalling, setIsCalling] = useState(false);

  const handleCallApi = () => {
    setIsCalling(true);
    setApiResponse(null);
    setTimeout(() => {
      setIsCalling(false);
      setApiResponse(
        JSON.stringify(
          {
            text: 'pub fn test_spi() -> Result<(), embassy_stm32::spi::Error> {\n    let mut spi = Spi::new(p.SPI1, ...);\n    Ok(())\n}',
            meta_info: {
              id: 'gen_' + Math.random().toString(36).substring(2, 8),
              finish_reason: 'stop',
              prompt_tokens: 14,
              completion_tokens: 38,
              cached_tokens: 12,
              radix_cache_hit: true,
              latency_ms: 124,
            },
          },
          null,
          2
        )
      );
    }, 600);
  };

  return (
    <div className="space-y-6 font-sans">
      {/* Header */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(249,115,22,0.1)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-4 border-b border-[#232530]">
          <div>
            <div className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Cpu className="w-4 h-4 text-orange-400" />
              <span>SGLang TP=2 Serving Engine · Unsloth Optimized</span>
            </div>
            <div className="text-[10px] mono text-gray-400 mt-0.5">
              Qwen/Qwen2.5-32B (4-bit QDoRA) · RadixAttention LRU Tree · Multi-LoRA Multi-Tenant
            </div>
          </div>
          <span className="text-[10px] mono px-3 py-1 rounded-md bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 font-bold flex items-center gap-2">
            <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
            SGLang :8080 ONLINE
          </span>
        </div>

        {/* Runtime KPIs */}
        <div className="grid grid-cols-2 md:grid-cols-5 gap-3 mt-4">
          <div className="bg-[#181a24] border border-[#262838] p-3 rounded-lg">
            <div className="text-[10px] mono uppercase text-gray-400 font-semibold">Active Model</div>
            <div className="text-xs font-bold text-white mt-1">Qwen2.5-32B</div>
            <div className="text-[9px] mono text-orange-400 font-semibold">Unsloth 4-bit</div>
          </div>

          <div className="bg-[#181a24] border border-[#262838] p-3 rounded-lg">
            <div className="text-[10px] mono uppercase text-gray-400 font-semibold">TP Cluster</div>
            <div className="text-base font-bold mono text-amber-400 mt-0.5">TP=2</div>
            <div className="text-[9px] mono text-gray-400">Dual RTX 3090</div>
          </div>

          <div className="bg-[#181a24] border border-[#262838] p-3 rounded-lg">
            <div className="text-[10px] mono uppercase text-gray-400 font-semibold">Port</div>
            <div className="text-base font-bold mono text-orange-400 mt-0.5">:8080</div>
            <div className="text-[9px] mono text-gray-400">HTTP & OpenAI API</div>
          </div>

          <div className="bg-[#181a24] border border-[#262838] p-3 rounded-lg">
            <div className="text-[10px] mono uppercase text-gray-400 font-semibold">Max LoRA</div>
            <div className="text-base font-bold mono text-emerald-400 mt-0.5">r=32</div>
            <div className="text-[9px] mono text-gray-400">Multi-adapter</div>
          </div>

          <div className="bg-[#181a24] border border-[#262838] p-3 rounded-lg">
            <div className="text-[10px] mono uppercase text-gray-400 font-semibold">Cache Hit</div>
            <div className="text-base font-bold mono text-amber-400 mt-0.5">{cacheHit}%</div>
            <div className="text-[9px] mono text-gray-400">RadixAttention</div>
          </div>
        </div>
      </div>

      {/* Active LoRA Adapters & REST API Tester */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Active LoRA Adapters */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 flex flex-col justify-between shadow-sm">
          <div>
            <div className="text-xs font-bold text-white uppercase tracking-wider mb-3 pb-2 border-b border-[#232530] flex items-center justify-between">
              <span>Active In-Memory LoRA Adapters</span>
              <span className="text-[10px] mono text-gray-400">3 / 8 slots used</span>
            </div>

            <div className="space-y-2">
              <div className="bg-[#181a24] border border-[#262838] p-3 rounded-lg flex items-center justify-between">
                <div>
                  <div className="text-xs font-semibold text-white">rust_v2 (embedded-rust)</div>
                  <div className="text-[10px] mono text-gray-400">rank=16 · alpha=32 · 12.4 MB</div>
                </div>
                <span className="text-[9px] mono px-2.5 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 font-bold">
                  LOADED
                </span>
              </div>

              <div className="bg-[#181a24] border border-[#262838] p-3 rounded-lg flex items-center justify-between">
                <div>
                  <div className="text-xs font-semibold text-white">pcb_v1 (kicad-schgen)</div>
                  <div className="text-[10px] mono text-gray-400">rank=16 · alpha=32 · 12.4 MB</div>
                </div>
                <span className="text-[9px] mono px-2.5 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 font-bold">
                  LOADED
                </span>
              </div>

              <div className="bg-[#181a24] border border-[#262838] p-3 rounded-lg flex items-center justify-between">
                <div>
                  <div className="text-xs font-semibold text-white">cad_v1 (build123d-step)</div>
                  <div className="text-[10px] mono text-gray-400">rank=16 · alpha=32 · 12.4 MB</div>
                </div>
                <span className="text-[9px] mono px-2.5 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 font-bold">
                  LOADED
                </span>
              </div>
            </div>
          </div>
        </div>

        {/* REST API Tester */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 flex flex-col justify-between shadow-sm">
          <div>
            <div className="text-xs font-bold text-white uppercase tracking-wider mb-3 pb-2 border-b border-[#232530] flex items-center justify-between">
              <span>SGLang Direct REST Test</span>
              <span className="text-[10px] mono text-orange-400">http://localhost:8080</span>
            </div>

            <div className="space-y-2.5 mb-3">
              <div className="flex gap-2">
                <span className="px-2.5 py-1 rounded bg-orange-500/10 text-orange-400 border border-orange-500/30 text-[10px] mono font-bold flex items-center">
                  POST
                </span>
                <select
                  value={activeEndpoint}
                  onChange={(e) => setActiveEndpoint(e.target.value)}
                  className="flex-1 px-3 py-1.5 rounded-lg bg-[#0b0c10] border border-[#232530] text-xs mono text-white focus:outline-none focus:border-orange-500 transition"
                >
                  <option value="/generate">/generate (Structured Generation)</option>
                  <option value="/update_weights_from_disk">/update_weights_from_disk (Hot-Swap)</option>
                  <option value="/load_lora_adapter">/load_lora_adapter (Deploy LoRA)</option>
                  <option value="/get_model_info">/get_model_info (Cache Telemetry)</option>
                </select>
              </div>

              <textarea
                value={payload}
                onChange={(e) => setPayload(e.target.value)}
                rows={3}
                className="w-full bg-[#0b0c10] border border-[#232530] rounded-lg p-2.5 text-xs mono text-gray-200 focus:outline-none focus:border-orange-500 transition resize-none leading-relaxed"
              />
            </div>

            <button
              onClick={handleCallApi}
              disabled={isCalling}
              className="w-full py-2 rounded-lg bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-gray-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center justify-center gap-1.5 cursor-pointer disabled:opacity-50 shadow-[0_0_15px_rgba(249,115,22,0.35)]"
            >
              <Play className="w-3.5 h-3.5 fill-current" />
              <span>{isCalling ? 'Sending Request...' : 'Send REST Request'}</span>
            </button>
          </div>

          {apiResponse && (
            <div className="mt-3">
              <pre className="bg-[#0b0c10] border border-orange-500/40 rounded-lg p-3 text-[10px] mono text-orange-300 max-h-36 overflow-y-auto leading-relaxed">
                {apiResponse}
              </pre>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
