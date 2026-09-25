import React, { useEffect, useState } from 'react';
import { useUI } from '../store/uiStore';

export function StatusBar() {
  const { setTab, setMobileCompanionOpen } = useUI();
  const [vram, setVram] = useState(12.4);
  const [tps, setTps] = useState(42.5);
  const [vramHistory, setVramHistory] = useState<number[]>(Array(24).fill(12.4));
  const [engineStatus] = useState<'Candle (Native): Active' | 'vLLM Sidecar: Running' | 'SGLang: Idle'>('vLLM Sidecar: Running');
  const [lanStatus] = useState('Visible');
  const [tunnelStatus] = useState('Inactive');

  useEffect(() => {
    // Hardware telemetry polling / simulation
    const interval = setInterval(() => {
      setVram((prev) => {
        const next = Math.min(23.8, Math.max(8.0, prev + (Math.random() - 0.48) * 0.4));
        setVramHistory((hist) => [...hist.slice(1), next]);
        return next;
      });
      setTps((prev) => Math.max(12, prev + (Math.random() - 0.5) * 3));
    }, 600);

    return () => clearInterval(interval);
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
          Local CUDA
        </span>
      </button>

      {/* Center: Real-time Telemetry with Sparkline */}
      <div className="hidden md:flex items-center gap-5">
        <button
          onClick={() => setTab('catalog')}
          title="Inspect VRAM Allocation in Model Catalog"
          className="flex items-center gap-2 hover:text-white transition cursor-pointer"
        >
          <span>VRAM:</span>
          <b className="text-[#FAFAFA] font-semibold">{vram.toFixed(1)}GB / 24GB</b>
          <div className="w-16 h-3 bg-[#18181b] rounded overflow-hidden border border-[#27272A] flex items-end px-0.5 pb-0.5">
            <svg width="60" height="10" viewBox="0 0 60 10" className="overflow-visible" aria-hidden>
              {vramHistory.map((val, idx) => {
                const height = Math.max(2, (val / 24) * 10);
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
