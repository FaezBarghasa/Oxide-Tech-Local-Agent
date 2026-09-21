import React, { useState, useEffect } from 'react';
import { motion, AnimatePresence } from 'motion/react';
import { useUI } from '../store/uiStore';
import { Cpu, X, Zap, HardDrive, CheckCircle2, Sliders, ShieldCheck } from 'lucide-react';

const ENGINES = [
  { id: 'Candle', name: 'Candle (Native Rust)', desc: 'Zero-overhead, pure Rust CUDA/Metal execution' },
  { id: 'llama.cpp', name: 'llama.cpp (GGUF)', desc: 'Optimized CPU/GPU hybrid quant memory runner' },
  { id: 'Mistral.rs', name: 'Mistral.rs (ISQ)', desc: 'In-situ quantization & speculative decoding' },
  { id: 'vLLM', name: 'vLLM (Docker Sidecar)', desc: 'PagedAttention high-concurrency batch engine' },
  { id: 'SGLang', name: 'SGLang (RadixAttention)', desc: 'Ultra-fast multi-turn prefix caching' },
] as const;

const QUANTS = [
  { id: 'Q4_K_M', name: 'Q4_K_M (4-bit Balanced)', vramMul: 0.45, speed: 'Fastest' },
  { id: 'Q5_K_M', name: 'Q5_K_M (5-bit Higher Quality)', vramMul: 0.58, speed: 'High' },
  { id: 'Q8_0', name: 'Q8_0 (8-bit High Fidelity)', vramMul: 0.85, speed: 'Moderate' },
  { id: 'FP16', name: 'FP16 (16-bit Full Precision)', vramMul: 1.6, speed: 'Heavy' },
] as const;

