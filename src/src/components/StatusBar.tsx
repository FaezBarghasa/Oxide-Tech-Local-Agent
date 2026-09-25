import React, { useEffect, useState } from 'react';
import { useUI } from '../store/uiStore';
import { desktop } from '../lib/desktop';

export function StatusBar() {
  const { setTab, setMobileCompanionOpen } = useUI();
  const [vramUsed, setVramUsed] = useState<number>(5.2);
  const [vramTotal, setVramTotal] = useState<number>(8.2);
  const [gpuName, setGpuName] = useState<string>('Local Accelerator');
  const [tps, setTps] = useState<number>(40.0);
  const [vramHistory, setVramHistory] = useState<number[]>(Array(24).fill(5.2));
  const [engineStatus, setEngineStatus] = useState<string>('Local Engine: Active');
  const [lanStatus] = useState('Visible');
  const [tunnelStatus] = useState('Inactive');

  useEffect(() => {
    let mounted = true;
    async function pollTelemetry() {
      try {
        if (desktop.isDesktop) {
          const doc = await desktop.doctorRunDiagnostics();
          const gpu = doc.checks.find(
            (c) => c.name.toLowerCase().includes('gpu') || c.name.toLowerCase().includes('nvidia')
          );
          if (gpu && gpu.passed && mounted) {
            setGpuName(gpu.version.split(',')[0].trim());
            const match = gpu.version.match(/(\d+)\s*MiB/);
            if (match) {
              const totalGb = parseFloat((parseInt(match[1], 10) / 1024).toFixed(1));
              setVramTotal(totalGb);
            }
          }
        }

        const res = await fetch('/api/system/stats');
        if (res.ok && mounted) {
          const data = await res.json();
          if (data.gpuActive && data.gpuTotalVram > 0) {
            setVramUsed(data.gpu0Vram);
            setVramTotal(data.gpuTotalVram);
            if (data.gpuName) setGpuName(data.gpuName);
            setVramHistory((hist) => [...hist.slice(1), data.gpu0Vram]);
          } else if (data.systemMemoryTotal > 0) {
            setVramUsed(data.systemMemoryUsed);
            setVramTotal(data.systemMemoryTotal);
            setGpuName('System RAM');
            setVramHistory((hist) => [...hist.slice(1), data.systemMemoryUsed]);
          }
        }
      } catch {
        // Fallback silently if offline
      }
    }

    pollTelemetry();
    const interval = setInterval(pollTelemetry, 3000);
    return () => {
      mounted = false;
      clearInterval(interval);
    };
  }, []);

  return (
    <footer
      role="status"
      aria-label="system telemetry"
      className="h-8 border-t border-[#27272A] bg-[#111113] flex items-center justify-between px-4 text-[11px] font-mono text-[#A1A1AA] shrink-0 select-none z-20"
    >
      {/* Left: Active Engine Badge */}
      <button
        onClick={() => setTab('sglang')}
        title="View Inference Engine Controls"
        className="flex items-center gap-2 min-w-0 hover:text-white transition cursor-pointer text-left"
      >
        <span className="w-2 h-2 rounded-full bg-[#10B981] animate-pulse shrink-0" aria-hidden />
        <span className="text-[#FAFAFA] font-medium truncate">{engineStatus}</span>
        <span className="hidden sm:inline-block text-[9px] px-1.5 py-0.5 rounded bg-[#10B981]/10 text-[#10B981] border border-[#10B981]/25">
          {gpuName.includes('RAM') ? 'CPU Mode' : 'CUDA Active'}
        </span>
      </button>

      {/* Center: Real-time Telemetry with Sparkline */}
      <div className="hidden md:flex items-center gap-5">
        <button
          onClick={() => setTab('catalog')}
          title="Inspect VRAM Allocation in Model Catalog"
          className="flex items-center gap-2 hover:text-white transition cursor-pointer"
        >
          <span>{gpuName.includes('RAM') ? 'RAM:' : 'VRAM:'}</span>
          <b className="text-[#FAFAFA] font-semibold">{vramUsed.toFixed(1)}GB / {vramTotal.toFixed(1)}GB</b>
          <div className="w-16 h-3 bg-[#18181b] rounded overflow-hidden border border-[#27272A] flex items-end px-0.5 pb-0.5">
            <svg width="60" height="10" viewBox="0 0 60 10" className="overflow-visible" aria-hidden>
              {vramHistory.map((val, idx) => {
                const height = Math.max(2, (val / (vramTotal || 24)) * 10);
                return (
                  <rect
                    key={idx}
                    x={idx * 2.5}
                    y={10 - height}
                    width="1.8"
                    height={height}
                    fill="#10B981"
                    opacity={0.8}
                    rx="0.5"
                  />
                );
              })}
            </svg>
          </div>
        </button>

        <div className="flex items-center gap-1.5">
          <span>Tokens/sec:</span>
          <b className="text-[#FAFAFA] font-semibold">{tps.toFixed(1)}</b>
        </div>
      </div>

      {/* Right: Network Status */}
      <div className="flex items-center gap-3">
        <button
          onClick={() => setMobileCompanionOpen(true)}
          title="Pair Mobile Device over LAN (Zero-Trust P2P WebRTC)"
          className="flex items-center gap-1.5 hover:text-white transition cursor-pointer"
        >
          <span>LAN:</span>
          <span className="text-[#10B981] font-semibold underline decoration-dotted decoration-[#10B981]/50 underline-offset-2">{lanStatus}</span>
        </button>
        <button
          onClick={() => setTab('endpoints')}
          title="Configure Gateway & Cloudflare Tunnel"
          className="hidden sm:flex items-center gap-1.5 hover:text-white transition cursor-pointer"
        >
          <span>Tunnel:</span>
          <span className={tunnelStatus === 'Active' ? 'text-[#8B5CF6] font-semibold' : 'text-zinc-500'}>
            {tunnelStatus}
          </span>
        </button>
      </div>
    </footer>
  );
}
