import React, { useState } from 'react';
import { LoraAdapter } from '../types';
import { Flame, Sparkles, Check, Play, RefreshCw, Cpu, Layers } from 'lucide-react';

export const ModelSoupTab: React.FC = () => {
  const [adapters, setAdapters] = useState<LoraAdapter[]>([
    {
      id: '1',
      name: 'unsloth-lora-embedded-rust-r16',
      domain: 'embedded_rust',
      rank: 16,
      weight: 0.45,
      color: 'sky',
      status: 'loaded',
    },
    {
      id: '2',
      name: 'unsloth-lora-kicad-schgen-r16',
      domain: 'pcb_design',
      rank: 16,
      weight: 0.35,
      color: 'purple',
      status: 'loaded',
    },
    {
      id: '3',
      name: 'unsloth-lora-cad-build123d-r16',
      domain: 'cad_3d',
      rank: 16,
      weight: 0.2,
      color: 'amber',
      status: 'loaded',
    },
  ]);

  const [mergeMethod, setMergeMethod] = useState('Task Arithmetic');
  const [exportFormat, setExportFormat] = useState('AWQ 4-bit (SGLang TP=2)');
  const [outputPath, setOutputPath] = useState('workspace/models/unsloth_soup_v1');
  const [isBlending, setIsBlending] = useState(false);
  const [blendResult, setBlendResult] = useState<string | null>(null);

  const totalWeight = adapters.reduce((sum, a) => sum + a.weight, 0);

  const handleUpdateWeight = (id: string, newWeight: number) => {
    setAdapters((prev) =>
      prev.map((a) => (a.id === id ? { ...a, weight: parseFloat(newWeight.toFixed(2)) } : a))
    );
  };

  const handleNormalize = () => {
    if (totalWeight === 0) return;
    setAdapters((prev) =>
      prev.map((a) => ({
        ...a,
        weight: parseFloat((a.weight / totalWeight).toFixed(2)),
      }))
    );
  };

  const handleBlend = () => {
    setIsBlending(true);
    setBlendResult(null);
    setTimeout(() => {
      setIsBlending(false);
      setBlendResult(
        `✓ Unsloth LoRA Model Soup compiled successfully with ${mergeMethod}!\n` +
        `• Base: Qwen/Qwen2.5-32B-Instruct (4-bit)\n` +
        `• Blended Deltas: ${adapters.map((a) => `${a.domain} (${a.weight})`).join(' + ')}\n` +
        `• Export Format: ${exportFormat}\n` +
        `• Target Path: ${outputPath}\n` +
        `• SGLang Hot-Swap: Active on :8080 (TP=2)`
      );
    }, 1200);
  };

  return (
    <div className="space-y-6 font-sans">
      {/* Header */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(249,115,22,0.1)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-4 border-b border-[#232530]">
          <div>
            <div className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Sparkles className="w-4 h-4 text-orange-400" />
              <span>Phase 6 · Unsloth LoRA Model Soup Blender</span>
            </div>
            <div className="text-[10px] mono text-gray-400 mt-0.5">
              θ_soup = θ_base + Σ wₖ · (θₖ - θ_base) · Task Arithmetic & TIES-Merging
            </div>
          </div>
          <div className="flex items-center gap-2.5">
            <button
              onClick={handleNormalize}
              className="px-3 py-2 rounded-lg bg-[#181a24] hover:bg-[#222432] text-gray-200 border border-[#2d3040] hover:border-orange-500/30 text-xs font-semibold uppercase tracking-wider transition flex items-center gap-1.5 cursor-pointer"
            >
              <RefreshCw className="w-3.5 h-3.5 text-orange-400" />
              Normalize (Σ=1.0)
            </button>
            <button
              onClick={handleBlend}
              disabled={isBlending}
              className="px-4 py-2 rounded-lg bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-gray-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-1.5 cursor-pointer disabled:opacity-50 shadow-[0_0_15px_rgba(249,115,22,0.35)]"
            >
              <Sparkles className="w-3.5 h-3.5" />
              <span>{isBlending ? 'Merging LoRAs...' : '🍲 Blend & Deploy'}</span>
            </button>
          </div>
        </div>

        {/* Blend Formula & Distribution Visualizer */}
        <div className="mt-4 p-4 rounded-xl bg-[#181a24] border border-[#262838]">
          <div className="text-[10px] mono uppercase text-gray-400 font-semibold mb-1">
            Current Task Arithmetic Vector
          </div>
          <div className="text-xs md:text-sm mono text-white font-bold mb-2.5 flex flex-wrap items-center gap-2">
            <span>θ_soup = θ_base +</span>
            <span className="text-orange-400">Δθ_rust({adapters[0]?.weight.toFixed(2)})</span>
            <span>+</span>
            <span className="text-amber-400">Δθ_pcb({adapters[1]?.weight.toFixed(2)})</span>
            <span>+</span>
            <span className="text-emerald-400">Δθ_cad({adapters[2]?.weight.toFixed(2)})</span>
          </div>

          <div className="flex items-center gap-3">
            <div className="flex-1 h-2 rounded-full overflow-hidden flex bg-[#0b0c10] border border-[#232530]">
              <div
                className="bg-orange-500 transition-all duration-300"
                style={{ width: `${((adapters[0]?.weight || 0) / (totalWeight || 1)) * 100}%` }}
                title="Embedded Rust"
              />
              <div
                className="bg-amber-500 transition-all duration-300"
                style={{ width: `${((adapters[1]?.weight || 0) / (totalWeight || 1)) * 100}%` }}
                title="PCB Design"
              />
              <div
                className="bg-emerald-500 transition-all duration-300"
                style={{ width: `${((adapters[2]?.weight || 0) / (totalWeight || 1)) * 100}%` }}
                title="CAD 3D"
              />
            </div>
            <span
              className={`text-xs mono font-bold ${
                Math.abs(totalWeight - 1.0) < 0.02 ? 'text-emerald-400' : 'text-amber-400'
              }`}
            >
              Σ = {totalWeight.toFixed(2)} {Math.abs(totalWeight - 1.0) < 0.02 ? '✓' : '⚠'}
            </span>
          </div>
        </div>
      </div>

      {/* Sliders and Configurations */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Adapter Weight Sliders */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 shadow-sm">
          <div className="text-xs font-bold text-white uppercase tracking-wider mb-3 pb-2 border-b border-[#232530]">
            Domain LoRA Adapter Weights
          </div>

          <div className="space-y-3">
            {adapters.map((a) => {
              const borderColors: Record<string, string> = {
                sky: 'text-orange-400',
                purple: 'text-amber-400',
                amber: 'text-emerald-400',
              };

              return (
                <div key={a.id} className="bg-[#181a24] border border-[#262838] p-3 rounded-lg">
                  <div className="flex items-center justify-between mb-1.5">
                    <div>
                      <div className="text-xs font-semibold text-white">{a.name}</div>
                      <div className={`text-[10px] mono ${borderColors[a.color] || 'text-gray-400'}`}>
                        domain: {a.domain} · rank: r={a.rank}
                      </div>
                    </div>
                    <span className="text-sm font-bold mono text-emerald-400">
                      {a.weight.toFixed(2)}
                    </span>
                  </div>

                  <input
                    type="range"
                    min="0"
                    max="1"
                    step="0.05"
                    value={a.weight}
                    onChange={(e) => handleUpdateWeight(a.id, parseFloat(e.target.value))}
                    className="w-full h-1.5 bg-[#0b0c10] rounded-lg appearance-none cursor-pointer accent-orange-500"
                  />
                </div>
              );
            })}
          </div>
        </div>

        {/* Merger Options & Export Destination */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 flex flex-col justify-between shadow-sm">
          <div>
            <div className="text-xs font-bold text-white uppercase tracking-wider mb-3 pb-2 border-b border-[#232530]">
              Blender & Merge Configuration
            </div>

            <div className="space-y-3">
              <div>
                <label className="text-[10px] mono uppercase text-gray-400 font-semibold block mb-1">
                  Merger Algorithm
                </label>
                <select
                  value={mergeMethod}
                  onChange={(e) => setMergeMethod(e.target.value)}
                  className="w-full px-3 py-2 rounded-lg bg-[#0b0c10] border border-[#232530] text-xs mono text-white focus:outline-none focus:border-orange-500 transition"
                >
                  <option>Task Arithmetic</option>
                  <option>TIES-Merging (Trimming & Elect Sign)</option>
                  <option>SLERP (Spherical Linear Interpolation)</option>
                  <option>Uniform Soup (Equal Averaging)</option>
                </select>
              </div>

              <div>
                <label className="text-[10px] mono uppercase text-gray-400 font-semibold block mb-1">
                  Export Format
                </label>
                <select
                  value={exportFormat}
                  onChange={(e) => setExportFormat(e.target.value)}
                  className="w-full px-3 py-2 rounded-lg bg-[#0b0c10] border border-[#232530] text-xs mono text-white focus:outline-none focus:border-orange-500 transition"
                >
                  <option>AWQ 4-bit (SGLang TP=2)</option>
                  <option>GGUF Q4_K_M (Ollama & Llama.cpp)</option>
                  <option>NVFP4 (Native Blackwell / Ada format)</option>
                  <option>Safetensors BF16 (Full Precision Checkpoint)</option>
                </select>
              </div>

              <div>
                <label className="text-[10px] mono uppercase text-gray-400 font-semibold block mb-1">
                  Output Deployment Path
                </label>
                <input
                  type="text"
                  value={outputPath}
                  onChange={(e) => setOutputPath(e.target.value)}
                  className="w-full px-3 py-2 rounded-lg bg-[#0b0c10] border border-[#232530] text-xs mono text-white focus:outline-none focus:border-orange-500 transition"
                />
              </div>
            </div>
          </div>

          {blendResult && (
            <div className="mt-3">
              <pre className="bg-[#0b0c10] border border-orange-500/40 rounded-lg p-3 text-[11px] mono text-orange-200 overflow-x-auto leading-relaxed">
                {blendResult}
              </pre>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
