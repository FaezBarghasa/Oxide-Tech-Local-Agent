import React, { useState, useEffect, useCallback } from 'react';
import {
  Cpu,
  Zap,
  Activity,
  Server,
  HardDrive,
  ShieldCheck,
  AlertCircle,
} from 'lucide-react';
import { desktop } from '../lib/desktop';
import { DoctorResult } from '../types';

export const HardwareClusterStatus: React.FC = () => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [gatewayStatus, setGatewayStatus] = useState<'checking' | 'online' | 'offline'>('checking');
  const [doctorResult, setDoctorResult] = useState<DoctorResult | null>(null);
  const [lastCheck, setLastCheck] = useState<string>('');
  const [hostInfo, setHostInfo] = useState<{ cpuThreads: number; ramGb: number; gpuName: string | null; gpuActive: boolean } | null>(null);

  const fetchSystemData = useCallback(async () => {
    try {
      if (desktop.isDesktop) {
        const [doc, gw] = await Promise.allSettled([
          desktop.doctorRunDiagnostics(),
          desktop.gatewayStatus(),
        ]);
        if (doc.status === 'fulfilled') setDoctorResult(doc.value);
        if (gw.status === 'fulfilled') {
          setGatewayStatus(gw.value === 200 ? 'online' : 'offline');
        }
      }
      // Host inspection
      if (desktop.isDesktop) {
        try {
          const cpuThreads = navigator.hardwareConcurrency || 0;
          const ramGb = Math.round(((performance as any).memory?.usedJSHeapSize / (1024 * 1024 * 1024)) * 10) / 10;
          setHostInfo({ cpuThreads, ramGb, gpuName: null, gpuActive: false });
        } catch {}
      }
      // Web mode health check
      if (!desktop.isDesktop) {
        try {
          const res = await fetch('/api/health');
          if (res.ok) {
            const data = await res.json();
            setGatewayStatus(data.rustGateway?.online ? 'online' : 'offline');
          }
        } catch {}
        setGatewayStatus('checking');
      }
      setLastCheck(new Date().toLocaleTimeString([], { hour: '2-digit', minute: '2-digit', second: '2-digit' }));
    } catch {
      setGatewayStatus('offline');
    }
  }, []);

  useEffect(() => {
    fetchSystemData();
    const interval = setInterval(fetchSystemData, 15000);
    return () => clearInterval(interval);
  }, [fetchSystemData]);

  const hasNvidiaGpu = doctorResult?.nvidiaGpu?.passed ?? false;
  const gpuName = hasNvidiaGpu ? (doctorResult?.nvidiaGpu?.version || 'NVIDIA GPU') : null;

  return (
    <div className="border-t border-white/[0.07] bg-[#09090b]/95 backdrop-blur-xl relative z-20 font-sans transition-all">
      <div
        className="px-4 md:px-6 py-2.5 flex flex-col md:flex-row items-center justify-between gap-3 cursor-pointer"
        onClick={() => setIsExpanded(!isExpanded)}
      >
        <div className="flex items-center gap-2">
          <Activity className="w-3.5 h-3.5 text-amber-400" />
          <span className="text-[11px] font-mono text-zinc-300 flex items-center gap-2">
            Hardware Cluster
            {lastCheck && <span className="text-zinc-600">{lastCheck}</span>}
          </span>
        </div>
        <div className="flex items-center gap-4 text-xs">
          <div className="flex items-center gap-2 bg-[#121216] border border-white/[0.06] px-3 py-1 rounded-lg">
            <span className={`w-2 h-2 rounded-full ${gatewayStatus === 'online' ? 'bg-emerald-400' : gatewayStatus === 'offline' ? 'bg-red-400' : 'bg-amber-400 animate-pulse'}`} />
            <span className="text-[10px] font-mono text-zinc-300">
              {gatewayStatus === 'online' ? 'GATEWAY LIVE' : gatewayStatus === 'offline' ? 'GATEWAY OFFLINE' : 'CHECKING'}
            </span>
          </div>
          <span className={`w-2 h-2 rounded-full ${isExpanded ? 'rotate-180' : ''} transition-transform text-zinc-500`} />
        </div>
      </div>

      {isExpanded && (
        <div className="px-4 md:px-6 py-4 bg-[#0e0e12] border-t border-white/[0.07] animate-in slide-in-from-bottom-4 duration-200">
          <div className="max-w-7xl mx-auto space-y-4">
            <div className="flex items-center justify-between pb-3 border-b border-white/[0.07]">
              <div className="flex items-center gap-2">
                <Cpu className="w-3.5 h-3.5 text-zinc-400" />
                <span className="text-[10px] font-mono text-zinc-400 uppercase tracking-wider">Host Inspection</span>
              </div>
              <span className="text-[10px] font-mono text-zinc-500">Real-time host data</span>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
              {/* CPU */}
              <div className="bg-[#121216] border border-white/[0.06] rounded-xl p-4 space-y-2.5">
                <div className="text-xs font-bold text-white flex items-center gap-2">
                  <Cpu className="w-3.5 h-3.5 text-amber-400" />
                  CPU Threads
                </div>
                <div className="text-[11px] font-mono text-zinc-300">
                  {hostInfo?.cpuThreads || navigator.hardwareConcurrency || '—'}
                </div>
                <div className="text-[10px] font-mono text-zinc-400">
                  Logical processors
                </div>
              </div>

              {/* Memory */}
              <div className="bg-[#121216] border border-white/[0.06] rounded-xl p-4 space-y-2.5">
                <div className="text-xs font-bold text-white flex items-center gap-2">
                  <HardDrive className="w-3.5 h-3.5 text-emerald-400" />
                  Memory
                </div>
                <div className="text-[11px] font-mono text-zinc-300">
                  {hostInfo?.ramGb ? `${hostInfo.ramGb} GB` : '—'}
                </div>
                <div className="text-[10px] font-mono text-zinc-400">
                  Heap allocated
                </div>
              </div>

              {/* GPU */}
              <div className="bg-[#121216] border border-white/[0.06] rounded-xl p-4 space-y-2.5">
                <div className="text-xs font-bold text-white flex items-center gap-2">
                  <Zap className="w-3.5 h-3.5 text-cyan-400" />
                  GPU Acceleration
                </div>
                <div className="text-[11px] font-mono text-zinc-300 truncate">
                  {gpuName || (hasNvidiaGpu ? 'Detected' : 'Not detected')}
                </div>
                <div className="text-[10px] font-mono text-zinc-400">
                  {hasNvidiaGpu ? 'Hardware accelerated' : 'Software fallback'}
                </div>
              </div>
            </div>

            {/* Doctor results summary */}
            {doctorResult && (
              <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-2">
                <div className="bg-[#121216] border border-white/[0.06] rounded-xl p-4 space-y-2.5">
                  <div className="text-xs font-bold text-white flex items-center gap-2">
                    <ShieldCheck className="w-3.5 h-3.5 text-emerald-400" />
                    Checks Passed
                  </div>
                  <div className="text-[11px] font-mono text-zinc-300">{doctorResult.passed}</div>
                  <div className="text-[10px] font-mono text-zinc-400">Total verified</div>
                </div>
                <div className="bg-[#121216] border border-white/[0.06] rounded-xl p-4 space-y-2.5">
                  <div className="text-xs font-bold text-white flex items-center gap-2">
                    <AlertCircle className="w-3.5 h-3.5 text-amber-400" />
                    Warnings
                  </div>
                  <div className="text-[11px] font-mono text-zinc-300">{doctorResult.warnings}</div>
                  <div className="text-[10px] font-mono text-zinc-400">Optional issues</div>
                </div>
                <div className="bg-[#121216] border border-white/[0.06] rounded-xl p-4 space-y-2.5">
                  <div className="text-xs font-bold text-white flex items-center gap-2">
                    <AlertCircle className="w-3.5 h-3.5 text-rose-400" />
                    Failures
                  </div>
                  <div className="text-[11px] font-mono text-zinc-300">{doctorResult.failed}</div>
                  <div className="text-[10px] font-mono text-zinc-400">Critical errors</div>
                </div>
              </div>
            )}

            {/* Cluster status */}
            <div className="bg-[#121216] border border-white/[0.06] rounded-xl p-4 space-y-2.5 mt-2">
              <div className="text-xs font-bold text-white flex items-center gap-2">
                <Server className="w-3.5 h-3.5 text-zinc-400" />
                Cluster Status
              </div>
              <div className="flex items-center gap-2">
                <span className="w-2 h-2 rounded-full bg-emerald-400" />
                <span className="text-[11px] font-mono text-zinc-300">Local-first offline agent runtime</span>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
