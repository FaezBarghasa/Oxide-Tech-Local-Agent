import React, { useState, useEffect } from 'react';
import {
  Cpu,
  Zap,
  Activity,
  ChevronUp,
  ChevronDown,
  Server,
  RefreshCw,
  HardDrive,
  ShieldCheck,
  AlertCircle,
} from 'lucide-react';
import { desktop } from '../lib/desktop';
import { DoctorResult } from '../types';

export const HardwareClusterStatus: React.FC = () => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [isLivePolling, setIsLivePolling] = useState(true);
  const [gatewayStatus, setGatewayStatus] = useState<'checking' | 'online' | 'offline'>('checking');
  const [doctorResult, setDoctorResult] = useState<DoctorResult | null>(null);
  const [lastCheck, setLastCheck] = useState<string>('');

  const fetchSystemData = async () => {
    try {
      if (desktop.isDesktop) {
        const [doc, gw] = await Promise.allSettled([
          desktop.doctorRunDiagnostics(),
          desktop.gatewayStatus(),
        ]);
        if (doc.status === 'fulfilled') {
          setDoctorResult(doc.value);
        }
        if (gw.status === 'fulfilled') {
          setGatewayStatus(gw.value === 200 ? 'online' : 'offline');
        } else {
          setGatewayStatus('offline');
        }
      } else {
        // Web mode health check
        try {
          const res = await fetch('/api/health');
          if (res.ok) {
            const data = await res.json();
            setGatewayStatus(data.rustGateway?.online ? 'online' : 'offline');
          } else {
            setGatewayStatus('offline');
          }
        } catch {
          setGatewayStatus('offline');
        }
      }
      setLastCheck(new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' }));
    } catch {
      setGatewayStatus('offline');
    }
  };

  useEffect(() => {
    fetchSystemData();
    if (!isLivePolling) return;
    const interval = setInterval(fetchSystemData, 10000);
    return () => clearInterval(interval);
  }, [isLivePolling]);

  // Extract real GPU info from Doctor scan if available
  const gpuCheck = doctorResult?.checks.find((c) => c.name.includes('GPU'));
  const hasNvidiaGpu = gpuCheck?.passed ?? false;
  const gpuName = gpuCheck?.version && gpuCheck.passed ? gpuCheck.version : 'CPU Lite / Direct Compute';

  return (
    <div className="border-t border-white/[0.07] bg-[#09090b]/95 backdrop-blur-xl relative z-20 font-sans transition-all">
      {/* Compact Main Footer Bar */}
      <div className="px-4 md:px-6 py-2.5 flex flex-col md:flex-row items-center justify-between gap-3">
        {/* Left Status & Brand */}
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2">
            <span
              className={`w-2 h-2 rounded-full ${
                gatewayStatus === 'online'
                  ? 'bg-emerald-400 shadow-[0_0_8px_rgba(16,185,129,0.8)] animate-pulse'
                  : 'bg-amber-400 shadow-[0_0_8px_rgba(245,158,11,0.5)]'
              }`}
            />
            <span className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-1.5">
              <Server className="w-3.5 h-3.5 text-amber-400" />
              <span>OXIDE AGENT OS</span>
            </span>
          </div>
          <span className="text-zinc-700 hidden sm:inline">|</span>
          <div className="text-[11px] font-mono text-zinc-300 flex items-center gap-2">
            <span className="text-zinc-400">Gateway:</span>
            <span
              className={`font-semibold ${
                gatewayStatus === 'online' ? 'text-emerald-400' : 'text-amber-400'
              }`}
            >
              {gatewayStatus === 'online' ? ':8080 ONLINE' : 'STANDALONE MODE'}
            </span>
          </div>
        </div>

        {/* Center Live Real-Time Info */}
        <div className="flex items-center gap-4 text-xs">
          {/* Real GPU / Compute Engine */}
          <div className="flex items-center gap-2 bg-[#121216] border border-white/[0.06] px-3 py-1 rounded-lg">
            <span className="text-[10px] font-mono text-zinc-400 font-semibold">Compute:</span>
            <span className={`text-[10px] font-mono font-medium ${hasNvidiaGpu ? 'text-emerald-300' : 'text-zinc-300'}`}>
              {gpuName}
            </span>
          </div>

          {/* Diagnostic status */}
          {doctorResult && (
            <div className="hidden sm:flex items-center gap-1.5 text-[10px] font-mono text-zinc-300 bg-[#121216] border border-white/[0.06] px-2.5 py-1 rounded-lg">
              <ShieldCheck className="w-3.5 h-3.5 text-emerald-400" />
              <span>
                {doctorResult.passed}/{doctorResult.checks.length} checks OK
              </span>
            </div>
          )}
        </div>

        {/* Right Detail Toggle Button */}
        <div className="flex items-center gap-2">
          <button
            onClick={() => fetchSystemData()}
            className="p-1 rounded-md text-zinc-400 hover:text-zinc-200 hover:bg-[#18181b] transition cursor-pointer"
            title="Refresh System Telemetry"
          >
            <RefreshCw className="w-3 h-3" />
          </button>

          <button
            onClick={() => setIsExpanded(!isExpanded)}
            className="px-2.5 py-1 rounded-lg bg-[#141418] hover:bg-[#1a1a20] text-zinc-200 border border-white/[0.08] text-[11px] font-medium transition flex items-center gap-1.5 cursor-pointer"
          >
            <span>{isExpanded ? 'Hide Telemetry' : 'System Hardware'}</span>
            {isExpanded ? (
              <ChevronDown className="w-3.5 h-3.5 text-amber-400" />
            ) : (
              <ChevronUp className="w-3.5 h-3.5 text-amber-400" />
            )}
          </button>
        </div>
      </div>

      {/* Expandable Hardware Dashboard Drawer */}
      {isExpanded && (
        <div className="p-4 md:p-6 bg-[#0e0e12] border-t border-white/[0.07] animate-in slide-in-from-bottom-4 duration-200">
          <div className="max-w-7xl mx-auto space-y-4">
            <div className="flex items-center justify-between pb-3 border-b border-white/[0.07]">
              <div className="flex items-center gap-2">
                <Cpu className="w-4 h-4 text-amber-400" />
                <span className="text-sm font-bold text-white">Verified Host Hardware & Toolchains</span>
              </div>
              <div className="text-[10px] font-mono text-zinc-400">
                Last Inspected: <span className="text-zinc-200">{lastCheck || 'Live'}</span>
              </div>
            </div>

            {/* Hardware Cards Grid */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              {/* Host Machine */}
              <div className="bg-[#121216] border border-white/[0.06] rounded-xl p-4 space-y-2.5">
                <div className="text-xs font-bold text-white flex items-center gap-2">
                  <span className="w-2 h-2 rounded-full bg-emerald-400" />
                  Host Platform
                </div>
                <div className="text-[11px] font-mono text-zinc-300">
                  Pop!_OS / Linux x86_64
                </div>
                <div className="text-[10px] font-mono text-zinc-400">
                  Local-first offline agent runtime
                </div>
              </div>

              {/* Compute Acceleration */}
              <div className="bg-[#121216] border border-white/[0.06] rounded-xl p-4 space-y-2.5">
                <div className="text-xs font-bold text-white flex items-center gap-2">
                  <span
                    className={`w-2 h-2 rounded-full ${hasNvidiaGpu ? 'bg-emerald-400' : 'bg-amber-400'}`}
                  />
                  GPU Accelerator
                </div>
                <div className="text-[11px] font-mono text-zinc-300 truncate">
                  {gpuName}
                </div>
                <div className="text-[10px] font-mono text-zinc-400">
                  {hasNvidiaGpu ? 'CUDA / Hardware Acceleration Enabled' : 'CPU SIMD / Host Memory'}
                </div>
              </div>

              {/* Memory / Storage */}
              <div className="bg-[#121216] border border-white/[0.06] rounded-xl p-4 space-y-2.5">
                <div className="text-xs font-bold text-white flex items-center gap-2">
                  <HardDrive className="w-3.5 h-3.5 text-amber-400" />
                  oxide-embed Storage
                </div>
                <div className="text-[11px] font-mono text-zinc-300">
                  Local AST & Memanto Fabric (.oxide/)
                </div>
                <div className="text-[10px] font-mono text-zinc-400">
                  STAIR Code-ToC Hierarchical Search
                </div>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

