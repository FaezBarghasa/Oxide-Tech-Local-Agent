import React, { useEffect, useState } from 'react';
export function StatusBar() {
  const [vram,setVram]=useState(12.4); const [tps,setTps]=useState(42.5); const [hist,setHist]=useState<number[]>(Array(24).fill(12));
  useEffect(()=>{ const t=setInterval(()=>{ setVram(v=>Math.min(24,Math.max(8,v+(Math.random()-0.5)))); setTps(v=>Math.max(5,v+(Math.random()-0.5)*4)); setHist(h=>[...h.slice(1),vram]); },500); return ()=>clearInterval(t); },[vram]);
  return <div role="status" aria-label="system telemetry" className="h-8 border-t border-white/10 bg-[#111113] flex items-center justify-between px-4 text-[11px] font-mono text-zinc-400 shrink-0">
    <div className="flex items-center gap-2"><span className="w-2 h-2 rounded-full bg-[#10B981] animate-pulse" aria-hidden/><span className="text-zinc-200">vLLM Sidecar: Running</span></div>
    <div className="hidden md:flex items-center gap-4"><span>VRAM: <b className="text-zinc-100">{vram.toFixed(1)}GB / 24GB</b></span>
      <svg width="72" height="16" aria-hidden>{hist.map((h,i)=><rect key={i} x={i*3} y={16-(h/24)*16} width="2" height={(h/24)*16} fill="#10B981" opacity="0.7"/>)}</svg>
      <span>Tokens/sec: <b className="text-zinc-100">{tps.toFixed(1)}</b></span></div>
    <div className="flex items-center gap-3"><span>LAN: <b className="text-[#10B981]">Visible</b></span><span>Tunnel: <b className="text-zinc-500">Inactive</b></span></div>
  </div>;
}
