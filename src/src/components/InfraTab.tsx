import React, { useState } from 'react';
import { Server, Terminal, CheckCircle2, Copy, Check, Play, RefreshCw } from 'lucide-react';

export const InfraTab: React.FC = () => {
  const [copied, setCopied] = useState<string | null>(null);
  const [isLaunching, setIsLaunching] = useState(false);

  const handleCopy = (text: string, key: string) => {
    navigator.clipboard.writeText(text);
    setCopied(key);
    setTimeout(() => setCopied(null), 2000);
  };

  const aptCommands = `sudo apt update && sudo apt install -y \\
  build-essential cmake pkg-config libssl-dev clang lld mold \\
  protobuf-compiler flatbuffers-compiler freecad-python3 blender \\
  libkicad-3d-api-dev libusb-1.0-0-dev qemu-system-x86 qemu-system-arm \\
  bubblewrap podman docker.io libvirt-daemon-system sccache`;

  const cargoConfig = `[build]
rustc-wrapper = "sccache"

[target.x86_64-unknown-linux-gnu]
linker = "clang"
rustflags = ["-C", "link-arg=-fuse-ld=lld"]

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip STM32F401RETx"`;

  const sglangScript = `#!/usr/bin/env bash
export CUDA_VISIBLE_DEVICES=0,1
export NCCL_P2P_DISABLE=0
export NCCL_IB_DISABLE=1

python3 -m sglang.launch_server \\
  --model-path Qwen/Qwen3.8-35B-Instruct-AWQ \\
  --tp-size 2 \\
  --enable-lora \\
  --max-lora-rank 32 \\
  --host 0.0.0.0 \\
  --port 8080 \\
  --mem-fraction-static 0.85 \\
  --context-length 32768 \\
  --schedule-heuristic lpm`;

  return (
    <div className="space-y-6 font-sans">
      {/* Overview Card */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(249,115,22,0.1)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-4 border-b border-[#232530]">
          <div>
            <div className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Server className="w-4 h-4 text-orange-400" />
              <span>Phase 0 · System Native Dependencies & Infrastructure</span>
            </div>
            <div className="text-[10px] mono text-gray-400 mt-0.5">
              Pop!_OS 24.04 LTS · build-essential · sccache · lld · mold · SurrealDB · Qdrant
            </div>
          </div>
          <button
            onClick={() => {
              setIsLaunching(true);
              setTimeout(() => setIsLaunching(false), 1500);
            }}
            disabled={isLaunching}
            className="px-4 py-2 rounded-lg bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-gray-950 text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-1.5 cursor-pointer disabled:opacity-50 shadow-[0_0_15px_rgba(249,115,22,0.35)]"
          >
            <RefreshCw className={`w-3.5 h-3.5 ${isLaunching ? 'animate-spin' : ''}`} />
            <span>{isLaunching ? 'Validating...' : 'Verify All Daemons'}</span>
          </button>
        </div>

        {/* Daemons Status Cards */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-3.5 mt-4">
          <div className="bg-[#181a24] border border-[#262838] rounded-xl p-4 flex items-start justify-between">
            <div>
              <div className="text-xs font-bold text-white flex items-center gap-2">
                <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
                SurrealDB Graph DB
              </div>
              <div className="text-[10px] mono text-gray-400 mt-0.5">port :8000 · memory engine</div>
              <div className="text-[11px] text-gray-300 mt-2 leading-relaxed">
                Manages AST relations, netlists, execution DAGs & episode logs.
              </div>
            </div>
            <span className="text-[9px] mono px-2.5 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 font-bold">
              HEALTHY
            </span>
          </div>

          <div className="bg-[#181a24] border border-[#262838] rounded-xl p-4 flex items-start justify-between">
            <div>
              <div className="text-xs font-bold text-white flex items-center gap-2">
                <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
                Qdrant Vector Engine
              </div>
              <div className="text-[10px] mono text-gray-400 mt-0.5">port :6333 · collection oxide_core</div>
              <div className="text-[11px] text-gray-300 mt-2 leading-relaxed">
                4,820 indexed documentation vectors with Mojo SIMD cosine accelerator.
              </div>
            </div>
            <span className="text-[9px] mono px-2.5 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 font-bold">
              HEALTHY
            </span>
          </div>

          <div className="bg-[#181a24] border border-[#262838] rounded-xl p-4 flex items-start justify-between">
            <div>
              <div className="text-xs font-bold text-white flex items-center gap-2">
                <span className="w-2 h-2 rounded-full bg-emerald-400 animate-pulse" />
                SGLang TP=2 Server
              </div>
              <div className="text-[10px] mono text-gray-400 mt-0.5">port :8080 · dual RTX 3090</div>
              <div className="text-[11px] text-gray-300 mt-2 leading-relaxed">
                Serving Qwen3.8-35B-AWQ with RadixAttention prefix caching and dynamic LoRA.
              </div>
            </div>
            <span className="text-[9px] mono px-2.5 py-0.5 rounded bg-emerald-500/10 text-emerald-400 border border-emerald-500/30 font-bold">
              ONLINE
            </span>
          </div>
        </div>
      </div>

      {/* Commands & Scripts Grid */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Native Toolchain Command */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 flex flex-col justify-between shadow-sm">
          <div>
            <div className="flex items-center justify-between mb-2">
              <span className="text-xs font-bold text-white flex items-center gap-1.5 uppercase tracking-wider">
                <Terminal className="w-3.5 h-3.5 text-orange-400" />
                Task 0.1 · Native Toolchain Install
              </span>
              <button
                onClick={() => handleCopy(aptCommands, 'apt')}
                className="text-[10px] mono text-gray-300 hover:text-white flex items-center gap-1 px-2.5 py-1 rounded bg-[#181a24] border border-[#262838] transition cursor-pointer"
              >
                {copied === 'apt' ? <Check className="w-3 h-3 text-emerald-400" /> : <Copy className="w-3 h-3" />}
                Copy
              </button>
            </div>
            <pre className="bg-[#0b0c10] border border-[#232530] rounded-lg p-3 text-[11px] mono text-gray-200 overflow-x-auto leading-relaxed">
              {aptCommands}
            </pre>
          </div>
          <div className="text-[10px] mono text-gray-400 mt-3">
            Includes clang, lld, mold, KiCad 3D API, protobuf, FreeCAD python bindings, and QEMU.
          </div>
        </div>

        {/* Cargo Config */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 flex flex-col justify-between shadow-sm">
          <div>
            <div className="flex items-center justify-between mb-2">
              <span className="text-xs font-bold text-white flex items-center gap-1.5 uppercase tracking-wider">
                <Terminal className="w-3.5 h-3.5 text-orange-400" />
                ~/.cargo/config.toml (sccache + lld)
              </span>
              <button
                onClick={() => handleCopy(cargoConfig, 'cargo')}
                className="text-[10px] mono text-gray-300 hover:text-white flex items-center gap-1 px-2.5 py-1 rounded bg-[#181a24] border border-[#262838] transition cursor-pointer"
              >
                {copied === 'cargo' ? <Check className="w-3 h-3 text-emerald-400" /> : <Copy className="w-3 h-3" />}
                Copy
              </button>
            </div>
            <pre className="bg-[#0b0c10] border border-[#232530] rounded-lg p-3 text-[11px] mono text-gray-200 overflow-x-auto leading-relaxed">
              {cargoConfig}
            </pre>
          </div>
          <div className="text-[10px] mono text-gray-400 mt-3">
            Speeds up compilation up to 4.5× using LLVM LLD linker and shared SCCache storage.
          </div>
        </div>
      </div>

      {/* SGLang Launch Script */}
      <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 shadow-sm">
        <div className="flex items-center justify-between mb-2">
          <span className="text-xs font-bold text-white flex items-center gap-1.5 uppercase tracking-wider">
            <Terminal className="w-3.5 h-3.5 text-orange-400" />
            scripts/run_inference_engine.sh (SGLang TP=2 Daemon)
          </span>
          <button
            onClick={() => handleCopy(sglangScript, 'sglang')}
            className="text-[10px] mono text-gray-300 hover:text-white flex items-center gap-1 px-2.5 py-1 rounded bg-[#181a24] border border-[#262838] transition cursor-pointer"
          >
            {copied === 'sglang' ? <Check className="w-3 h-3 text-emerald-400" /> : <Copy className="w-3 h-3" />}
            Copy Script
          </button>
        </div>
        <pre className="bg-[#0b0c10] border border-[#232530] rounded-lg p-3.5 text-[11px] mono text-orange-200/90 overflow-x-auto leading-relaxed font-mono">
          {sglangScript}
        </pre>
      </div>
    </div>
  );
};
