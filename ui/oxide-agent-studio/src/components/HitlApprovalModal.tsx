import React, { useState } from 'react';
import { 
  ShieldAlert, 
  CheckCircle2, 
  XCircle, 
  AlertTriangle, 
  Cpu, 
  Terminal, 
  FileCode, 
  Radio, 
  Lock,
  Flame
} from 'lucide-react';

export interface HitlActionRequest {
  id: string;
  timestamp: string;
  title: string;
  subsystem: 'probe-rs' | 'git' | 'eda-kicad' | 'fs-sandbox' | 'qemu-redox';
  impactLevel: 'CRITICAL' | 'HIGH' | 'MEDIUM';
  description: string;
  proposedCommand?: string;
  targetPath?: string;
  parameters: Record<string, string | number | boolean>;
  verificationPassed: boolean;
  status: 'PENDING' | 'APPROVED' | 'REJECTED';
  reviewerScore?: number; // 0.0 - 1.0 (LLM-as-judge / dual reviewer)
  reviewerVerdict?: string;
  confidenceScore?: number; // 0.0 - 1.0
}

interface HitlApprovalModalProps {
  requests: HitlActionRequest[];
  onApprove: (id: string) => void;
  onReject: (id: string, reason?: string) => void;
  onClose?: () => void;
}

