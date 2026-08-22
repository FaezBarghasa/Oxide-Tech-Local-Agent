import React, { useState, useEffect } from 'react';
import {
  Cpu,
  Zap,
  Activity,
  ChevronUp,
  ChevronDown,
  Server,
  Layers,
  Thermometer,
  Fan,
  RefreshCw,
  Sliders,
  CheckCircle2,
} from 'lucide-react';

export const HardwareClusterStatus: React.FC = () => {
  const [isExpanded, setIsExpanded] = useState(false);
  const [isLivePolling, setIsLivePolling] = useState(true);

  // Live fluctuating telemetry state for dual RTX 3090 TP=2
  const [gpu0, setGpu0] = useState({
    name: 'NVIDIA GeForce RTX 3090 #0 (Rank 0)',
    vramUsed: 18.4,
    vramTotal: 24.0,
    utilization: 87,
    temp: 64,
    power: 314,
    powerLimit: 350,
    fanSpeed: 68,
    pcieThroughput: 13.4,
    activeLayer: 'Qwen3.8-35B TP-0 (11.2GB) + Radix KV (5.4GB) + LoRA (1.8GB)',
  });

  const [gpu1, setGpu1] = useState({
    name: 'NVIDIA GeForce RTX 3090 #1 (Rank 1)',
    vramUsed: 17.9,
    vramTotal: 24.0,
    utilization: 85,
    temp: 62,
    power: 298,
    powerLimit: 350,
    fanSpeed: 65,
    pcieThroughput: 13.4,
    activeLayer: 'Qwen3.8-35B TP-1 (11.2GB) + Radix KV (5.1GB) + LoRA (1.6GB)',
  });

  const [hostRam, setHostRam] = useState({ used: 42.8, total: 128.0 });
  const [cpuUsage, setCpuUsage] = useState(24);
  const [allReduceLatency, setAllReduceLatency] = useState(0.84);

  // Periodic subtle live fluctuation to demonstrate real-time telemetry
  useEffect(() => {
    if (!isLivePolling) return;
    const interval = setInterval(() => {
      setGpu0((prev) => ({
        ...prev,
        utilization: Math.min(99, Math.max(65, prev.utilization + Math.floor((Math.random() - 0.5) * 6))),
        vramUsed: Number((18.2 + Math.random() * 0.5).toFixed(1)),
        power: Math.min(345, Math.max(285, prev.power + Math.floor((Math.random() - 0.5) * 8))),
        temp: Math.min(72, Math.max(58, prev.temp + (Math.random() > 0.6 ? 1 : Math.random() < 0.4 ? -1 : 0))),
      }));

      setGpu1((prev) => ({
        ...prev,
        utilization: Math.min(98, Math.max(62, prev.utilization + Math.floor((Math.random() - 0.5) * 6))),
        vramUsed: Number((17.7 + Math.random() * 0.5).toFixed(1)),
        power: Math.min(340, Math.max(280, prev.power + Math.floor((Math.random() - 0.5) * 8))),
        temp: Math.min(70, Math.max(58, prev.temp + (Math.random() > 0.6 ? 1 : Math.random() < 0.4 ? -1 : 0))),
      }));

      setAllReduceLatency(Number((0.82 + Math.random() * 0.06).toFixed(2)));
      setCpuUsage(Math.min(45, Math.max(18, Math.round(24 + (Math.random() - 0.5) * 6))));
    }, 1800);

    return () => clearInterval(interval);
  }, [isLivePolling]);

  const totalVramUsed = Number((gpu0.vramUsed + gpu1.vramUsed).toFixed(1));
  const totalVramMax = gpu0.vramTotal + gpu1.vramTotal;
  const vramPercent = Math.round((totalVramUsed / totalVramMax) * 100);
  const avgGpuUtil = Math.round((gpu0.utilization + gpu1.utilization) / 2);

  return (
    <div className="border-t border-[#232530] bg-[#0c0d12]/95 backdrop-blur-xl relative z-20 font-sans transition-all">
      {/* Compact Main Footer Bar */}
      <div className="px-4 md:px-6 py-2 flex flex-col md:flex-row items-center justify-between gap-3">
        {/* Left Status & Brand */}
        <div className="flex items-center gap-3">
          <div className="flex items-center gap-2">
            <span className="w-2 h-2 rounded-full bg-emerald-400 shadow-[0_0_8px_rgba(34,197,94,0.8)] animate-pulse" />
            <span className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-1.5">
              <Server className="w-3.5 h-3.5 text-orange-400" />
              <span>TP=2 CLUSTER</span>
            </span>
          </div>
          <span className="text-gray-600 hidden sm:inline">|</span>
          <div className="text-[11px] mono text-gray-300 flex items-center gap-2">
            <span>Dual RTX 3090</span>
            <span className="text-orange-400 font-semibold">{totalVramUsed}/{totalVramMax} GB ({vramPercent}%)</span>
          </div>
        </div>

        {/* Center Live Real-Time Gauges */}
        <div className="flex items-center gap-4 text-xs">
          {/* GPU 0 VRAM Pill */}
          <div className="flex items-center gap-2 bg-[#14151e] border border-[#232530] px-3 py-1 rounded-lg">
            <span className="text-[10px] mono text-gray-400 font-semibold">GPU 0:</span>
            <div className="w-16 bg-[#0b0c10] h-2 rounded-full overflow-hidden border border-[#262838]">
              <div
                className="h-full bg-gradient-to-r from-orange-500 to-amber-400 rounded-full transition-all duration-500"
                style={{ width: `${(gpu0.vramUsed / gpu0.vramTotal) * 100}%` }}
              />
            </div>
            <span className="text-[10px] mono text-orange-300 font-bold">{gpu0.vramUsed}G</span>
            <span className="text-[9px] mono text-gray-400">({gpu0.utilization}% load)</span>
          </div>

          {/* GPU 1 VRAM Pill */}
          <div className="flex items-center gap-2 bg-[#14151e] border border-[#232530] px-3 py-1 rounded-lg">
            <span className="text-[10px] mono text-gray-400 font-semibold">GPU 1:</span>
            <div className="w-16 bg-[#0b0c10] h-2 rounded-full overflow-hidden border border-[#262838]">
              <div
                className="h-full bg-gradient-to-r from-orange-500 to-amber-400 rounded-full transition-all duration-500"
                style={{ width: `${(gpu1.vramUsed / gpu1.vramTotal) * 100}%` }}
              />
            </div>
            <span className="text-[10px] mono text-orange-300 font-bold">{gpu1.vramUsed}G</span>
            <span className="text-[9px] mono text-gray-400">({gpu1.utilization}% load)</span>
          </div>

          {/* Interconnect Latency */}
          <div className="hidden lg:flex items-center gap-1.5 text-[10px] mono text-emerald-400 bg-emerald-500/10 border border-emerald-500/20 px-2.5 py-1 rounded-lg">
            <Zap className="w-3 h-3 fill-current text-emerald-400" />
            <span>NCCL: {allReduceLatency}ms</span>
          </div>
        </div>

        {/* Right Detail Toggle Button */}
        <div className="flex items-center gap-2">
          <button
            onClick={() => setIsLivePolling(!isLivePolling)}
            className={`px-2 py-1 rounded-md text-[10px] mono transition flex items-center gap-1 cursor-pointer ${
              isLivePolling
                ? 'bg-emerald-500/15 text-emerald-300 border border-emerald-500/30'
                : 'bg-[#181a24] text-gray-400 border border-[#262838]'
            }`}
            title="Toggle Live Telemetry Polling"
          >
            <Activity className={`w-3 h-3 ${isLivePolling ? 'animate-spin' : ''}`} />
            <span>{isLivePolling ? 'LIVE' : 'PAUSED'}</span>
          </button>

          <button
            onClick={() => setIsExpanded(!isExpanded)}
            className="px-2.5 py-1 rounded-lg bg-[#161824] hover:bg-[#202230] text-gray-200 border border-[#262838] text-[11px] font-medium transition flex items-center gap-1 cursor-pointer"
          >
            <span>{isExpanded ? 'Hide Cluster Gauges' : 'Cluster Telemetry'}</span>
            {isExpanded ? <ChevronDown className="w-3.5 h-3.5 text-orange-400" /> : <ChevronUp className="w-3.5 h-3.5 text-orange-400" />}
          </button>
        </div>
      </div>

      {/* Expandable Deep Hardware Dashboard Drawer */}
      {isExpanded && (
        <div className="p-4 md:p-6 bg-[#0e1017] border-t border-[#232530] animate-in slide-in-from-bottom-4 duration-200">
          <div className="max-w-7xl mx-auto space-y-4">
            <div className="flex items-center justify-between pb-3 border-b border-[#232530]">
              <div className="flex items-center gap-2">
                <Cpu className="w-4 h-4 text-orange-400" />
                <span className="text-sm font-bold text-white">SGLang Tensor Parallelism (TP=2) Dual-GPU Telemetry</span>
                <span className="text-[10px] mono px-2 py-0.5 rounded bg-orange-500/10 text-orange-400 border border-orange-500/30 font-semibold">
                  PCIe 4.0 P2P Active
                </span>
              </div>
              <div className="text-[10px] mono text-gray-400">
                RadixAttention Cache Hit Rate: <span className="text-emerald-400 font-bold">89.4%</span>
              </div>
            </div>

            {/* GPU Cards Grid */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {/* GPU 0 Shard Card */}
              <div className="bg-[#14151e] border border-[#262838] rounded-xl p-4 space-y-3">
                <div className="flex items-center justify-between">
                  <div>
                    <div className="text-xs font-bold text-white flex items-center gap-2">
                      <span className="w-2 h-2 rounded-full bg-emerald-400" />
                      {gpu0.name}
                    </div>
                    <div className="text-[10px] mono text-gray-400 mt-0.5">Primary Node · CUDA Device 0</div>
                  </div>
                  <span className="text-[10px] mono font-bold text-orange-400 bg-orange-500/10 px-2 py-0.5 rounded border border-orange-500/20">
                    TP RANK 0
                  </span>
                </div>

                {/* VRAM Progress */}
                <div>
                  <div className="flex items-center justify-between text-[11px] mono mb-1">
                    <span className="text-gray-400">VRAM Allocation</span>
                    <span className="text-white font-bold">
                      {gpu0.vramUsed} GB / {gpu0.vramTotal} GB ({Math.round((gpu0.vramUsed / gpu0.vramTotal) * 100)}%)
                    </span>
                  </div>
                  <div className="w-full bg-[#0b0c10] h-3 rounded-full overflow-hidden border border-[#232530]">
                    <div
                      className="h-full bg-gradient-to-r from-orange-500 to-amber-400 rounded-full transition-all duration-500"
                      style={{ width: `${(gpu0.vramUsed / gpu0.vramTotal) * 100}%` }}
                    />
                  </div>
                </div>

                {/* Memory Breakdown */}
                <div className="p-2.5 rounded-lg bg-[#0b0c10] border border-[#232530] text-[10px] mono text-gray-300">
                  <div className="text-gray-400 text-[9px] uppercase font-bold tracking-wider mb-1">Allocated Memory Pools</div>
                  <div className="text-orange-200/90 leading-relaxed font-mono">
                    {gpu0.activeLayer}
                  </div>
                </div>

                {/* Hardware Sensors Grid */}
                <div className="grid grid-cols-4 gap-2 pt-1 text-center text-[10px] mono">
                  <div className="bg-[#0b0c10] p-2 rounded border border-[#232530]">
                    <div className="text-gray-400 flex items-center justify-center gap-1">
                      <Activity className="w-3 h-3 text-orange-400" />
                      SM Load
                    </div>
                    <div className="text-xs font-bold text-white mt-1">{gpu0.utilization}%</div>
                  </div>
                  <div className="bg-[#0b0c10] p-2 rounded border border-[#232530]">
                    <div className="text-gray-400 flex items-center justify-center gap-1">
                      <Thermometer className="w-3 h-3 text-amber-400" />
                      Temp
                    </div>
                    <div className="text-xs font-bold text-amber-400 mt-1">{gpu0.temp}°C</div>
                  </div>
                  <div className="bg-[#0b0c10] p-2 rounded border border-[#232530]">
                    <div className="text-gray-400 flex items-center justify-center gap-1">
                      <Zap className="w-3 h-3 text-orange-400" />
                      Power
                    </div>
                    <div className="text-xs font-bold text-white mt-1">{gpu0.power}W</div>
                  </div>
                  <div className="bg-[#0b0c10] p-2 rounded border border-[#232530]">
                    <div className="text-gray-400 flex items-center justify-center gap-1">
                      <Fan className="w-3 h-3 text-emerald-400" />
                      Fan
                    </div>
                    <div className="text-xs font-bold text-emerald-400 mt-1">{gpu0.fanSpeed}%</div>
                  </div>
                </div>
              </div>

              {/* GPU 1 Shard Card */}
              <div className="bg-[#14151e] border border-[#262838] rounded-xl p-4 space-y-3">
                <div className="flex items-center justify-between">
                  <div>
                    <div className="text-xs font-bold text-white flex items-center gap-2">
                      <span className="w-2 h-2 rounded-full bg-emerald-400" />
                      {gpu1.name}
                    </div>
                    <div className="text-[10px] mono text-gray-400 mt-0.5">Secondary Node · CUDA Device 1</div>
                  </div>
                  <span className="text-[10px] mono font-bold text-orange-400 bg-orange-500/10 px-2 py-0.5 rounded border border-orange-500/20">
                    TP RANK 1
                  </span>
                </div>

                {/* VRAM Progress */}
                <div>
                  <div className="flex items-center justify-between text-[11px] mono mb-1">
                    <span className="text-gray-400">VRAM Allocation</span>
                    <span className="text-white font-bold">
                      {gpu1.vramUsed} GB / {gpu1.vramTotal} GB ({Math.round((gpu1.vramUsed / gpu1.vramTotal) * 100)}%)
                    </span>
                  </div>
                  <div className="w-full bg-[#0b0c10] h-3 rounded-full overflow-hidden border border-[#232530]">
                    <div
                      className="h-full bg-gradient-to-r from-orange-500 to-amber-400 rounded-full transition-all duration-500"
                      style={{ width: `${(gpu1.vramUsed / gpu1.vramTotal) * 100}%` }}
                    />
                  </div>
                </div>

                {/* Memory Breakdown */}
                <div className="p-2.5 rounded-lg bg-[#0b0c10] border border-[#232530] text-[10px] mono text-gray-300">
                  <div className="text-gray-400 text-[9px] uppercase font-bold tracking-wider mb-1">Allocated Memory Pools</div>
                  <div className="text-orange-200/90 leading-relaxed font-mono">
                    {gpu1.activeLayer}
                  </div>
                </div>

                {/* Hardware Sensors Grid */}
                <div className="grid grid-cols-4 gap-2 pt-1 text-center text-[10px] mono">
                  <div className="bg-[#0b0c10] p-2 rounded border border-[#232530]">
                    <div className="text-gray-400 flex items-center justify-center gap-1">
                      <Activity className="w-3 h-3 text-orange-400" />
                      SM Load
                    </div>
                    <div className="text-xs font-bold text-white mt-1">{gpu1.utilization}%</div>
                  </div>
                  <div className="bg-[#0b0c10] p-2 rounded border border-[#232530]">
                    <div className="text-gray-400 flex items-center justify-center gap-1">
                      <Thermometer className="w-3 h-3 text-amber-400" />
                      Temp
                    </div>
                    <div className="text-xs font-bold text-amber-400 mt-1">{gpu1.temp}°C</div>
                  </div>
                  <div className="bg-[#0b0c10] p-2 rounded border border-[#232530]">
                    <div className="text-gray-400 flex items-center justify-center gap-1">
                      <Zap className="w-3 h-3 text-orange-400" />
                      Power
                    </div>
                    <div className="text-xs font-bold text-white mt-1">{gpu1.power}W</div>
                  </div>
                  <div className="bg-[#0b0c10] p-2 rounded border border-[#232530]">
                    <div className="text-gray-400 flex items-center justify-center gap-1">
                      <Fan className="w-3 h-3 text-emerald-400" />
                      Fan
                    </div>
                    <div className="text-xs font-bold text-emerald-400 mt-1">{gpu1.fanSpeed}%</div>
                  </div>
                </div>
              </div>
            </div>

            {/* Host Node & Interconnect Footer Bar */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3 pt-2 text-[11px] mono">
              <div className="p-3 bg-[#14151e] border border-[#262838] rounded-xl flex items-center justify-between">
                <div>
                  <div className="text-gray-400 text-[10px]">Host System RAM</div>
                  <div className="text-white font-bold mt-0.5">{hostRam.used} / {hostRam.total} GB (DDR5)</div>
                </div>
                <span className="text-[10px] text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20 font-bold">
                  OK
                </span>
              </div>

              <div className="p-3 bg-[#14151e] border border-[#262838] rounded-xl flex items-center justify-between">
                <div>
                  <div className="text-gray-400 text-[10px]">Host CPU Threads</div>
                  <div className="text-white font-bold mt-0.5">32 Cores @ {cpuUsage}% Utilization</div>
                </div>
                <span className="text-[10px] text-emerald-400 bg-emerald-500/10 px-2 py-0.5 rounded border border-emerald-500/20 font-bold">
                  ASYNC
                </span>
              </div>

              <div className="p-3 bg-[#14151e] border border-[#262838] rounded-xl flex items-center justify-between">
                <div>
                  <div className="text-gray-400 text-[10px]">NCCL All-Reduce Bus</div>
                  <div className="text-white font-bold mt-0.5">26.8 GB/s · {allReduceLatency} ms</div>
                </div>
                <span className="text-[10px] text-orange-400 bg-orange-500/10 px-2 py-0.5 rounded border border-orange-500/20 font-bold">
                  TP=2 SYNC
                </span>
              </div>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
