import React, { useState } from 'react';
import { CheckCircle2, XCircle, Play, RefreshCw, AlertCircle, Clock, ShieldCheck, Download, FileCheck } from 'lucide-react';
import { desktop } from '../lib/desktop';

export const VerificationTab: React.FC = () => {
  const [isRunning, setIsRunning] = useState(false);
  const [lastRunTime, setLastRunTime] = useState('Never');
  const [workspacePath, setWorkspacePath] = useState('.');
  const [bundle, setBundle] = useState<any | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [exportPath, setExportPath] = useState('/tmp/oxide_evidence');
  const [exportMsg, setExportMsg] = useState<string | null>(null);

  const handleRunVerification = async () => {
    setIsRunning(true);
    setError(null);
    setExportMsg(null);
    try {
      const res = await desktop.verifierRunSuite({
        workspace_path: workspacePath,
      });
      setBundle(res);
      setLastRunTime(new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }));
    } catch (err: any) {
      setError(err?.message || String(err));
    } finally {
      setIsRunning(false);
    }
  };

  const handleExport = async () => {
    if (!bundle) return;
    try {
      const res = await desktop.verifierExportEvidence(exportPath);
      setExportMsg(`Evidence bundle successfully exported to: ${res}`);
      setTimeout(() => setExportMsg(null), 4000);
    } catch (err: any) {
      setError(err?.message || String(err));
    }
  };

  return (
    <div className="space-y-6 font-sans">
      {/* Header */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(16,185,129,0.1)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 pb-4 border-b border-[#232530]">
          <div>
            <div className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <ShieldCheck className="w-4 h-4 text-emerald-400" />
              <span>Deterministic Verifier Suite & Evidence Bundles</span>
            </div>
            <div className="text-[11px] mono text-gray-400 mt-1">
              Automated compilation, formal proof verification, diff audit & cryptographic evidence signing
            </div>
          </div>
          <div className="flex items-center gap-3">
            <span className="text-[10px] mono text-gray-400">Last verified: {lastRunTime}</span>
            <button
              onClick={handleRunVerification}
              disabled={isRunning}
              className="px-4 py-2 rounded-lg bg-gradient-to-r from-emerald-500 to-teal-500 hover:from-emerald-400 hover:to-teal-400 text-slate-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-2 cursor-pointer disabled:opacity-50 shadow-[0_0_15px_rgba(16,185,129,0.35)]"
            >
              <RefreshCw className={`w-3.5 h-3.5 ${isRunning ? 'animate-spin' : ''}`} />
              <span>{isRunning ? 'Running Verification...' : 'Run Verification Suite'}</span>
            </button>
          </div>
        </div>

        {/* Inputs */}
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 pt-4">
          <div>
            <label className="text-[10px] uppercase tracking-widest text-gray-400 mb-1.5 block">
              Workspace Target Path
            </label>
            <input
              type="text"
              value={workspacePath}
              onChange={(e) => setWorkspacePath(e.target.value)}
              className="w-full bg-[#0c0d12] border border-[#2c2f3d] rounded-lg px-3 py-2 text-xs font-mono text-gray-200 focus:outline-none focus:border-emerald-500/60"
            />
          </div>

          <div>
            <label className="text-[10px] uppercase tracking-widest text-gray-400 mb-1.5 block">
              Evidence Export Directory
            </label>
            <div className="flex gap-2">
              <input
                type="text"
                value={exportPath}
                onChange={(e) => setExportPath(e.target.value)}
                className="w-full bg-[#0c0d12] border border-[#2c2f3d] rounded-lg px-3 py-2 text-xs font-mono text-gray-200 focus:outline-none focus:border-emerald-500/60"
              />
              <button
                onClick={handleExport}
                disabled={!bundle || isRunning}
                className="px-3 py-2 rounded-lg bg-[#1a1c26] hover:bg-[#222532] border border-[#2c2f3d] text-xs font-bold text-gray-200 flex items-center gap-1.5 transition cursor-pointer disabled:opacity-50 shrink-0"
              >
                <Download className="w-3.5 h-3.5 text-emerald-400" />
                <span>Export</span>
              </button>
            </div>
          </div>
        </div>
      </div>

      {exportMsg && (
        <div className="p-4 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-300 text-xs mono flex items-center gap-2">
          <FileCheck className="w-4 h-4 text-emerald-400 shrink-0" />
          <span>{exportMsg}</span>
        </div>
      )}

      {error && (
        <div className="p-4 rounded-xl bg-rose-500/10 border border-rose-500/30 text-rose-300 text-xs mono">
          {error}
        </div>
      )}

      {/* Verification Stages & Report */}
      {bundle && (
        <div className="space-y-6">
          <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl space-y-4">
            <div className="flex items-center justify-between pb-3 border-b border-[#232530]">
              <div>
                <h3 className="text-xs font-bold text-white uppercase tracking-wider">
                  Verification Execution Matrix
                </h3>
                <div className="text-[10px] mono text-gray-400 mt-0.5">
                  Task ID: {bundle.task_id}
                </div>
              </div>
              <span className={`text-[10px] mono px-2.5 py-0.5 rounded font-bold border ${
                bundle.verified_success
                  ? 'bg-emerald-500/10 text-emerald-400 border-emerald-500/30'
                  : 'bg-rose-500/10 text-rose-400 border-rose-500/30'
              }`}>
                {bundle.verified_success ? 'ALL STAGES PASSED' : 'STAGE FAILED'}
              </span>
            </div>

            <div className="divide-y divide-[#232530] overflow-hidden rounded-xl border border-[#232530] bg-[#0c0d12]">
              {bundle.reports?.map((r: any, idx: number) => (
                <div key={idx} className="p-3.5 flex items-center justify-between gap-3 hover:bg-[#14151f] transition">
                  <div className="flex items-center gap-3">
                    {r.passed ? (
                      <CheckCircle2 className="w-4 h-4 text-emerald-400 shrink-0" />
                    ) : (
                      <XCircle className="w-4 h-4 text-rose-400 shrink-0" />
                    )}
                    <div>
                      <div className="text-xs font-bold text-white mono">{r.stage}</div>
                      <div className="text-[10px] mono text-gray-400 mt-0.5">
                        Duration: {r.duration_ms}ms
                      </div>
                    </div>
                  </div>
                  <div className="text-right">
                    <span className="text-[10px] mono text-emerald-400 font-bold">
                      {r.passed ? 'PASS' : 'FAIL'}
                    </span>
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* Git Diff Card */}
          {bundle.git_diff && (
            <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl space-y-3">
              <div className="text-xs font-bold text-white uppercase tracking-wider pb-2 border-b border-[#232530]">
                Workspace Active Git Patch & Unstaged Diff
              </div>
              <pre className="bg-[#0c0d12] border border-[#232530] rounded-xl p-4 text-[11px] mono text-emerald-200/90 overflow-x-auto leading-relaxed max-h-[300px]">
                {bundle.git_diff}
              </pre>
            </div>
          )}
        </div>
      )}
    </div>
  );
};
