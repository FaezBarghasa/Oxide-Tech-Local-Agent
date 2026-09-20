import React, { useState, useEffect } from 'react';
import { Stethoscope, RefreshCw, CheckCircle2, XCircle, AlertTriangle, ShieldCheck, Terminal, Cpu } from 'lucide-react';
import { desktop } from '../lib/desktop';
import { DoctorResult, DiagnosticCheck } from '../types';

export const DoctorTab: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [installingUdev, setInstallingUdev] = useState(false);
  const [udevMsg, setUdevMsg] = useState<string | null>(null);
  const [doctorResult, setDoctorResult] = useState<DoctorResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  const runDiagnostics = async () => {
    setLoading(true);
    setError(null);
    try {
      if (desktop.isDesktop) {
        const res = await desktop.doctorRunDiagnostics();
        setDoctorResult(res);
      } else {
        // Mock fallback for browser dev mode
        setTimeout(() => {
          setDoctorResult({
            passed: 10,
            warnings: 2,
            failed: 0,
            ready: true,
            timestamp: new Date().toLocaleTimeString(),
            checks: [
              { name: 'Rust Compiler', command: 'rustc', required: true, passed: true, version: 'rustc 1.85.0', error: null },
              { name: 'Cargo', command: 'cargo', required: true, passed: true, version: 'cargo 1.85.0', error: null },
              { name: 'Node.js', command: 'node', required: true, passed: true, version: 'v22.14.0', error: null },
              { name: 'pnpm', command: 'pnpm', required: false, passed: true, version: '9.15.0', error: null },
              { name: 'Bubblewrap Sandbox', command: 'bwrap', required: true, passed: true, version: '0.9.0', error: null },
              { name: 'Git', command: 'git', required: true, passed: true, version: '2.43.0', error: null },
              { name: 'probe-rs (STM32/ARM)', command: 'probe-rs', required: false, passed: true, version: '0.24.0', error: null },
              { name: 'QEMU x86_64', command: 'qemu-system-x86_64', required: false, passed: true, version: '8.2.2', error: null },
              { name: 'KiCad CLI (EDA)', command: 'kicad-cli', required: false, passed: true, version: '8.0.0', error: null },
              { name: 'Ngspice (Electrical Sim)', command: 'ngspice', required: false, passed: true, version: '42', error: null },
              { name: 'Ollama (Local LLM)', command: 'ollama', required: false, passed: true, version: '0.5.4', error: null },
              { name: 'NVIDIA GPU Hardware Acceleration', command: 'nvidia-smi', required: false, passed: true, version: 'NVIDIA RTX 3090, 24576 MiB', error: null },
              { name: 'Hardware Debugger udev Rules', command: '/etc/udev/rules.d/99-probe-rs.rules', required: false, passed: true, version: 'present', error: null },
            ]
          });
          setLoading(false);
        }, 600);
      }
    } catch (err: any) {
      setError(err?.message || String(err));
    } finally {
      if (desktop.isDesktop) {
        setLoading(false);
      }
    }
  };

  const handleInstallUdev = async () => {
    setInstallingUdev(true);
    setUdevMsg(null);
    try {
      if (desktop.isDesktop) {
        const msg = await desktop.doctorInstallUdevRules();
        setUdevMsg(msg);
        await runDiagnostics();
      } else {
        setUdevMsg('udev installer requires desktop native mode with pkexec.');
      }
    } catch (err: any) {
      setUdevMsg(`Error: ${err?.message || String(err)}`);
    } finally {
      setInstallingUdev(false);
    }
  };

  useEffect(() => {
    runDiagnostics();
  }, []);

  return (
    <div className="space-y-6 font-sans">
      {/* Header & Control Banner */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(16,185,129,0.1)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-[#232530]">
          <div>
            <div className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Stethoscope className="w-4 h-4 text-emerald-400" />
              <span>System & Environment Diagnostics · Doctor</span>
            </div>
            <div className="text-[11px] mono text-gray-400 mt-1">
              Deterministic verification of compilers, sandboxes, hardware probes & GPU accelerators
            </div>
          </div>
          <div className="flex items-center gap-3">
            <button
              onClick={runDiagnostics}
              disabled={loading}
              className="px-4 py-2 rounded-lg bg-emerald-500 hover:bg-emerald-400 text-slate-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-2 cursor-pointer disabled:opacity-50 shadow-[0_0_15px_rgba(16,185,129,0.35)]"
            >
              <RefreshCw className={`w-3.5 h-3.5 ${loading ? 'animate-spin' : ''}`} />
              <span>{loading ? 'Diagnosing...' : 'Run Diagnostics'}</span>
            </button>
          </div>
        </div>

        {/* Scorecard */}
        {doctorResult && (
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 pt-5">
            <div className="bg-[#161822] border border-[#242738] rounded-xl p-4 flex items-center gap-3">
              <div className="w-9 h-9 rounded-lg bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-400 font-bold">
                <CheckCircle2 className="w-5 h-5" />
              </div>
              <div>
                <div className="text-lg font-bold text-white mono">{doctorResult.passed}</div>
                <div className="text-[10px] uppercase tracking-wider text-gray-400">Passed Checks</div>
              </div>
            </div>

            <div className="bg-[#161822] border border-[#242738] rounded-xl p-4 flex items-center gap-3">
              <div className="w-9 h-9 rounded-lg bg-amber-500/10 border border-amber-500/20 flex items-center justify-center text-amber-400 font-bold">
                <AlertTriangle className="w-5 h-5" />
              </div>
              <div>
                <div className="text-lg font-bold text-white mono">{doctorResult.warnings}</div>
                <div className="text-[10px] uppercase tracking-wider text-gray-400">Optional Warnings</div>
              </div>
            </div>

            <div className="bg-[#161822] border border-[#242738] rounded-xl p-4 flex items-center gap-3">
              <div className="w-9 h-9 rounded-lg bg-rose-500/10 border border-rose-500/20 flex items-center justify-center text-rose-400 font-bold">
                <XCircle className="w-5 h-5" />
              </div>
              <div>
                <div className="text-lg font-bold text-white mono">{doctorResult.failed}</div>
                <div className="text-[10px] uppercase tracking-wider text-gray-400">Critical Failures</div>
              </div>
            </div>
          </div>
        )}
      </div>

      {error && (
        <div className="p-4 rounded-xl bg-rose-500/10 border border-rose-500/30 text-rose-300 text-xs mono">
          {error}
        </div>
      )}

      {/* Diagnostics Table */}
      {doctorResult && (
        <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl space-y-4">
          <div className="flex items-center justify-between pb-3 border-b border-[#232530]">
            <h3 className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Terminal className="w-4 h-4 text-emerald-400" />
              <span>Toolchain & Hardware Check Matrix</span>
            </h3>
            <span className="text-[10px] mono text-gray-400">
              {doctorResult.checks.length} Subsystems Inspected
            </span>
          </div>

          <div className="divide-y divide-[#232530] overflow-hidden rounded-xl border border-[#232530] bg-[#0c0d12]">
            {doctorResult.checks.map((check: DiagnosticCheck, idx: number) => (
              <div key={idx} className="p-3.5 flex flex-col sm:flex-row sm:items-center justify-between gap-2 hover:bg-[#14151f] transition">
                <div className="flex items-center gap-3">
                  {check.passed ? (
                    <span className="text-emerald-400 shrink-0">
                      <CheckCircle2 className="w-4 h-4" />
                    </span>
                  ) : check.required ? (
                    <span className="text-rose-400 shrink-0">
                      <XCircle className="w-4 h-4" />
                    </span>
                  ) : (
                    <span className="text-amber-400 shrink-0">
                      <AlertTriangle className="w-4 h-4" />
                    </span>
                  )}
                  <div>
                    <div className="text-xs font-semibold text-white flex items-center gap-2">
                      {check.name}
                      {check.required && (
                        <span className="text-[8px] mono px-1.5 py-0.2 rounded bg-rose-500/10 text-rose-400 border border-rose-500/20 uppercase font-bold">
                          Required
                        </span>
                      )}
                    </div>
                    <div className="text-[10px] mono text-gray-400 mt-0.5">
                      Command: <span className="text-gray-300">{check.command}</span>
                    </div>
                  </div>
                </div>

                <div className="flex items-center gap-3 self-end sm:self-center">
                  <div className="text-right">
                    <div className="text-[11px] mono text-emerald-400/90 font-medium">
                      {check.version}
                    </div>
                    {check.error && (
                      <div className="text-[10px] mono text-rose-400 max-w-md truncate">
                        {check.error}
                      </div>
                    )}
                  </div>
                  {check.name.includes('udev') && !check.passed && (
                    <button
                      onClick={handleInstallUdev}
                      disabled={installingUdev}
                      className="px-2.5 py-1 rounded bg-orange-500 hover:bg-orange-400 text-slate-950 text-[10px] font-bold uppercase transition cursor-pointer disabled:opacity-50"
                    >
                      {installingUdev ? 'Installing...' : 'Install udev'}
                    </button>
                  )}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Udev Actions Footer */}
      {udevMsg && (
        <div className="p-4 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-300 text-xs mono">
          {udevMsg}
        </div>
      )}
    </div>
  );
};