export const HitlApprovalModal: React.FC<HitlApprovalModalProps> = ({
  requests,
  onApprove,
  onReject,
}) => {
  const [selectedId, setSelectedId] = useState<string>(requests[0]?.id || '');
  const [rejectReason, setRejectReason] = useState<string>('');

  const currentReq = requests.find((r) => r.id === selectedId) || requests[0];

  if (!requests.length || !currentReq) {
    return null;
  }

  const getSubsystemIcon = (subsystem: HitlActionRequest['subsystem']) => {
    switch (subsystem) {
      case 'probe-rs':
        return <Cpu className="w-5 h-5 text-amber-400" />;
      case 'eda-kicad':
        return <Radio className="w-5 h-5 text-purple-400" />;
      case 'git':
        return <FileCode className="w-5 h-5 text-blue-400" />;
      case 'qemu-redox':
        return <Terminal className="w-5 h-5 text-emerald-400" />;
      default:
        return <AlertTriangle className="w-5 h-5 text-orange-400" />;
    }
  };

  const getImpactBadge = (level: HitlActionRequest['impactLevel']) => {
    switch (level) {
      case 'CRITICAL':
        return (
          <span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-semibold bg-rose-500/20 text-rose-300 border border-rose-500/40 animate-pulse">
            <Flame className="w-3.5 h-3.5" /> CRITICAL IMPACT
          </span>
        );
      case 'HIGH':
        return (
          <span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-semibold bg-amber-500/20 text-amber-300 border border-amber-500/40">
            <AlertTriangle className="w-3.5 h-3.5" /> HIGH RISK
          </span>
        );
      default:
        return (
          <span className="inline-flex items-center gap-1.5 px-2.5 py-0.5 rounded-full text-xs font-semibold bg-cyan-500/20 text-cyan-300 border border-cyan-500/40">
            <Lock className="w-3.5 h-3.5" /> CONTROLLED ACTION
          </span>
        );
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/80 backdrop-blur-md">
      <div className="relative w-full max-w-4xl bg-slate-900 border border-slate-700/80 rounded-2xl shadow-2xl overflow-hidden flex flex-col max-h-[90vh]">
        {/* Modal Header */}
        <div className="px-6 py-4 bg-gradient-to-r from-slate-900 via-slate-800 to-slate-900 border-b border-slate-700/80 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <div className="p-2 rounded-xl bg-amber-500/10 border border-amber-500/30 text-amber-400">
              <ShieldAlert className="w-6 h-6" />
            </div>
            <div>
              <h3 className="text-lg font-bold text-slate-100 flex items-center gap-2">
                Human-in-the-Loop (HITL) Execution Gate
                <span className="text-xs px-2 py-0.5 rounded bg-slate-800 text-slate-400 border border-slate-700">
                  {requests.length} Pending
                </span>
              </h3>
              <p className="text-xs text-slate-400">
                Deterministic agent policy requires explicit operator authorization for high-consequence operations.
              </p>
            </div>
          </div>
          {getImpactBadge(currentReq.impactLevel)}
        </div>

        {/* Modal Body */}
        <div className="flex-1 grid grid-cols-12 divide-x divide-slate-800 overflow-hidden">
          {/* Left: Action List */}
          <div className="col-span-4 bg-slate-950/50 overflow-y-auto p-3 space-y-2">
            <p className="text-[11px] font-semibold tracking-wider text-slate-400 uppercase px-2">
              Action Queue
            </p>
            {requests.map((req) => (
              <button
                key={req.id}
                onClick={() => setSelectedId(req.id)}
                className={`w-full text-left p-3 rounded-xl border transition-all text-xs ${
                  req.id === currentReq.id
                    ? 'bg-slate-800/90 border-cyan-500/50 text-white shadow-lg'
                    : 'bg-slate-900/40 border-slate-800/80 text-slate-400 hover:bg-slate-800/40 hover:text-slate-200'
                }`}
              >
                <div className="flex items-center justify-between mb-1.5">
                  <div className="flex items-center gap-2 font-medium">
                    {getSubsystemIcon(req.subsystem)}
                    <span className="truncate">{req.subsystem}</span>
                  </div>
                  <span className="text-[10px] text-slate-500 font-mono">
                    {req.timestamp}
                  </span>
                </div>
                <p className="text-[11px] font-semibold text-slate-200 truncate">
                  {req.title}
                </p>
              </button>
            ))}
          </div>

          {/* Right: Detailed Inspection */}
          <div className="col-span-8 p-6 overflow-y-auto space-y-5 bg-slate-900/50">
            <div>
              <div className="flex items-center gap-2 mb-1">
                {getSubsystemIcon(currentReq.subsystem)}
                <span className="text-xs font-mono font-medium text-cyan-400">
                  [{currentReq.subsystem.toUpperCase()}]
                </span>
              </div>
              <h4 className="text-base font-bold text-slate-100">
                {currentReq.title}
              </h4>
              <p className="text-xs text-slate-300 mt-1 leading-relaxed">
                {currentReq.description}
              </p>
            </div>

            {/* Proposed Command / Payload */}
            {currentReq.proposedCommand && (
              <div className="space-y-1.5">
                <span className="text-xs font-semibold text-slate-400">
                  Target Execution Command:
                </span>
                <div className="p-3 bg-black/60 border border-slate-800 rounded-xl font-mono text-xs text-emerald-400 break-all select-all">
                  $ {currentReq.proposedCommand}
                </div>
              </div>
            )}

            {/* Execution Parameters */}
            <div className="space-y-2">
              <span className="text-xs font-semibold text-slate-400">
                Parameters & Environment:
              </span>
              <div className="p-3 bg-slate-950/60 border border-slate-800 rounded-xl font-mono text-[11px] text-slate-300 space-y-1">
                {Object.entries(currentReq.parameters).map(([key, val]) => (
                  <div key={key} className="flex justify-between border-b border-slate-800/50 py-1 last:border-none">
                    <span className="text-slate-400">{key}:</span>
                    <span className="text-cyan-300 font-semibold">{String(val)}</span>
                  </div>
                ))}
              </div>
            </div>

            {/* Verification Pre-check Status & Secondary Reviewer Score */}
            <div className="space-y-2">
              <div className={`p-3 rounded-xl border flex items-center gap-3 text-xs ${
                currentReq.verificationPassed 
                  ? 'bg-emerald-950/20 border-emerald-500/30 text-emerald-300' 
                  : 'bg-rose-950/20 border-rose-500/30 text-rose-300'
              }`}>
                {currentReq.verificationPassed ? (
                  <>
                    <CheckCircle2 className="w-5 h-5 text-emerald-400 shrink-0" />
                    <span>Deterministic pre-flight checks and verifier passes (cargo check / DRC clean).</span>
                  </>
                ) : (
                  <>
                    <XCircle className="w-5 h-5 text-rose-400 shrink-0" />
                    <span>Warning: Pre-flight verifier reported potential compilation or DRC warnings.</span>
                  </>
                )}
              </div>

              {(currentReq.reviewerScore !== undefined || currentReq.confidenceScore !== undefined) && (
                <div className="p-3 bg-slate-950/70 border border-slate-800/80 rounded-xl flex items-center justify-between text-xs font-mono">
                  <div className="flex items-center gap-2">
                    <ShieldAlert className="w-4 h-4 text-cyan-400" />
                    <span className="text-slate-400">LLM-as-Judge Confidence:</span>
                    <span className="text-cyan-300 font-bold">
                      {Math.round((currentReq.confidenceScore ?? 0.95) * 100)}%
                    </span>
                  </div>
                  {currentReq.reviewerScore !== undefined && (
                    <div className="flex items-center gap-2">
                      <span className="text-slate-400">Safety Verdict:</span>
                      <span className={`px-2 py-0.5 rounded text-[10px] font-bold ${
                        currentReq.reviewerScore >= 0.8
                          ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/40'
                          : 'bg-amber-500/20 text-amber-300 border border-amber-500/40'
                      }`}>
                        {currentReq.reviewerVerdict || (currentReq.reviewerScore >= 0.8 ? 'PASS' : 'FLAGGED')} ({(currentReq.reviewerScore * 10).toFixed(1)}/10)
                      </span>
                    </div>
                  )}
                </div>
              )}
            </div>

            {/* Reject reason input */}
            <div className="space-y-1.5">
              <label className="text-xs text-slate-400 font-medium">
                Rejection Note / Corrective Directive (optional):
              </label>
              <input
                type="text"
                value={rejectReason}
                onChange={(e) => setRejectReason(e.target.value)}
                placeholder="e.g. Abort flash: Pinout PA9 reconfigured in hardware rev B."
                className="w-full px-3 py-2 bg-slate-950 border border-slate-800 rounded-xl text-xs text-slate-200 placeholder-slate-600 focus:outline-none focus:border-cyan-500"
              />
            </div>
          </div>
        </div>

        {/* Modal Footer Controls */}
        <div className="px-6 py-4 bg-slate-950 border-t border-slate-800 flex items-center justify-between">
          <div className="text-xs text-slate-500 flex items-center gap-2">
            <Lock className="w-3.5 h-3.5" />
            Signed approval will unblock the ReAct DAG loop.
          </div>
          <div className="flex items-center gap-3">
            <button
              onClick={() => {
                onReject(currentReq.id, rejectReason);
                setRejectReason('');
              }}
              className="px-4 py-2 rounded-xl text-xs font-medium text-rose-400 hover:text-rose-300 hover:bg-rose-500/10 border border-rose-500/30 transition-all flex items-center gap-1.5"
            >
              <XCircle className="w-4 h-4" /> Reject Action
            </button>
            <button
              onClick={() => onApprove(currentReq.id)}
              className="px-5 py-2 rounded-xl text-xs font-semibold text-white bg-gradient-to-r from-cyan-600 to-emerald-600 hover:from-cyan-500 hover:to-emerald-500 shadow-lg shadow-cyan-900/30 border border-cyan-400/30 transition-all flex items-center gap-1.5"
            >
              <CheckCircle2 className="w-4 h-4" /> Approve & Execute
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