export function DeploySlideOver() {
  const { deployModel, setDeployModel, toast } = useUI();
  const [engine, setEngine] = useState<string>('Candle');
  const [quant, setQuant] = useState<string>('Q4_K_M');
  const [contextLen, setContextLen] = useState<number>(8192);
  const [isDeploying, setIsDeploying] = useState(false);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape' && deployModel) {
        setDeployModel(null);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [deployModel, setDeployModel]);

  // Model size estimation base in GB
  const getBaseSize = (name: string | null): number => {
    if (!name) return 8;
    if (name.includes('70B') || name.includes('70b')) return 40;
    if (name.includes('32B') || name.includes('32b')) return 20;
    if (name.includes('14B') || name.includes('14b')) return 10;
    if (name.includes('8B') || name.includes('7B') || name.includes('8b') || name.includes('7b')) return 6;
    if (name.includes('3B') || name.includes('1.5B')) return 2.5;
    return 8;
  };

  const selectedQuantObj = QUANTS.find((q) => q.id === quant) || QUANTS[0];
  const baseSize = getBaseSize(deployModel);
  const estimatedVram = (baseSize * selectedQuantObj.vramMul + (contextLen / 8192) * 1.2).toFixed(1);

  const handleDeploy = () => {
    setIsDeploying(true);
    setTimeout(() => {
      setIsDeploying(false);
      toast(`Deployed ${deployModel || 'Model'} on ${engine} (${quant})`);
      setDeployModel(null);
    }, 800);
  };

  return (
    <AnimatePresence>
      {deployModel && (
        <>
          {/* Backdrop */}
          <motion.div
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            exit={{ opacity: 0 }}
            className="fixed inset-0 z-[60] bg-black/60 backdrop-blur-sm"
            onClick={() => setDeployModel(null)}
          />

          {/* Slide-over panel */}
          <motion.aside
            initial={{ x: 440 }}
            animate={{ x: 0 }}
            exit={{ x: 440 }}
            transition={{ type: 'spring', damping: 28, stiffness: 280 }}
            role="dialog"
            aria-modal="true"
            aria-label={`Deploy ${deployModel}`}
            className="fixed right-0 top-0 bottom-0 z-[61] w-[420px] max-w-[94vw] bg-[#111113] border-l border-[#27272A] shadow-2xl p-6 flex flex-col justify-between overflow-y-auto font-sans scrollbar-thin"
          >
            <div className="space-y-6">
              {/* Header */}
              <div className="flex items-start justify-between pb-4 border-b border-[#27272A]">
                <div>
                  <div className="flex items-center gap-2">
                    <h2 className="text-base font-bold text-[#FAFAFA]">{deployModel}</h2>
                    <span className="text-[10px] font-mono px-2 py-0.5 rounded-full bg-[#10B981]/15 text-[#10B981] border border-[#10B981]/30">
                      Ready
                    </span>
                  </div>
                  <p className="text-xs text-[#A1A1AA] mt-1">
                    Select execution engine, quantization level, and hardware profile.
                  </p>
                </div>
                <button
                  onClick={() => setDeployModel(null)}
                  className="p-1 rounded-md text-zinc-400 hover:text-white hover:bg-[#18181b] transition cursor-pointer"
                >
                  <X className="w-4 h-4" />
                </button>
              </div>

              {/* Engine Selector */}
              <div className="space-y-2">
                <label className="text-[11px] font-mono uppercase tracking-wider text-zinc-400 font-bold flex items-center justify-between">
                  <span>Execution Engine</span>
                  <span className="text-zinc-500 lowercase">hardware backend</span>
                </label>
                <div className="space-y-1.5" role="radiogroup">
                  {ENGINES.map((e) => {
                    const active = engine === e.id;
                    return (
                      <button
                        key={e.id}
                        role="radio"
                        aria-checked={active}
                        onClick={() => setEngine(e.id)}
                        className={`w-full p-3 rounded-lg border text-left transition flex items-center justify-between cursor-pointer ${
                          active
                            ? 'bg-[#10B981]/10 border-[#10B981]/40 text-[#FAFAFA]'
                            : 'bg-[#18181b] border-[#27272A] text-zinc-300 hover:border-zinc-500'
                        }`}
                      >
                        <div>
                          <div className="text-xs font-semibold flex items-center gap-1.5">
                            {e.name}
                            {active && <CheckCircle2 className="w-3.5 h-3.5 text-[#10B981]" />}
                          </div>
                          <div className="text-[10px] text-zinc-400 mt-0.5">{e.desc}</div>
                        </div>
                      </button>
                    );
                  })}
                </div>
              </div>

              {/* Quantization Picker */}
              <div className="space-y-2">
                <label className="text-[11px] font-mono uppercase tracking-wider text-zinc-400 font-bold flex items-center justify-between">
                  <span>Quantization Format</span>
                  <span className="text-zinc-500 lowercase">precision</span>
                </label>
                <div className="grid grid-cols-2 gap-2" role="radiogroup">
                  {QUANTS.map((q) => {
                    const active = quant === q.id;
                    return (
                      <button
                        key={q.id}
                        role="radio"
                        aria-checked={active}
                        onClick={() => setQuant(q.id)}
                        className={`p-2.5 rounded-lg border text-left transition cursor-pointer ${
                          active
                            ? 'bg-[#8B5CF6]/15 border-[#8B5CF6]/50 text-[#FAFAFA]'
                            : 'bg-[#18181b] border-[#27272A] text-zinc-300 hover:border-zinc-500'
                        }`}
                      >
                        <div className="text-xs font-semibold">{q.id}</div>
                        <div className="text-[10px] text-zinc-400 mt-0.5 font-mono">{q.speed}</div>
                      </button>
                    );
                  })}
                </div>
              </div>

              {/* Context Length Slider */}
              <div className="space-y-2">
                <div className="flex items-center justify-between text-xs font-mono text-zinc-400">
                  <span>Context Window:</span>
                  <span className="text-[#FAFAFA] font-bold">{contextLen.toLocaleString()} tokens</span>
                </div>
                <input
                  type="range"
                  min="2048"
                  max="32768"
                  step="2048"
                  value={contextLen}
                  onChange={(e) => setContextLen(Number(e.target.value))}
                  className="w-full accent-[#10B981] bg-[#18181b] h-1.5 rounded-lg cursor-pointer"
                />
              </div>

              {/* Dynamic VRAM Estimator Badge */}
              <div className="rounded-lg bg-[#18181b] border border-[#27272A] p-3.5 space-y-1">
                <div className="flex items-center justify-between text-xs font-mono">
                  <span className="text-zinc-400">Estimated VRAM:</span>
                  <span className="text-[#10B981] font-bold text-sm">{estimatedVram} GB</span>
                </div>
                <div className="text-[10px] font-mono text-zinc-500">
                  Target: 24GB VRAM GPU · Headroom: {(24 - Number(estimatedVram)).toFixed(1)} GB
                </div>
              </div>
            </div>

            {/* Action Buttons */}
            <div className="pt-6 space-y-2 border-t border-[#27272A]">
              <button
                onClick={handleDeploy}
                disabled={isDeploying}
                className="w-full py-3 rounded-lg bg-[#FAFAFA] text-black text-xs font-bold uppercase tracking-wider hover:bg-white hover:-translate-y-px transition active:translate-y-0 disabled:opacity-50 cursor-pointer shadow-lg flex items-center justify-center gap-2"
              >
                <Zap className="w-4 h-4 fill-current" />
                {isDeploying ? 'Deploying Model to GPU…' : 'Deploy Model'}
              </button>
              <button
                onClick={() => setDeployModel(null)}
                className="w-full py-2 text-xs text-zinc-400 hover:text-zinc-200 transition cursor-pointer"
              >
                Cancel (Esc)
              </button>
            </div>
          </motion.aside>
        </>
      )}
    </AnimatePresence>
  );
}
