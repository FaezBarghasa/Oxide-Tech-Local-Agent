import React, { useState } from 'react';
import { Code2, Play, CheckCircle2, Copy, Check } from 'lucide-react';

export const EndpointsTab: React.FC = () => {
  const [activeEndpoint, setActiveEndpoint] = useState<string | null>(null);
  const [responseView, setResponseView] = useState<string | null>(null);
  const [copied, setCopied] = useState<string | null>(null);

  const trainerEndpoints = [
    { method: 'POST', path: '/api/trainer/jobs', desc: 'Start fine-tuning job with Unsloth FSDP-QDoRA' },
    { method: 'GET', path: '/api/trainer/jobs/{jobId}', desc: 'Fetch job training loss, step & GPU memory metrics' },
    { method: 'DEL', path: '/api/trainer/jobs/{jobId}', desc: 'Abort running training loop and save emergency checkpoint' },
    { method: 'POST', path: '/api/trainer/merge', desc: 'Merge LoRA checkpoint into base AWQ/GGUF model' },
    { method: 'POST', path: '/api/trainer/sweep', desc: 'Execute Optuna hyperparameter optimization sweep' },
    { method: 'POST', path: '/api/trainer/data/preprocess', desc: 'Mojo-accelerated tokenization & batch collation' },
    { method: 'POST', path: '/api/trainer/eval', desc: 'Run LM-Eval-Harness on embedded Rust & KiCad benchmarks' },
    { method: 'POST', path: '/api/trainer/export', desc: 'Export GGUF, AWQ, EXL2, or Safetensors weights' },
    { method: 'POST', path: '/api/trainer/deploy/adapter', desc: 'Hot-swap fine-tuned LoRA into active SGLang server' },
  ];

  const runnerEndpoints = [
    { method: 'POST', path: '/api/runner/sessions', desc: 'Create persistent agent session in SurrealDB' },
    { method: 'GET', path: '/api/runner/sessions/{id}/history', desc: 'Fetch conversation message history and tool trace logs' },
    { method: 'POST', path: '/api/runner/chat', desc: 'Send prompt with SSE token streaming and tool dispatch' },
    { method: 'POST', path: '/api/runner/tools/execute', desc: 'Direct execution of MCP tools within Bubblewrap sandbox' },
    { method: 'PATCH', path: '/api/runner/sessions/{id}/context', desc: 'Update system prompt and inject Tree-Sitter AST' },
    { method: 'GET', path: '/api/runner/health', desc: 'Cluster health, dual GPU VRAM & cache telemetry' },
    { method: 'POST', path: '/api/runner/sessions/{id}/abort', desc: 'Abort current active agent generation trace' },
  ];

  const nexusEndpoints = [
    { method: 'POST', path: '/api/nexus/feedback', desc: 'Submit user & compiler feedback for online DPO alignment' },
    { method: 'POST', path: '/api/nexus/micro-train', desc: 'Trigger 10-step micro-adaptation on failure cases' },
    { method: 'POST', path: '/api/nexus/rag/upsert', desc: 'Upsert code documents with Mojo SIMD embeddings' },
    { method: 'POST', path: '/api/nexus/rag/search', desc: 'Execute semantic similarity search in Qdrant' },
    { method: 'GET', path: '/api/nexus/soups', desc: 'List available model soup checkpoints & test scores' },
  ];

  const handleTestEndpoint = (method: string, path: string) => {
    setActiveEndpoint(`${method} ${path}`);
    setResponseView(
      JSON.stringify(
        {
          status: 'OK',
          code: 200,
          endpoint: path,
          method,
          timestamp: new Date().toISOString(),
          data: {
            success: true,
            cluster: 'Dual RTX 3090 · SGLang TP=2',
            latency_ms: Math.round(18 + Math.random() * 20),
            payload: { message: `Simulated response from ${path}` },
          },
        },
        null,
        2
      )
    );
  };

  const methodColors: Record<string, string> = {
    GET: 'bg-emerald-500/10 text-emerald-400 border-emerald-500/30',
    POST: 'bg-cyan-500/10 text-cyan-400 border-cyan-500/30',
    DEL: 'bg-rose-500/10 text-rose-400 border-rose-500/30',
    PATCH: 'bg-amber-500/10 text-amber-400 border-amber-500/30',
  };

  return (
    <div className="space-y-6">
      {/* Response Drawer if Active */}
      {responseView && (
        <div className="bg-white/5 border border-cyan-500/40 rounded-2xl p-5 backdrop-blur-xl shadow-lg relative overflow-hidden">
          <div className="flex items-center justify-between mb-3 pb-2 border-b border-white/10">
            <span className="text-xs font-bold mono text-cyan-300">
              Live Test Output: {activeEndpoint}
            </span>
            <button
              onClick={() => setResponseView(null)}
              className="text-[10px] mono text-slate-400 hover:text-white transition px-2 py-0.5 rounded-lg bg-white/5 border border-white/10 cursor-pointer"
            >
              Close
            </button>
          </div>
          <pre className="bg-black/50 border border-white/10 rounded-xl p-3.5 text-[11px] mono text-emerald-400 max-h-48 overflow-y-auto leading-relaxed backdrop-blur-md">
            {responseView}
          </pre>
        </div>
      )}

      {/* Trainer API */}
      <div className="bg-white/5 border border-white/10 rounded-2xl overflow-hidden backdrop-blur-xl shadow-md">
        <div className="px-5 py-4 border-b border-white/10 flex items-center justify-between">
          <div>
            <div className="text-xs font-bold text-white uppercase tracking-wider">TrainerAPI · /api/trainer</div>
            <div className="text-[10px] mono text-slate-400">
              Unsloth FSDP-QDoRA & GRPO training lifecycle endpoints
            </div>
          </div>
          <span className="text-[10px] mono px-2.5 py-0.5 rounded-full bg-indigo-500/10 text-indigo-400 border border-indigo-500/30 font-bold">
            9 ROUTES
          </span>
        </div>

        <div className="divide-y divide-white/5">
          {trainerEndpoints.map((ep, idx) => (
            <div
              key={idx}
              onClick={() => handleTestEndpoint(ep.method, ep.path)}
              className="px-5 py-3 flex items-center gap-3.5 hover:bg-white/5 cursor-pointer transition"
            >
              <span
                className={`text-[9px] mono font-bold px-2 py-0.5 rounded border ${
                  methodColors[ep.method]
                }`}
              >
                {ep.method}
              </span>
              <span className="text-xs mono text-slate-200 font-semibold flex-1">{ep.path}</span>
              <span className="text-[11px] text-slate-400 hidden md:inline">{ep.desc}</span>
              <button className="text-[10px] mono font-bold uppercase tracking-wider px-2.5 py-1 rounded-lg bg-white/5 hover:bg-cyan-500 hover:text-slate-950 text-slate-300 border border-white/10 transition cursor-pointer">
                Test
              </button>
            </div>
          ))}
        </div>
      </div>

      {/* Runner API */}
      <div className="bg-white/5 border border-white/10 rounded-2xl overflow-hidden backdrop-blur-xl shadow-md">
        <div className="px-5 py-4 border-b border-white/10 flex items-center justify-between">
          <div>
            <div className="text-xs font-bold text-white uppercase tracking-wider">RunnerAPI · /api/runner</div>
            <div className="text-[10px] mono text-slate-400">
              Rust/SGLang agent execution, context compaction & tool invocation
            </div>
          </div>
          <span className="text-[10px] mono px-2.5 py-0.5 rounded-full bg-cyan-500/10 text-cyan-400 border border-cyan-500/30 font-bold">
            7 ROUTES
          </span>
        </div>

        <div className="divide-y divide-white/5">
          {runnerEndpoints.map((ep, idx) => (
            <div
              key={idx}
              onClick={() => handleTestEndpoint(ep.method, ep.path)}
              className="px-5 py-3 flex items-center gap-3.5 hover:bg-white/5 cursor-pointer transition"
            >
              <span
                className={`text-[9px] mono font-bold px-2 py-0.5 rounded border ${
                  methodColors[ep.method]
                }`}
              >
                {ep.method}
              </span>
              <span className="text-xs mono text-slate-200 font-semibold flex-1">{ep.path}</span>
              <span className="text-[11px] text-slate-400 hidden md:inline">{ep.desc}</span>
              <button className="text-[10px] mono font-bold uppercase tracking-wider px-2.5 py-1 rounded-lg bg-white/5 hover:bg-cyan-500 hover:text-slate-950 text-slate-300 border border-white/10 transition cursor-pointer">
                Test
              </button>
            </div>
          ))}
        </div>
      </div>

      {/* Nexus API */}
      <div className="bg-white/5 border border-white/10 rounded-2xl overflow-hidden backdrop-blur-xl shadow-md">
        <div className="px-5 py-4 border-b border-white/10 flex items-center justify-between">
          <div>
            <div className="text-xs font-bold text-white uppercase tracking-wider">NexusAPI · /api/nexus</div>
            <div className="text-[10px] mono text-slate-400">
              Online DPO learning, Mojo SIMD RAG & Model Soup management
            </div>
          </div>
          <span className="text-[10px] mono px-2.5 py-0.5 rounded-full bg-amber-500/10 text-amber-400 border border-amber-500/30 font-bold">
            5 ROUTES
          </span>
        </div>

        <div className="divide-y divide-white/5">
          {nexusEndpoints.map((ep, idx) => (
            <div
              key={idx}
              onClick={() => handleTestEndpoint(ep.method, ep.path)}
              className="px-5 py-3 flex items-center gap-3.5 hover:bg-white/5 cursor-pointer transition"
            >
              <span
                className={`text-[9px] mono font-bold px-2 py-0.5 rounded border ${
                  methodColors[ep.method]
                }`}
              >
                {ep.method}
              </span>
              <span className="text-xs mono text-slate-200 font-semibold flex-1">{ep.path}</span>
              <span className="text-[11px] text-slate-400 hidden md:inline">{ep.desc}</span>
              <button className="text-[10px] mono font-bold uppercase tracking-wider px-2.5 py-1 rounded-lg bg-white/5 hover:bg-cyan-500 hover:text-slate-950 text-slate-300 border border-white/10 transition cursor-pointer">
                Test
              </button>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
};
