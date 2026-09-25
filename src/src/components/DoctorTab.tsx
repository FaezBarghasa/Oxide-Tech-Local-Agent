import React, { useState, useEffect, useCallback } from 'react';
import {
  Settings,
  Save,
  RefreshCw,
  Sliders,
  CheckCircle2,
  AlertCircle,
  FileText,
  Server,
  Cpu,
  HardDrive,
  Wrench,
  X,
} from 'lucide-react';
import { desktop } from '../lib/desktop';
import { DoctorResult } from '../types';

export const DoctorTab: React.FC = () => {
  const [loading, setLoading] = useState(false);
  const [doctorResult, setDoctorResult] = useState<DoctorResult | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [udevMsg, setUdevMsg] = useState<string | null>(null);

  const runDiagnostics = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await desktop.doctorRunDiagnostics();
      setDoctorResult(result);
    } catch (err: any) {
      setError(err?.message || String(err));
    } finally {
      setLoading(false);
    }
  }, []);

  const installUdev = useCallback(async () => {
    try {
      if (desktop.isDesktop) {
        await desktop.doctorInstallUdevRules();
        setUdevMsg('udev rules installed successfully.');
        setTimeout(() => setUdevMsg(null), 3000);
      }
    } catch (err: any) {
      setUdevMsg(`Error: ${err?.message || String(err)}`);
    }
  }, []);

  useEffect(() => {
    runDiagnostics();
  }, []);

  const result = doctorResult || {
    passed: 0,
    warnings: 0,
    failed: 0,
    timestamp: new Date().toISOString(),
    checks: [],
    nvidiaGpu: { passed: false, version: '', error: null },
    summary: '',
  };

  return (
    <div className="space-y-6 font-sans">
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(16,185,129,0.06)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-[#232530]">
          <div>
            <h2 className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Settings className="w-4 h-4 text-emerald-400" />
              System Diagnostics
            </h2>
            <p className="text-[11px] mono text-gray-400 mt-1">
              Verified environment scan · {result.timestamp ? new Date(result.timestamp).toLocaleString() : 'Pending'}
            </p>
          </div>
          <div className="flex items-center gap-3">
            <button
              onClick={runDiagnostics}
              disabled={loading}
              className="px-4 py-2 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-2 cursor-pointer disabled:opacity-50"
            >
              <RefreshCw className={`w-3 h-3 ${loading ? 'animate-spin' : ''}`} />
              {loading ? 'Diagnosing...' : 'Run Diagnostics'}
            </button>
          </div>
        </div>

        <div className="grid grid-cols-2 sm:grid-cols-4 gap-3 mt-4">
          <div className="bg-[#161822] border border-[#242738] rounded-xl p-4 flex items-center gap-3">
            <div className="w-9 h-9 rounded-lg bg-emerald-500/10 border border-emerald-500/20 flex items-center justify-center text-emerald-400 font-bold">
              <CheckCircle2 className="w-4 h-4" />
            </div>
            <div>
              <div className="text-lg font-bold text-white mono">{result.passed}</div>
              <div className="text-[10px] uppercase tracking-wider text-gray-400">Passed</div>
            </div>
          </div>
          <div className="bg-[#161822] border border-[#242738] rounded-xl p-4 flex items-center gap-3">
            <div className="w-9 h-9 rounded-lg bg-amber-500/10 border border-amber-500/20 flex items-center justify-center text-amber-400 font-bold">
              <Sliders className="w-4 h-4" />
            </div>
            <div>
              <div className="text-lg font-bold text-white mono">{result.warnings}</div>
              <div className="text-[10px] uppercase tracking-wider text-gray-400">Warnings</div>
            </div>
          </div>
          <div className="bg-[#161822] border border-[#242738] rounded-xl p-4 flex items-center gap-3">
            <div className="w-9 h-9 rounded-lg bg-rose-500/10 border border-rose-500/20 flex items-center justify-center text-rose-400 font-bold">
              <X className="w-4 h-4" />
            </div>
            <div>
              <div className="text-lg font-bold text-white mono">{result.failed}</div>
              <div className="text-[10px] uppercase tracking-wider text-gray-400">Critical Failures</div>
            </div>
          </div>
          <div className="bg-[#161822] border border-[#242738] rounded-xl p-4 flex items-center gap-3">
            <div className="w-9 h-9 rounded-lg bg-cyan-500/10 border border-cyan-500/20 flex items-center justify-center text-cyan-400 font-bold">
              <Server className="w-4 h-4" />
            </div>
            <div>
              <div className="text-lg font-bold text-white mono">
                {result.nvidiaGpu?.passed ? 'GPU' : 'CPU'}
              </div>
              <div className="text-[10px] uppercase tracking-wider text-gray-400">Acceleration</div>
            </div>
          </div>
        </div>
      </div>

      {error && (
        <div className="p-4 rounded-xl bg-rose-500/10 border border-rose-500/30 text-rose-300 text-xs mono flex items-center gap-2">
          <AlertCircle className="w-3.5 h-3.5 shrink-0" />
          {error}
        </div>
      )}

      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl space-y-4">
        <div className="flex items-center justify-between pb-3 border-b border-[#232530]">
          <h3 className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2">
            <Wrench className="w-3.5 h-3.5 text-zinc-400" />
            Toolchain Verification
          </h3>
          {udevMsg && (
            <span className="text-[10px] font-mono text-emerald-400">{udevMsg}</span>
          )}
        </div>

        <div className="divide-y divide-[#232530] overflow-hidden rounded-xl border border-[#232530] bg-[#0c0d12]">
          {result.checks?.length ? (
            result.checks.map((check) => (
              <div key={check.name} className="flex items-center gap-3 p-3 hover:bg-[#161822] transition">
                <div className="w-8 h-8 rounded-lg bg-[#18181e] flex items-center justify-center shrink-0">
                  {check.passed ? (
                    <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400" />
                  ) : (
                    <AlertCircle className="w-3.5 h-3.5 text-rose-400" />
                  )}
                </div>
                <div className="flex-1 min-w-0">
                  <div className="text-xs font-semibold text-white flex items-center gap-2">
                    {check.name}
                    {check.required && (
                      <span className="text-[9px] font-mono text-rose-400 bg-rose-500/10 px-1.5 py-0.5 rounded">REQUIRED</span>
                    )}
                  </div>
                  <div className="text-[10px] mono text-gray-400 mt-0.5">{check.version || '—'}</div>
                  {check.error && (
                    <div className="text-[10px] mono text-rose-400 mt-0.5 max-w-md truncate">{check.error}</div>
                  )}
                </div>
              </div>
            ))
          ) : (
            <div className="p-8 text-center text-zinc-500 text-xs">Run diagnostics to see results</div>
          )}
        </div>

        <div className="flex items-center justify-between mt-4 pt-4 border-t border-[#232530]">
          <div className="flex items-center gap-2">
            <HardDrive className="w-3.5 h-3.5 text-zinc-500" />
            <span className="text-[10px] font-mono text-zinc-500">udev rules</span>
          </div>
          <button
            onClick={installUdev}
            className="px-2.5 py-1 rounded bg-[#18181e] hover:bg-[#202330] border border-[#2c2f3d] text-[10px] mono text-gray-300 flex items-center gap-1.5 transition cursor-pointer"
          >
            <Wrench className="w-3 h-3" />
            Install udev
          </button>
        </div>
      </div>
    </div>
  );
};
