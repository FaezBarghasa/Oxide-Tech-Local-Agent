import React, { useState } from 'react';
import { useUI } from '../store/uiStore';
import {
  Database,
  FileCode,
  Flame,
  Brain,
  UploadCloud,
  CheckCircle2,
  Sliders,
  ArrowRight,
  Download,
  Sparkles,
  Zap,
} from 'lucide-react';

export const DatasetRecipeTab: React.FC = () => {
  const { toast } = useUI();
  const [activeTab, setActiveTab] = useState<'formatter' | 'exporter' | 'compactor'>('formatter');

  // Tab 1: Dataset Formatter State
  const [dragActive, setDragActive] = useState(false);
  const [selectedFile, setSelectedFile] = useState<string | null>('telemetry_embedded_episodes.jsonl');
  const [template, setTemplate] = useState<'ChatML' | 'Llama-3' | 'Alpaca'>('ChatML');
  const [isConverting, setIsConverting] = useState(false);
  const [progress, setProgress] = useState(0);

  // Tab 2: GGUF Exporter State
  const [exportModel, setExportModel] = useState('Qwen2.5-Coder-7B-LoRA');
  const [exportQuant, setExportQuant] = useState<'Q4_K_M' | 'Q5_K_M' | 'Q8_0'>('Q4_K_M');
  const [isExporting, setIsExporting] = useState(false);
  const [exportProgress, setExportProgress] = useState(0);

  // Tab 3: Context Compactor State
  const [autoCompact, setAutoCompact] = useState(true);
  const [compactThreshold, setCompactThreshold] = useState(80);

  const handleConvert = () => {
    setIsConverting(true);
    setProgress(0);
    const timer = setInterval(() => {
      setProgress((p) => {
        if (p >= 100) {
          clearInterval(timer);
          setIsConverting(false);
          toast(`Converted dataset using ${template} template!`);
          return 100;
        }
        return p + 20;
      });
    }, 250);
  };

  const handleExportGGUF = () => {
    setIsExporting(true);
    setExportProgress(0);
    const timer = setInterval(() => {
      setExportProgress((p) => {
        if (p >= 100) {
          clearInterval(timer);
          setIsExporting(false);
          toast(`Exported ${exportModel} to GGUF (${exportQuant})`);
          return 100;
        }
        return p + 25;
      });
    }, 300);
  };

  return (
    <div className="space-y-6 max-w-5xl font-sans">
      {/* Header & Sub-navigation */}
      <div className="bg-[#111113] border border-[#27272A] rounded-xl p-6 shadow-lg">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-[#27272A]">
          <div>
            <h2 className="text-base font-bold text-[#FAFAFA] flex items-center gap-2">
              <Database className="w-5 h-5 text-[#8B5CF6]" />
              Tooling Suite & Data Operations
            </h2>
            <p className="text-xs text-[#A1A1AA] mt-1 font-mono">
              Unsloth data preparation · GGUF export compilation · Context compression
            </p>
          </div>

          {/* Sub-tabs */}
          <div className="flex items-center gap-1.5 bg-[#18181b] p-1 rounded-lg border border-[#27272A]">
            <button
              onClick={() => setActiveTab('formatter')}
              className={`px-3 py-1.5 rounded-md text-xs font-medium transition cursor-pointer ${
                activeTab === 'formatter'
                  ? 'bg-[#8B5CF6] text-white shadow-sm'
                  : 'text-zinc-400 hover:text-white'
              }`}
            >
              Dataset Formatter
            </button>
            <button
              onClick={() => setActiveTab('exporter')}
              className={`px-3 py-1.5 rounded-md text-xs font-medium transition cursor-pointer ${
                activeTab === 'exporter'
                  ? 'bg-[#8B5CF6] text-white shadow-sm'
                  : 'text-zinc-400 hover:text-white'
              }`}
            >
              GGUF Exporter
            </button>
            <button
              onClick={() => setActiveTab('compactor')}
              className={`px-3 py-1.5 rounded-md text-xs font-medium transition cursor-pointer ${
                activeTab === 'compactor'
                  ? 'bg-[#8B5CF6] text-white shadow-sm'
                  : 'text-zinc-400 hover:text-white'
              }`}
            >
              Context Compactor
            </button>
          </div>
        </div>

        {/* Tab 1: Dataset Formatter */}
        {activeTab === 'formatter' && (
          <div className="pt-6 space-y-6">
            {/* Drag & Drop Zone */}
            <div
              onDragOver={(e) => {
                e.preventDefault();
                setDragActive(true);
              }}
              onDragLeave={() => setDragActive(false)}
              onDrop={(e) => {
                e.preventDefault();
                setDragActive(false);
                if (e.dataTransfer.files[0]) {
                  setSelectedFile(e.dataTransfer.files[0].name);
                  toast(`Loaded file: ${e.dataTransfer.files[0].name}`);
                }
              }}
              className={`border-2 border-dashed rounded-xl p-8 text-center transition-colors cursor-pointer flex flex-col items-center justify-center gap-3 ${
                dragActive
                  ? 'border-[#8B5CF6] bg-[#8B5CF6]/10'
                  : 'border-[#27272A] bg-[#18181b]/50 hover:border-zinc-500'
              }`}
            >
              <div className="w-12 h-12 rounded-full bg-[#8B5CF6]/10 border border-[#8B5CF6]/30 flex items-center justify-center">
                <UploadCloud className="w-6 h-6 text-[#8B5CF6]" />
              </div>
              <div>
                <div className="text-sm font-semibold text-[#FAFAFA]">
                  {selectedFile ? `Selected: ${selectedFile}` : 'Drag & drop your .jsonl or .csv dataset here'}
                </div>
                <div className="text-xs text-[#A1A1AA] mt-1 font-mono">
                  Supports ShareGPT, OpenAI format, and raw instruction-response pairs
                </div>
              </div>
            </div>

            {/* Template Selector & Action */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 items-end">
              <div className="space-y-1.5">
                <label className="text-xs font-mono uppercase text-zinc-400 font-semibold">
                  Target Prompt Template
                </label>
                <select
                  value={template}
                  onChange={(e) => setTemplate(e.target.value as any)}
                  className="w-full bg-[#18181b] border border-[#27272A] rounded-lg px-3 py-2.5 text-xs text-[#FAFAFA] font-mono outline-none cursor-pointer focus:border-[#8B5CF6]"
                >
                  <option value="ChatML">ChatML (&lt;|im_start|&gt;system...)</option>
                  <option value="Llama-3">Llama-3 (&lt;|start_header_id|&gt;...)</option>
                  <option value="Alpaca">Alpaca (### Instruction:...)</option>
                </select>
              </div>

              <button
                onClick={handleConvert}
                disabled={isConverting}
                className="w-full py-2.5 rounded-lg bg-[#FAFAFA] text-black text-xs font-bold uppercase tracking-wider hover:bg-white hover:-translate-y-px active:translate-y-0 transition cursor-pointer disabled:opacity-50 flex items-center justify-center gap-2"
              >
                <Sparkles className="w-4 h-4 fill-current" />
                {isConverting ? `Converting (${progress}%)…` : 'Convert & Optimize Dataset'}
              </button>
            </div>

            {/* Progress Bar */}
            {isConverting && (
              <div className="space-y-1.5">
                <div className="flex justify-between text-xs font-mono text-zinc-400">
                  <span>Formatting & Deduplication Progress</span>
                  <span>{progress}%</span>
                </div>
                <div className="w-full bg-[#18181b] h-2 rounded-full overflow-hidden border border-[#27272A]">
                  <div
                    className="bg-[#10B981] h-full transition-all duration-300"
                    style={{ width: `${progress}%` }}
                  />
                </div>
              </div>
            )}
          </div>
        )}

        {/* Tab 2: GGUF Exporter */}
        {activeTab === 'exporter' && (
          <div className="pt-6 space-y-6">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="space-y-1.5">
                <label className="text-xs font-mono uppercase text-zinc-400 font-semibold">
                  Source Fine-Tuned Model
                </label>
                <select
                  value={exportModel}
                  onChange={(e) => setExportModel(e.target.value)}
                  className="w-full bg-[#18181b] border border-[#27272A] rounded-lg px-3 py-2.5 text-xs text-[#FAFAFA] font-mono outline-none cursor-pointer focus:border-[#8B5CF6]"
                >
                  <option value="Qwen2.5-Coder-7B-LoRA">Qwen2.5-Coder-7B-LoRA (Embassy Firmware)</option>
                  <option value="Llama-3.3-8B-KiCad">Llama-3.3-8B-KiCad (PCB Layout Expert)</option>
                  <option value="DeepSeek-R1-Distill-8B">DeepSeek-R1-Distill-8B (STAIR Reasoner)</option>
                </select>
              </div>

              <div className="space-y-1.5">
                <label className="text-xs font-mono uppercase text-zinc-400 font-semibold">
                  Target Quantization Format
                </label>
                <select
                  value={exportQuant}
                  onChange={(e) => setExportQuant(e.target.value as any)}
                  className="w-full bg-[#18181b] border border-[#27272A] rounded-lg px-3 py-2.5 text-xs text-[#FAFAFA] font-mono outline-none cursor-pointer focus:border-[#8B5CF6]"
                >
                  <option value="Q4_K_M">Q4_K_M (Recommended 4-bit medium)</option>
                  <option value="Q5_K_M">Q5_K_M (High accuracy 5-bit)</option>
                  <option value="Q8_0">Q8_0 (Near-lossless 8-bit)</option>
                </select>
              </div>
            </div>

            <div className="rounded-lg bg-[#18181b] border border-[#27272A] p-4 flex items-center justify-between">
              <div>
                <div className="text-xs font-bold text-[#FAFAFA]">Target Destination</div>
                <div className="text-xs font-mono text-[#10B981] mt-0.5">
                  workspace/models/{exportModel.toLowerCase()}-{exportQuant.toLowerCase()}.gguf
                </div>
              </div>
              <button
                onClick={handleExportGGUF}
                disabled={isExporting}
                className="px-5 py-2.5 rounded-lg bg-[#FAFAFA] text-black text-xs font-bold hover:bg-white transition hover:-translate-y-px active:translate-y-0 disabled:opacity-50 cursor-pointer flex items-center gap-2"
              >
                <Download className="w-4 h-4" />
                {isExporting ? `Exporting (${exportProgress}%)…` : 'Export GGUF'}
              </button>
            </div>

            {isExporting && (
              <div className="space-y-1.5">
                <div className="flex justify-between text-xs font-mono text-zinc-400">
                  <span>Weight Quantization & Export Progress</span>
                  <span>{exportProgress}%</span>
                </div>
                <div className="w-full bg-[#18181b] h-2 rounded-full overflow-hidden border border-[#27272A]">
                  <div
                    className="bg-[#8B5CF6] h-full transition-all duration-300"
                    style={{ width: `${exportProgress}%` }}
                  />
                </div>
              </div>
            )}
          </div>
        )}

        {/* Tab 3: Context Compactor */}
        {activeTab === 'compactor' && (
          <div className="pt-6 space-y-6">
            <div className="rounded-lg bg-[#18181b] border border-[#27272A] p-5 flex items-center justify-between">
              <div>
                <div className="text-sm font-bold text-[#FAFAFA]">Auto-Compacting Memory (STAIR Engine)</div>
                <div className="text-xs text-[#A1A1AA] mt-1">
                  Automatically condenses long conversation contexts into structured AST summaries when approaching token limit.
                </div>
              </div>
              <button
                role="switch"
                aria-checked={autoCompact}
                onClick={() => {
                  setAutoCompact(!autoCompact);
                  toast(`Auto-compaction ${!autoCompact ? 'Enabled' : 'Disabled'}`);
                }}
                className={`w-12 h-6 rounded-full transition-colors relative cursor-pointer ${
                  autoCompact ? 'bg-[#10B981]' : 'bg-[#27272A]'
                }`}
              >
                <span
                  className={`absolute top-0.5 w-5 h-5 rounded-full bg-white transition-all shadow-md ${
                    autoCompact ? 'left-6.5' : 'left-0.5'
                  }`}
                />
              </button>
            </div>

            {/* Trigger Threshold Slider */}
            <div className="space-y-3 bg-[#18181b] border border-[#27272A] rounded-lg p-5">
              <div className="flex justify-between text-xs font-mono">
                <span className="text-zinc-400 font-semibold">Compaction Trigger Threshold</span>
                <span className="text-[#10B981] font-bold">{compactThreshold}% of context window</span>
              </div>
              <input
                type="range"
                min="50"
                max="95"
                step="5"
                value={compactThreshold}
                onChange={(e) => setCompactThreshold(parseInt(e.target.value))}
                className="w-full accent-[#10B981] bg-[#111113] h-2 rounded-lg cursor-pointer"
              />
              <div className="flex justify-between text-[10px] font-mono text-zinc-500">
                <span>Conservative (50%)</span>
                <span>Balanced (80%)</span>
                <span>Aggressive (95%)</span>
              </div>
            </div>

            <div className="p-4 rounded-lg bg-[#10B981]/10 border border-[#10B981]/30 flex items-center gap-3">
              <CheckCircle2 className="w-5 h-5 text-[#10B981] shrink-0" />
              <div className="text-xs font-mono text-zinc-200">
                Memanto semantic cache active. Token savings: <b>64.2%</b> across multi-turn sessions.
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
};
