import React, { useState } from 'react';
import { Binary, Upload, Play, Cpu, ShieldAlert, Code2, Copy, Check, FileCode, Flame } from 'lucide-react';
import { desktop } from '../lib/desktop';

export const ReForgeTab: React.FC = () => {
  const [filePath, setFilePath] = useState('/home/jrad/RustroverProjects/Oxide-Tech-Local-Agent/target/release/oxide-tech-local-agent');
  const [arch, setArch] = useState('auto');
  const [decompile, setDecompile] = useState(true);
  const [loading, setLoading] = useState(false);
  const [result, setResult] = useState<any | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [copiedCode, setCopiedCode] = useState(false);

  const handleAnalyze = async () => {
    setLoading(true);
    setError(null);
    try {
      if (desktop.isDesktop) {
        const res = await desktop.reforgeAnalyzeFile({
          file_path: filePath,
          arch,
          decompile,
        });
        setResult(res);
      } else {
        // Mock fallback for web mode
        setTimeout(() => {
          setResult({
            domain: 'Native Executable / Shared Object',
            file_size: 4289120,
            format: 'ELF 64-bit LSB pie executable, x86-64',
            entry_point: '0x0004a210',
            arm_vector_table: null,
            rtos: null,
            avg_entropy: 6.42,
            entropy_chunks: [
              { offset: 0, entropy: 4.12 },
              { offset: 4096, entropy: 5.89 },
              { offset: 8192, entropy: 6.74 },
              { offset: 12288, entropy: 7.82 },
            ],
            ptx_analysis: null,
            functions: [
              {
                name: 're_forge::analyzer::BinaryAnalyzer::analyze_file',
                start_address: '0x0005b100',
                instructions: [
                  { address: '0x0005b100', mnemonic: 'push rbp', length: 1, is_call: false, is_branch: false, is_return: false },
                  { address: '0x0005b101', mnemonic: 'mov rbp, rsp', length: 3, is_call: false, is_branch: false, is_return: false },
                  { address: '0x0005b104', mnemonic: 'sub rsp, 0x80', length: 4, is_call: false, is_branch: false, is_return: false },
                  { address: '0x0005b108', mnemonic: 'call 0x0004f800', length: 5, is_call: true, is_branch: false, is_return: false },
                ]
              }
            ],
            total_instructions: 18452,
            decompiled_code: `// ── Recovered Function: re_forge::analyzer::BinaryAnalyzer::analyze_file ──
pub fn analyze_file(path: &Path) -> Result<BinaryAnalyzer, Box<dyn std::error::Error>> {
    // Neural AST reconstruction from x86_64 ELF basic blocks
    let raw_bytes = std::fs::read(path)?;
    let parsed = goblin::Object::parse(&raw_bytes)?;
    Ok(BinaryAnalyzer::from_goblin(parsed))
}`,
          });
          setLoading(false);
        }, 500);
      }
    } catch (err: any) {
      setError(err?.message || String(err));
    } finally {
      if (desktop.isDesktop) {
        setLoading(false);
      }
    }
  };

  const copyDecompiled = () => {
    if (result?.decompiled_code) {
      navigator.clipboard.writeText(result.decompiled_code);
      setCopiedCode(true);
      setTimeout(() => setCopiedCode(false), 2000);
    }
  };

  return (
    <div className="space-y-6 font-sans">
      {/* Header & Inputs */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(6,182,212,0.1)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-[#232530]">
          <div>
            <div className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Binary className="w-4 h-4 text-cyan-400" />
              <span>RE-Forge Studio · Binary & GPU Reverse Engineering</span>
            </div>
            <div className="text-[11px] mono text-gray-400 mt-1">
              ARM Cortex-M IVT, RTOS detection, Shannon entropy, CUDA PTX lifter & Safe-Rust decompilation
            </div>
          </div>
          <button
            onClick={handleAnalyze}
            disabled={loading || !filePath.trim()}
            className="px-4 py-2 rounded-lg bg-gradient-to-r from-cyan-500 to-blue-500 hover:from-cyan-400 hover:to-blue-400 text-slate-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-2 cursor-pointer disabled:opacity-50 shadow-[0_0_15px_rgba(6,182,212,0.35)]"
          >
            <Play className={`w-3.5 h-3.5 ${loading ? 'animate-spin' : ''}`} />
            <span>{loading ? 'Analyzing Binary...' : 'Analyze & Decompile'}</span>
          </button>
        </div>

        {/* Form Controls */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 pt-4">
          <div className="lg:col-span-2">
            <label className="text-[10px] uppercase tracking-widest text-gray-400 mb-1.5 block">
              Target Binary, Firmware (.bin/.hex) or PTX File Path
            </label>
            <div className="flex gap-2">
              <input
                type="text"
                value={filePath}
                onChange={(e) => setFilePath(e.target.value)}
                placeholder="/path/to/binary.elf"
                className="w-full bg-[#0c0d12] border border-[#2c2f3d] rounded-lg px-3 py-2 text-xs font-mono text-cyan-200 placeholder:text-gray-600 focus:outline-none focus:border-cyan-500/60"
              />
            </div>
          </div>

          <div className="flex items-end gap-3">
            <div className="flex-1">
              <label className="text-[10px] uppercase tracking-widest text-gray-400 mb-1.5 block">
                Target Architecture
              </label>
              <select
                value={arch}
                onChange={(e) => setArch(e.target.value)}
                className="w-full bg-[#0c0d12] border border-[#2c2f3d] rounded-lg px-3 py-2 text-xs font-mono text-gray-200 focus:outline-none focus:border-cyan-500/60 cursor-pointer"
              >
                <option value="auto">Auto-Detect</option>
                <option value="arm">ARM Cortex-M (Embedded)</option>
                <option value="x86_64">x86_64 Native (ELF/PE)</option>
                <option value="cuda">NVIDIA CUDA PTX</option>
              </select>
            </div>

            <label className="flex items-center gap-2 pb-2 text-[11px] font-mono text-gray-300 cursor-pointer">
              <input
                type="checkbox"
                checked={decompile}
                onChange={(e) => setDecompile(e.target.checked)}
                className="accent-cyan-500"
              />
              Decompile
            </label>
          </div>
        </div>
      </div>

      {error && (
        <div className="p-4 rounded-xl bg-rose-500/10 border border-rose-500/30 text-rose-300 text-xs mono">
          {error}
        </div>
      )}

      {/* Analysis Results View */}
      {result && (
        <div className="space-y-6">
          {/* Metadata Cards */}
          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-4">
            <div className="bg-[#111217] border border-[#232530] rounded-xl p-4">
              <div className="text-[10px] uppercase tracking-wider text-gray-400">Target Domain</div>
              <div className="text-xs font-bold text-cyan-300 mt-1">{result.domain}</div>
              <div className="text-[10px] mono text-gray-400 mt-0.5">{result.format}</div>
            </div>

            <div className="bg-[#111217] border border-[#232530] rounded-xl p-4">
              <div className="text-[10px] uppercase tracking-wider text-gray-400">Binary Image Size</div>
              <div className="text-sm font-bold text-white mono mt-1">{(result.file_size / 1024).toFixed(1)} KB</div>
              <div className="text-[10px] mono text-gray-400 mt-0.5">{result.file_size} bytes</div>
            </div>

            <div className="bg-[#111217] border border-[#232530] rounded-xl p-4">
              <div className="text-[10px] uppercase tracking-wider text-gray-400">Entry Point</div>
              <div className="text-sm font-bold text-emerald-400 mono mt-1">{result.entry_point || 'N/A'}</div>
              <div className="text-[10px] mono text-gray-400 mt-0.5">Start vector address</div>
            </div>

            <div className="bg-[#111217] border border-[#232530] rounded-xl p-4">
              <div className="text-[10px] uppercase tracking-wider text-gray-400">Avg Shannon Entropy</div>
              <div className="text-sm font-bold text-amber-400 mono mt-1">{result.avg_entropy ? `${result.avg_entropy.toFixed(2)} / 8.0` : 'N/A'}</div>
              <div className="text-[10px] mono text-gray-400 mt-0.5">{result.entropy_chunks?.length || 0} scanned 4KB blocks</div>
            </div>
          </div>

          {/* ARM Vector Table & RTOS (if firmware) */}
          {result.arm_vector_table && (
            <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl space-y-4">
              <div className="flex items-center justify-between pb-3 border-b border-[#232530]">
                <h3 className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2">
                  <Cpu className="w-4 h-4 text-cyan-400" />
                  <span>ARM Cortex-M Interrupt Vector Table & RTOS Signatures</span>
                </h3>
                {result.rtos?.detected_rtos && (
                  <span className="text-[10px] mono px-2.5 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 font-bold">
                    RTOS: {result.rtos.detected_rtos} ({(result.rtos.confidence * 100).toFixed(0)}%)
                  </span>
                )}
              </div>

              <div className="grid grid-cols-2 sm:grid-cols-4 gap-3">
                <div className="bg-[#0c0d12] border border-[#232530] p-3 rounded-lg">
                  <div className="text-[9px] uppercase tracking-wider text-gray-400">Initial SP</div>
                  <div className="text-xs font-bold mono text-cyan-300 mt-0.5">{result.arm_vector_table.initial_sp}</div>
                </div>
                <div className="bg-[#0c0d12] border border-[#232530] p-3 rounded-lg">
                  <div className="text-[9px] uppercase tracking-wider text-gray-400">Reset Handler</div>
                  <div className="text-xs font-bold mono text-emerald-400 mt-0.5">{result.arm_vector_table.reset_handler}</div>
                </div>
                <div className="bg-[#0c0d12] border border-[#232530] p-3 rounded-lg">
                  <div className="text-[9px] uppercase tracking-wider text-gray-400">HardFault</div>
                  <div className="text-xs font-bold mono text-rose-400 mt-0.5">{result.arm_vector_table.hardfault_handler}</div>
                </div>
                <div className="bg-[#0c0d12] border border-[#232530] p-3 rounded-lg">
                  <div className="text-[9px] uppercase tracking-wider text-gray-400">SysTick</div>
                  <div className="text-xs font-bold mono text-amber-400 mt-0.5">{result.arm_vector_table.systick_handler}</div>
                </div>
              </div>
            </div>
          )}

          {/* CUDA PTX Analysis (if GPU) */}
          {result.ptx_analysis && (
            <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl space-y-4">
              <div className="flex items-center justify-between pb-3 border-b border-[#232530]">
                <h3 className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2">
                  <Flame className="w-4 h-4 text-orange-400" />
                  <span>CUDA GPU PTX Kernel & Tensor Core Pattern Inspector</span>
                </h3>
                <span className="text-[10px] mono text-cyan-400">{result.ptx_analysis.target_arch}</span>
              </div>

              <div className="grid grid-cols-1 sm:grid-cols-3 gap-3">
                <div className="bg-[#0c0d12] border border-[#232530] p-3 rounded-lg">
                  <div className="text-[9px] uppercase tracking-wider text-gray-400">Kernel Name</div>
                  <div className="text-xs font-bold mono text-orange-300 mt-0.5 truncate">{result.ptx_analysis.kernel_name}</div>
                </div>
                <div className="bg-[#0c0d12] border border-[#232530] p-3 rounded-lg">
                  <div className="text-[9px] uppercase tracking-wider text-gray-400">Shared Memory</div>
                  <div className="text-xs font-bold mono text-white mt-0.5">{result.ptx_analysis.shared_memory_bytes} bytes</div>
                </div>
                <div className="bg-[#0c0d12] border border-[#232530] p-3 rounded-lg">
                  <div className="text-[9px] uppercase tracking-wider text-gray-400">Async Copy (cp.async)</div>
                  <div className="text-xs font-bold mono text-emerald-400 mt-0.5">{result.ptx_analysis.uses_async_copy ? 'Enabled' : 'Disabled'}</div>
                </div>
              </div>
            </div>
          )}

          {/* Decompiled Safe Rust Output */}
          {result.decompiled_code && (
            <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl space-y-3">
              <div className="flex items-center justify-between pb-3 border-b border-[#232530]">
                <h3 className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2">
                  <Code2 className="w-4 h-4 text-cyan-400" />
                  <span>Neural Safe-Rust Decompiler Reconstruction</span>
                </h3>
                <button
                  onClick={copyDecompiled}
                  className="px-3 py-1 rounded bg-[#1c1e2a] hover:bg-[#252838] border border-[#2e3244] text-[10px] mono text-cyan-300 flex items-center gap-1.5 transition cursor-pointer"
                >
                  {copiedCode ? <Check className="w-3 h-3 text-emerald-400" /> : <Copy className="w-3 h-3" />}
                  <span>{copiedCode ? 'Copied' : 'Copy Rust Code'}</span>
                </button>
              </div>

              <pre className="bg-[#0c0d12] border border-[#232530] rounded-xl p-4 text-[11px] mono text-cyan-100 overflow-x-auto leading-relaxed max-h-[500px]">
                {result.decompiled_code}
              </pre>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
