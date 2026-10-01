import React, { useState, useEffect, useCallback } from 'react';
import {
  Download,
  RefreshCw,
  ShieldCheck,
  CheckCircle2,
  AlertTriangle,
  Zap,
  X,
  Sparkles,
  ArrowRight,
  HardDrive
} from 'lucide-react';
import {
  updaterCheck,
  updaterDownloadAndApply,
  updaterRestart,
  onUpdateProgress,
  UpdateCheckResponse,
  UpdateProgress,
  UpdateStage
} from '../lib/desktop';

interface UpdateModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const UpdateModal: React.FC<UpdateModalProps> = ({ isOpen, onClose }) => {
  const [checking, setChecking] = useState(false);
  const [downloading, setDownloading] = useState(false);
  const [updateInfo, setUpdateInfo] = useState<UpdateCheckResponse | null>(null);
  const [progress, setProgress] = useState<UpdateProgress | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [statusMsg, setStatusMsg] = useState<string | null>(null);
  const [readyToRestart, setReadyToRestart] = useState(false);

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    const setupListener = async () => {
      unlisten = await onUpdateProgress((p) => {
        setProgress(p);
        if (p.stage === 'ReadyToRestart') {
          setReadyToRestart(true);
          setDownloading(false);
        }
      });
    };

    if (isOpen) {
      setupListener();
      handleCheck();
    }

