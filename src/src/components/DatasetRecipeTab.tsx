import React, { useState } from 'react';
import { Database, Play, ArrowRight, CheckCircle2, FileCode, Layers, Sparkles } from 'lucide-react';

export const DatasetRecipeTab: React.FC = () => {
  const [pipelineState, setPipelineState] = useState([
    { type: 'Loader', label: 'SurrealDB Trajectories & Embassy Rust Source', status: 'Complete' },
    { type: 'AST Chunker', label: 'Tree-Sitter Rust Function Scope Pruner', status: 'Complete' },
    { type: 'Synthesizer', label: 'Teacher CoT Reasoner (<think> Generator)', status: 'Complete' },
    { type: 'Unsloth Exporter', label: 'workspace/data/unsloth_grpo_prompts.jsonl', status: 'Ready' },
  ]);

  const [isRunning, setIsRunning] = useState(false);

  const handleRunPipeline = () => {
    setIsRunning(true);
    pipelineState.forEach((_, idx) => {
      setTimeout(() => {
        setPipelineState((prev) =>
          prev.map((n, i) => (i === idx ? { ...n, status: 'Processing' } : n))
        );
      }, idx * 600);

      setTimeout(() => {
        setPipelineState((prev) =>
          prev.map((n, i) => (i === idx ? { ...n, status: 'Complete' } : n))
        );
        if (idx === pipelineState.length - 1) {
          setIsRunning(false);
        }
      }, idx * 600 + 900);
    });
  };

  const sources = [
    { name: '🦀 Embassy Rust AST', desc: 'tree-sitter-rust · 1,240 bare-metal functions', count: '1,240 items', status: 'READY' },
    { name: '🔌 KiCad Schematics', desc: '.kicad_sch S-expressions · 380 power & MCU designs', count: '380 files', status: 'READY' },
    { name: '📐 build123d STEP CAD', desc: '3D parametric solid models · 520 mechanical parts', count: '520 parts', status: 'READY' },
    { name: '📄 PDF Datasheets', desc: 'STM32 / TI component registers & pinouts', count: '1,680 pages', status: 'READY' },
    { name: '🗄️ SurrealDB Trajectories', desc: 'Agent multi-step action logs & tool rollouts', count: '1,000 episodes', status: 'READY' },
  ];

  return (
    <div className="space-y-6 font-sans">
      {/* Header Banner */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(249,115,22,0.1)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-4 border-b border-[#232530]">
          <div>
            <div className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Database className="w-4 h-4 text-orange-400" />
              <span>Phase 6 · Unsloth Dataset Recipe Studio</span>
            </div>
            <div className="text-[10px] mono text-gray-400 mt-0.5">
              Multi-source AST extraction · Teacher CoT reasoning pairs · Unsloth GRPO prompt formatting
            </div>
          </div>
          <button
            onClick={handleRunPipeline}
            disabled={isRunning}
            className="px-4 py-2 rounded-lg bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-gray-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-1.5 cursor-pointer disabled:opacity-50 shadow-[0_0_15px_rgba(249,115,22,0.35)]"
          >
            <Play className="w-3.5 h-3.5 fill-current" />
            <span>{isRunning ? 'Processing Pipeline...' : '▶ Execute Recipe Pipeline'}</span>
          </button>
        </div>

        {/* Pipeline Nodes Flow */}
        <div className="flex items-center gap-2.5 overflow-x-auto pt-4 pb-1">
          {pipelineState.map((node, idx) => (
            <React.Fragment key={idx}>
              <div className="bg-[#181a24] border border-[#262838] rounded-xl p-3.5 min-w-[210px] shrink-0">
                <div className="text-[9px] mono uppercase text-orange-400 font-bold mb-1">
                  {node.type}
                </div>
                <div className="text-xs font-bold text-white leading-snug">{node.label}</div>
                <div className="mt-3">
                  <span
                    className={`text-[9px] mono px-2 py-0.5 rounded font-bold ${
                      node.status === 'Complete'
                        ? 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/30'
                        : node.status === 'Processing'
                        ? 'bg-orange-500/10 text-orange-400 border border-orange-500/30 animate-pulse'
                        : 'bg-[#111217] text-gray-400 border border-[#232530]'
                    }`}
                  >
                    {node.status.toUpperCase()}
                  </span>
                </div>
              </div>
              {idx < pipelineState.length - 1 && (
                <ArrowRight className="w-4 h-4 text-gray-600 shrink-0" />
              )}
            </React.Fragment>
          ))}
        </div>
      </div>

      {/* Sources and Metrics */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Ingestion Sources */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 shadow-sm">
          <div className="text-xs font-bold text-white uppercase tracking-wider mb-3 pb-2 border-b border-[#232530]">
            Ingestion Data Sources
          </div>
          <div className="space-y-2">
            {sources.map((s, idx) => (
              <div
                key={idx}
                className="bg-[#181a24] border border-[#262838] rounded-lg p-3 flex items-center justify-between"
              >
                <div>
                  <div className="text-xs font-semibold text-white">{s.name}</div>
                  <div className="text-[10px] mono text-gray-400">{s.desc}</div>
                </div>
                <div className="text-right">
                  <span className="text-[9px] mono px-2 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 font-bold block mb-0.5">
                    {s.status}
                  </span>
                  <span className="text-[9px] mono text-gray-400">{s.count}</span>
                </div>
              </div>
            ))}
          </div>
        </div>

        {/* Recipe Output Telemetry */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 flex flex-col justify-between shadow-sm">
          <div>
            <div className="text-xs font-bold text-white uppercase tracking-wider mb-3 pb-2 border-b border-[#232530]">
              Recipe Rollout Metrics
            </div>

            <div className="grid grid-cols-2 gap-2.5 mb-3">
              <div className="bg-[#181a24] border border-[#262838] p-3 rounded-lg">
                <div className="text-[10px] mono uppercase text-gray-400 font-semibold">Total Prompts</div>
                <div className="text-xl font-bold mono text-orange-400 mt-0.5">4,820</div>
                <div className="text-[9px] mono text-gray-400 mt-0.5">Bare-metal & CAD tasks</div>
              </div>

              <div className="bg-[#181a24] border border-[#262838] p-3 rounded-lg">
                <div className="text-[10px] mono uppercase text-gray-400 font-semibold">Rollout Pairs</div>
                <div className="text-xl font-bold mono text-emerald-400 mt-0.5">4,820</div>
                <div className="text-[9px] mono text-gray-400 mt-0.5">1:1 Prompt-Rollout match</div>
              </div>

              <div className="bg-[#181a24] border border-[#262838] p-3 rounded-lg">
                <div className="text-[10px] mono uppercase text-gray-400 font-semibold">Avg Token Len</div>
                <div className="text-xl font-bold mono text-amber-400 mt-0.5">1,284</div>
                <div className="text-[9px] mono text-gray-400 mt-0.5">Tokens per rollout trajectory</div>
              </div>

              <div className="bg-[#181a24] border border-[#262838] p-3 rounded-lg">
                <div className="text-[10px] mono uppercase text-gray-400 font-semibold">CoT Coverage</div>
                <div className="text-xl font-bold mono text-emerald-400 mt-0.5">94.2%</div>
                <div className="text-[9px] mono text-gray-400 mt-0.5">&lt;think&gt; reasoning traces</div>
              </div>
            </div>
          </div>

          <div>
            <div className="text-[10px] mono uppercase text-gray-400 font-semibold mb-1">
              Destination Target Path
            </div>
            <div className="p-2.5 rounded-lg bg-[#0b0c10] border border-[#232530] text-xs mono text-orange-300 font-bold">
              workspace/data/unsloth_grpo_prompts.jsonl
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
