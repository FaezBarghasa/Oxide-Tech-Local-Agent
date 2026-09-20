import React, { useState, useEffect, useCallback } from 'react';
import { desktop, ModelInfo, ModelListResponse } from '../lib/desktop';
import { Cpu, RefreshCw, ChevronDown, Check, Sliders, HardDrive, Sparkles, Server } from 'lucide-react';

interface ModelSelectorProps {
  selectedModel: string;
  selectedProvider: string;
  onModelChange: (model: ModelInfo) => void;
  temperature: number;
  onTemperatureChange: (temp: number) => void;
  maxTokens: number;
  onMaxTokensChange: (tokens: number) => void;
}

export const ModelSelector: React.FC<ModelSelectorProps> = ({
  selectedModel,
  selectedProvider,
  onModelChange,
  temperature,
  onTemperatureChange,
  maxTokens,
  onMaxTokensChange,
}) => {
  const [isOpen, setIsOpen] = useState(false);
  const [isParamsOpen, setIsParamsOpen] = useState(false);
  const [modelData, setModelData] = useState<ModelListResponse | null>(null);
  const [loading, setLoading] = useState(false);

  const fetchModels = useCallback(async () => {
    setLoading(true);
    try {
      const data = await desktop.modelListAvailable();
      setModelData(data);
      if (!selectedModel && data.models.length > 0) {
        const defaultMod = data.models.find((m) => m.is_running) || data.models[0];
        onModelChange(defaultMod);
      }
    } catch {
      // Fallback
    } finally {
      setLoading(false);
    }
  }, [selectedModel, onModelChange]);

  useEffect(() => {
    fetchModels();
  }, [fetchModels]);

  const activeModelObj = modelData?.models.find(
    (m) => m.name === selectedModel || m.id === selectedModel
  );

  return (
    <div className="flex items-center gap-2 relative">
      {/* Model Trigger Button */}
      <div className="relative">
        <button
          onClick={() => setIsOpen(!isOpen)}
          className="flex items-center gap-2 px-3 py-1.5 rounded-lg bg-[#141418] hover:bg-[#1a1a20] border border-white/[0.08] hover:border-amber-500/[0.3] text-xs transition cursor-pointer"
        >
          <div className="flex items-center gap-1.5">
            {selectedProvider === 'local_gguf' ? (
              <HardDrive className="w-3.5 h-3.5 text-emerald-400" />
            ) : selectedProvider === 'ollama' ? (
              <Cpu className="w-3.5 h-3.5 text-amber-400" />
            ) : selectedProvider === 'sglang' ? (
              <Server className="w-3.5 h-3.5 text-cyan-400" />
            ) : (
              <Sparkles className="w-3.5 h-3.5 text-purple-400" />
            )}
            <span className="font-mono font-medium text-white max-w-[180px] truncate">
              {activeModelObj?.name || selectedModel || 'Select Model'}
            </span>
          </div>

          <span className="text-[10px] px-1.5 py-0.5 rounded bg-white/[0.05] text-zinc-400 font-mono">
            {activeModelObj?.size_formatted || selectedProvider}
          </span>

          <ChevronDown className="w-3 h-3 text-zinc-400" />
        </button>

        {/* Dropdown Menu */}
        {isOpen && (
          <>
            <div className="fixed inset-0 z-40" onClick={() => setIsOpen(false)} />
            <div className="absolute top-full mt-1.5 left-0 w-80 max-h-96 overflow-y-auto rounded-xl bg-[#121216] border border-white/[0.12] shadow-2xl p-2 z-50 backdrop-blur-xl">
              <div className="flex items-center justify-between px-2 py-1.5 border-b border-white/[0.06] mb-1">
                <span className="text-[11px] font-semibold text-zinc-300 font-mono">
                  Available Models ({modelData?.models.length || 0})
                </span>
                <button
                  onClick={fetchModels}
                  disabled={loading}
                  className="p-1 rounded hover:bg-white/[0.06] text-zinc-400 hover:text-white transition cursor-pointer"
                  title="Rescan ~/models and Ollama"
                >
                  <RefreshCw className={`w-3 h-3 ${loading ? 'animate-spin' : ''}`} />
                </button>
              </div>

              {modelData?.models.map((m) => {
                const isSelected = m.name === selectedModel || m.id === selectedModel;
                return (
                  <div
                    key={m.id}
                    onClick={() => {
                      onModelChange(m);
                      setIsOpen(false);
                    }}
                    className={`flex items-start justify-between p-2 rounded-lg cursor-pointer transition ${
                      isSelected
                        ? 'bg-amber-500/[0.12] border border-amber-500/[0.3]'
                        : 'hover:bg-white/[0.04]'
                    }`}
                  >
                    <div className="min-w-0 pr-2">
                      <div className="flex items-center gap-1.5">
                        <span className="text-xs font-medium text-white truncate">{m.name}</span>
                        {m.is_running && (
                          <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
                        )}
                      </div>
                      <p className="text-[10px] text-zinc-400 truncate mt-0.5">{m.description}</p>
                    </div>

                    <div className="flex flex-col items-end shrink-0 gap-1">
                      <span
                        className={`text-[9px] px-1.5 py-0.5 rounded font-mono ${
                          m.provider === 'local_gguf'
                            ? 'bg-emerald-500/[0.15] text-emerald-300 border border-emerald-500/[0.2]'
                            : m.provider === 'ollama'
                            ? 'bg-amber-500/[0.15] text-amber-300 border border-amber-500/[0.2]'
                            : 'bg-zinc-800 text-zinc-300'
                        }`}
                      >
                        {m.size_formatted}
                      </span>
                      {isSelected && <Check className="w-3 h-3 text-amber-400" />}
                    </div>
                  </div>
                );
              })}

              <div className="mt-2 pt-2 border-t border-white/[0.06] px-2 text-[10px] text-zinc-400 font-mono">
                💡 Tip: Drop `.gguf` models into <code className="text-amber-300">~/models/</code> to run instantly.
              </div>
            </div>
          </>
        )}
      </div>

      {/* Model Hyperparameters Slider Popover */}
      <div className="relative">
        <button
          onClick={() => setIsParamsOpen(!isParamsOpen)}
          className="p-1.5 rounded-lg bg-[#141418] hover:bg-[#1a1a20] border border-white/[0.08] hover:border-amber-500/[0.3] text-zinc-400 hover:text-white transition cursor-pointer"
          title="Inference Hyperparameters"
        >
          <Sliders className="w-3.5 h-3.5" />
        </button>

        {isParamsOpen && (
          <>
            <div className="fixed inset-0 z-40" onClick={() => setIsParamsOpen(false)} />
            <div className="absolute top-full mt-1.5 right-0 w-72 rounded-xl bg-[#121216] border border-white/[0.12] shadow-2xl p-3 z-50 backdrop-blur-xl">
              <h4 className="text-xs font-semibold text-white mb-3 font-mono flex items-center justify-between">
                <span>Model Tuning</span>
                <span className="text-[10px] text-amber-400/80 font-normal">Unsloth-Style</span>
              </h4>

              <div className="space-y-3 text-xs">
                <div>
                  <div className="flex justify-between text-zinc-400 text-[11px] mb-1">
                    <span>Temperature</span>
                    <span className="font-mono text-white">{temperature}</span>
                  </div>
                  <input
                    type="range"
                    min="0"
                    max="1"
                    step="0.05"
                    value={temperature}
                    onChange={(e) => onTemperatureChange(parseFloat(e.target.value))}
                    className="w-full accent-amber-400 h-1 bg-zinc-700 rounded-lg cursor-pointer"
                  />
                </div>

                <div>
                  <div className="flex justify-between text-zinc-400 text-[11px] mb-1">
                    <span>Max Output Tokens</span>
                    <span className="font-mono text-white">{maxTokens}</span>
                  </div>
                  <div className="grid grid-cols-4 gap-1 text-[10px] font-mono">
                    {[2048, 4096, 8192, 16384].map((tok) => (
                      <button
                        key={tok}
                        onClick={() => onMaxTokensChange(tok)}
                        className={`py-1 rounded border text-center transition cursor-pointer ${
                          maxTokens === tok
                            ? 'bg-amber-500/[0.15] border-amber-500/40 text-amber-300 font-bold'
                            : 'bg-white/[0.04] border-white/[0.06] text-zinc-400 hover:text-white'
                        }`}
                      >
                        {tok >= 1000 ? `${tok / 1024}k` : tok}
                      </button>
                    ))}
                  </div>
                </div>
              </div>
            </div>
          </>
        )}
      </div>
    </div>
  );
};