    return () => {
      if (unlisten) unlisten();
    };
  }, [isOpen]);

  const handleCheck = useCallback(async () => {
    setChecking(true);
    setError(null);
    setStatusMsg(null);
    try {
      const res = await updaterCheck();
      setUpdateInfo(res);
      if (!res.update_available) {
        setStatusMsg(`Oxide-Tech Studio is up to date (v${res.current_version}).`);
      }
    } catch (err: any) {
      setError(err?.message || String(err));
    } finally {
      setChecking(false);
    }
  }, []);

  const handleStartUpdate = useCallback(async () => {
    if (!updateInfo?.platform_release) {
      setError('No compatible binary release found for your platform.');
      return;
    }

    setDownloading(true);
    setError(null);
    try {
      const rel = updateInfo.platform_release;
      const res = await updaterDownloadAndApply(rel.url, rel.blake3, rel.size_bytes);
      if (res.success) {
        setReadyToRestart(true);
      }
    } catch (err: any) {
      setError(err?.message || String(err));
    } finally {
      setDownloading(false);
    }
  }, [updateInfo]);

  const handleRelaunch = useCallback(async () => {
    try {
      await updaterRestart();
    } catch (err: any) {
      setError(err?.message || String(err));
    }
  }, []);

  if (!isOpen) return null;

  const getStageLabel = (stage?: UpdateStage): string => {
    if (!stage) return 'Idle';
    if (typeof stage === 'string') {
      switch (stage) {
        case 'Checking':
          return 'Verifying Release Attestation...';
        case 'Downloading':
          return 'Streaming Chunked Payload...';
        case 'VerifyingSignature':
          return 'Validating BLAKE3 Cryptographic Hash...';
        case 'ApplyingPayload':
          return 'Executing Atomic Executable Swap...';
        case 'ReadyToRestart':
          return 'Update Ready for Hot Restart!';
        default:
          return stage;
      }
    }
    return `Error: ${(stage as any).Failed || 'Unknown'}`;
  };

  const percent =
    progress && progress.total_bytes > 0
      ? Math.min(100, Math.round((progress.bytes_downloaded / progress.total_bytes) * 100))
      : 0;

  const downloadedMb = progress ? (progress.bytes_downloaded / (1024 * 1024)).toFixed(1) : '0.0';
  const totalMb = progress ? (progress.total_bytes / (1024 * 1024)).toFixed(1) : '0.0';
  const speedMb = progress ? (progress.speed_bytes_per_sec / (1024 * 1024)).toFixed(2) : '0.00';

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-md p-4">
      <div className="bg-[#111318] border border-cyan-500/30 rounded-2xl w-full max-w-2xl shadow-2xl overflow-hidden flex flex-col max-h-[90vh]">
        {/* Header */}
        <div className="flex items-center justify-between px-6 py-4 border-b border-gray-800 bg-[#161922]">
          <div className="flex items-center gap-3">
            <div className="p-2 rounded-lg bg-cyan-500/10 text-cyan-400 border border-cyan-500/20">
              <Zap className="w-5 h-5 animate-pulse" />
            </div>
            <div>
              <h2 className="text-lg font-bold text-white flex items-center gap-2">
                Oxide-Tech In-App Auto-Update Core
              </h2>
              <p className="text-xs text-gray-400">
                Ground-truth Ed25519 attestation & atomic in-place binary swap
              </p>
            </div>
          </div>
          <button
            onClick={onClose}
            className="p-1.5 text-gray-400 hover:text-white rounded-lg hover:bg-gray-800 transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Content Body */}
        <div className="p-6 space-y-6 overflow-y-auto flex-1">
          {/* Version Pill Info */}
          <div className="grid grid-cols-2 gap-4">
            <div className="p-4 rounded-xl bg-gray-900/60 border border-gray-800 flex flex-col justify-between">
              <span className="text-xs text-gray-500 uppercase tracking-wider font-semibold">
                Installed Version
              </span>
              <span className="text-lg font-mono font-bold text-gray-200 mt-1">
                v{updateInfo?.current_version || '0.5.0'}
              </span>
              <span className="text-[11px] text-emerald-400 flex items-center gap-1 mt-2">
                <CheckCircle2 className="w-3.5 h-3.5" /> Single-Binary Native
              </span>
            </div>

            <div className="p-4 rounded-xl bg-gray-900/60 border border-cyan-500/20 flex flex-col justify-between">
              <span className="text-xs text-gray-500 uppercase tracking-wider font-semibold">
                Latest Remote Attestation
              </span>
              <span className="text-lg font-mono font-bold text-cyan-400 mt-1">
                {updateInfo?.target_version ? `v${updateInfo.target_version}` : 'Checking...'}
              </span>
              <span className="text-[11px] text-gray-400 flex items-center gap-1 mt-2">
                <ShieldCheck className="w-3.5 h-3.5 text-cyan-400" /> Ed25519 Cryptographic Attestation
              </span>
            </div>
          </div>

          {/* Feedback & Errors */}
          {error && (
            <div className="p-4 rounded-xl bg-red-500/10 border border-red-500/30 text-red-300 text-sm flex items-start gap-3">
              <AlertTriangle className="w-5 h-5 text-red-400 shrink-0 mt-0.5" />
              <div>
                <p className="font-semibold">Update Check Notice</p>
                <p className="text-xs text-red-300/80 mt-1 font-mono">{error}</p>
              </div>
            </div>
          )}

          {statusMsg && !error && (
            <div className="p-4 rounded-xl bg-emerald-500/10 border border-emerald-500/30 text-emerald-300 text-sm flex items-center gap-3">
              <CheckCircle2 className="w-5 h-5 text-emerald-400 shrink-0" />
              <span>{statusMsg}</span>
            </div>
          )}

          {/* Changelog Section */}
          {updateInfo?.changelog && (
            <div className="space-y-2">
              <div className="flex items-center justify-between">
                <span className="text-xs font-semibold text-gray-300 flex items-center gap-1.5">
                  <Sparkles className="w-3.5 h-3.5 text-amber-400" /> Release Changelog (v{updateInfo.target_version})
                </span>
                {updateInfo.min_os_version && (
                  <span className="text-[11px] font-mono text-gray-500">
                    Min OS: {updateInfo.min_os_version}
                  </span>
                )}
              </div>
              <div className="p-4 rounded-xl bg-black/40 border border-gray-800 text-xs text-gray-300 font-mono whitespace-pre-wrap leading-relaxed max-h-36 overflow-y-auto">
                {updateInfo.changelog}
              </div>
            </div>
          )}

          {/* Download & Progress */}
          {downloading || progress ? (
            <div className="p-4 rounded-xl bg-cyan-950/20 border border-cyan-500/30 space-y-3">
              <div className="flex items-center justify-between text-xs">
                <span className="font-semibold text-cyan-300 flex items-center gap-2">
                  <RefreshCw className="w-3.5 h-3.5 animate-spin text-cyan-400" />
                  {getStageLabel(progress?.stage)}
                </span>
                <span className="font-mono text-cyan-400 font-bold">{percent}%</span>
              </div>

              {/* Progress Track */}
              <div className="h-2 w-full bg-gray-800 rounded-full overflow-hidden">
                <div
                  className="h-full bg-gradient-to-r from-cyan-500 to-blue-500 transition-all duration-300 ease-out"
                  style={{ width: `${percent}%` }}
                />
              </div>

              <div className="flex items-center justify-between text-[11px] font-mono text-gray-400">
                <span>
                  {downloadedMb} MB / {totalMb} MB
                </span>
                <span>Speed: {speedMb} MB/s</span>
                <span className="flex items-center gap-1">
                  <HardDrive className="w-3 h-3 text-cyan-400" /> BLAKE3 Stream
                </span>
              </div>
            </div>
          ) : null}
        </div>

        {/* Footer Actions */}
        <div className="flex items-center justify-between px-6 py-4 border-t border-gray-800 bg-[#161922]">
          <button
            onClick={handleCheck}
            disabled={checking || downloading}
            className="px-4 py-2 rounded-xl text-xs font-semibold bg-gray-800 hover:bg-gray-700 text-gray-200 flex items-center gap-2 transition-colors disabled:opacity-50"
          >
            <RefreshCw className={`w-3.5 h-3.5 ${checking ? 'animate-spin' : ''}`} />
            Re-Check Manifest
          </button>

          <div className="flex items-center gap-3">
            {readyToRestart ? (
              <button
                onClick={handleRelaunch}
                className="px-5 py-2.5 rounded-xl text-xs font-bold bg-gradient-to-r from-emerald-500 to-teal-500 hover:from-emerald-400 hover:to-teal-400 text-black flex items-center gap-2 shadow-lg shadow-emerald-500/20 transition-all"
              >
                <Zap className="w-4 h-4" />
                Update & Relaunch Process
              </button>
            ) : updateInfo?.update_available ? (
              <button
                onClick={handleStartUpdate}
                disabled={downloading}
                className="px-5 py-2.5 rounded-xl text-xs font-bold bg-gradient-to-r from-cyan-500 to-blue-500 hover:from-cyan-400 hover:to-blue-400 text-black flex items-center gap-2 shadow-lg shadow-cyan-500/20 transition-all disabled:opacity-50"
              >
                <Download className="w-4 h-4" />
                {downloading ? 'Downloading...' : 'Download & Apply Update'}
              </button>
            ) : (
              <button
                onClick={onClose}
                className="px-4 py-2 rounded-xl text-xs font-semibold bg-gray-800 hover:bg-gray-700 text-gray-300 transition-colors"
              >
                Close
              </button>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};
