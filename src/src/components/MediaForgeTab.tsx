import React, { useState } from 'react';
import {
  Sparkles,
  Image as ImageIcon,
  Video,
  Sliders,
  Download,
  Play,
  RotateCcw,
  Zap,
  Layers,
  Cpu,
  Eye,
  CheckCircle2,
} from 'lucide-react';
import { mediaGenerateImage, mediaGenerateVideo, DiffusionResponse, VideoGenerationResponse } from '../lib/desktop';

export const MediaForgeTab: React.FC = () => {
  const [activeMode, setActiveMode] = useState<'image' | 'video'>('image');
  const [prompt, setPrompt] = useState('High precision multi-layer PCB layout with gold traces, FPGA chip, and glowing SMD LEDs');
  const [negativePrompt, setNegativePrompt] = useState('blurry, low resolution, distorted, watermark');
  const [steps, setSteps] = useState(4);
  const [guidanceScale, setGuidanceScale] = useState(7.5);
  const [scheduler, setScheduler] = useState('FlowMatchEuler');
  const [seed, setSeed] = useState<number | undefined>(undefined);
  const [width, setWidth] = useState(1024);
  const [height, setHeight] = useState(1024);

  // Video specific state
  const [numFrames, setNumFrames] = useState(49);
  const [fps, setFps] = useState(24);

  const [generating, setGenerating] = useState(false);
  const [lastImageResult, setLastImageResult] = useState<DiffusionResponse | null>(null);
  const [lastVideoResult, setLastVideoResult] = useState<VideoGenerationResponse | null>(null);
  const [error, setError] = useState<string | null>(null);

  const handleGenerate = async () => {
    if (!prompt.trim() || generating) return;
    setGenerating(true);
    setError(null);

    try {
      if (activeMode === 'image') {
        const res = await mediaGenerateImage(prompt, {
          negative_prompt: negativePrompt,
          width,
          height,
          steps,
          guidance_scale: guidanceScale,
          seed,
          scheduler,
        });
        setLastImageResult(res);
      } else {
        const res = await mediaGenerateVideo(prompt, {
          num_frames: numFrames,
          fps,
          width: 768,
          height: 512,
        });
        setLastVideoResult(res);
      }
    } catch (err: any) {
      setError(err?.message || String(err));
    } finally {
      setGenerating(false);
    }
  };

  return (
    <div className="flex flex-col lg:flex-row gap-5 h-[calc(100vh-140px)] min-h-[600px] font-sans">
      {/* 1. Left Controls Panel */}
      <div className="w-full lg:w-96 shrink-0 bg-[#111113] border border-[#27272A] rounded-xl p-5 shadow-lg flex flex-col justify-between overflow-y-auto scrollbar-thin">
        <div className="space-y-4">
          <div className="flex items-center justify-between pb-3 border-b border-[#27272A]">
            <div className="flex items-center gap-2">
              <Sparkles className="w-4 h-4 text-purple-400" />
              <h3 className="text-xs font-bold uppercase tracking-wider text-[#FAFAFA]">Generative Media Forge</h3>
            </div>
            <span className="text-[9px] font-mono px-2 py-0.5 rounded bg-purple-500/10 text-purple-400 border border-purple-500/20 font-semibold">
              CANDLE & ORT
            </span>
          </div>

          {/* Mode Switcher */}
          <div className="grid grid-cols-2 gap-2 bg-[#18181b] p-1 rounded-lg border border-[#27272A]">
            <button
              onClick={() => setActiveMode('image')}
              className={`py-2 px-3 rounded-md text-xs font-mono font-bold flex items-center justify-center gap-1.5 transition ${
                activeMode === 'image'
                  ? 'bg-purple-600 text-white shadow-md'
                  : 'text-zinc-400 hover:text-white'
              }`}
            >
              <ImageIcon className="w-3.5 h-3.5" />
              Flux / SDXL
            </button>
            <button
              onClick={() => setActiveMode('video')}
              className={`py-2 px-3 rounded-md text-xs font-mono font-bold flex items-center justify-center gap-1.5 transition ${
                activeMode === 'video'
                  ? 'bg-purple-600 text-white shadow-md'
                  : 'text-zinc-400 hover:text-white'
              }`}
            >
              <Video className="w-3.5 h-3.5" />
              LTX Video
            </button>
          </div>

          {/* Prompt Box */}
          <div className="space-y-1.5">
            <label className="text-[11px] font-mono uppercase text-zinc-400 font-semibold">Positive Prompt</label>
            <textarea
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
              rows={3}
              className="w-full bg-[#18181b] border border-[#27272A] rounded-lg p-2.5 text-xs text-zinc-200 focus:border-purple-500 focus:outline-none font-mono resize-none"
              placeholder="Describe the image or schematic to synthesize…"
            />
          </div>

          {/* Negative Prompt Box */}
          {activeMode === 'image' && (
            <div className="space-y-1.5">
              <label className="text-[11px] font-mono uppercase text-zinc-400 font-semibold">Negative Prompt</label>
              <textarea
                value={negativePrompt}
                onChange={(e) => setNegativePrompt(e.target.value)}
                rows={2}
                className="w-full bg-[#18181b] border border-[#27272A] rounded-lg p-2 text-xs text-zinc-200 focus:border-purple-500 focus:outline-none font-mono resize-none"
                placeholder="Undesired artifacts, styles…"
              />
            </div>
          )}

          {/* Sampling Parameters */}
          <div className="space-y-3 pt-2 border-t border-[#27272A]">
            <div className="space-y-1">
              <div className="flex justify-between text-xs font-mono">
                <span className="text-zinc-400">Diffusion Steps</span>
                <span className="text-[#FAFAFA] font-bold">{steps}</span>
              </div>
              <input
                type="range"
                min="1"
                max="50"
                step="1"
                value={steps}
                onChange={(e) => setSteps(parseInt(e.target.value))}
                className="w-full accent-purple-500 bg-[#18181b] h-1.5 rounded-lg cursor-pointer"
              />
              <div className="flex justify-between text-[9px] font-mono text-zinc-500">
                <span>Fast Turbo (1-4)</span>
                <span>High Quality (20-50)</span>
              </div>
            </div>

            {activeMode === 'image' && (
              <>
                <div className="space-y-1">
                  <label className="text-[10px] font-mono uppercase text-zinc-400 font-semibold">Image Model</label>
                  <select
                    value={scheduler}
                    onChange={(e) => setScheduler(e.target.value)}
                    className="w-full bg-[#18181b] border border-[#27272A] rounded-lg p-2 text-xs font-mono text-zinc-200 focus:border-purple-500 outline-none"
                  >
                    <option value="FlowMatchEuler">Flux.1-schnell (4-step FP8 Distilled)</option>
                    <option value="DPMSolverPlusPlus">SDXL Turbo (Real-time)</option>
                    <option value="DDIM">Stable Diffusion 3.5 Medium</option>
                  </select>
                </div>

                <div className="space-y-1">
                  <div className="flex justify-between text-xs font-mono">
                    <span className="text-zinc-400">Guidance Scale (CFG)</span>
                    <span className="text-[#FAFAFA] font-bold">{guidanceScale.toFixed(1)}</span>
                  </div>
                  <input
                    type="range"
                    min="0.0"
                    max="15.0"
                    step="0.5"
                    value={guidanceScale}
                    onChange={(e) => setGuidanceScale(parseFloat(e.target.value))}
                    className="w-full accent-purple-500 bg-[#18181b] h-1.5 rounded-lg cursor-pointer"
                  />
                </div>
              </>
            )}

            {activeMode === 'video' && (
              <>
                <div className="space-y-1">
                  <label className="text-[10px] font-mono uppercase text-zinc-400 font-semibold">Video Generator Engine</label>
                  <select
                    className="w-full bg-[#18181b] border border-[#27272A] rounded-lg p-2 text-xs font-mono text-zinc-200 focus:border-purple-500 outline-none"
                  >
                    <option value="ltx-distilled">LTX-2.3 (Distilled 4.5x Fast Sampler - 8 Steps)</option>
                    <option value="ltx-standard">LTX-Video 0.9.1 (Standard 30 Steps)</option>
                    <option value="wan-2.1">Wan 2.1 T2V (1.3B DiT)</option>
                  </select>
                </div>

                <div className="space-y-1">
                  <div className="flex justify-between text-xs font-mono">
                    <span className="text-zinc-400">Frames Count</span>
                    <span className="text-[#FAFAFA] font-bold">{numFrames} frames ({fps} fps)</span>
                  </div>
                  <input
                    type="range"
                    min="16"
                    max="97"
                    step="8"
                    value={numFrames}
                    onChange={(e) => setNumFrames(parseInt(e.target.value))}
                    className="w-full accent-purple-500 bg-[#18181b] h-1.5 rounded-lg cursor-pointer"
                  />
                </div>

                <div className="flex items-center justify-between p-2.5 bg-[#18181b] rounded-lg border border-[#27272A]">
                  <div>
                    <div className="text-xs font-bold text-white">FP8 Block-wise Quant</div>
                    <div className="text-[10px] text-zinc-400">Save 45% VRAM on 16GB GPUs</div>
                  </div>
                  <input
                    type="checkbox"
                    defaultChecked
                    className="w-4 h-4 accent-purple-500 rounded cursor-pointer"
                  />
                </div>
              </>
            )}
          </div>
        </div>

        {/* Generate Button */}
        <div className="pt-4 border-t border-[#27272A] space-y-2">
          <button
            onClick={handleGenerate}
            disabled={generating || !prompt.trim()}
            className="w-full py-3 rounded-lg bg-gradient-to-r from-purple-600 to-indigo-600 hover:from-purple-500 hover:to-indigo-500 text-white text-xs font-bold uppercase tracking-wider transition flex items-center justify-center gap-2 cursor-pointer shadow-lg disabled:opacity-50"
          >
            <Sparkles className={`w-4 h-4 ${generating ? 'animate-spin' : ''}`} />
            {generating ? 'Synthesizing with Pure Rust...' : `Generate ${activeMode === 'image' ? 'Image' : 'Video'}`}
          </button>
          <div className="text-center text-[10px] mono text-zinc-500">
            Native CUDA PTX / Metal Tensor Pipeline
          </div>
        </div>
      </div>

      {/* 2. Main Visual Canvas Viewport */}
      <div className="flex-1 bg-[#111113] border border-[#27272A] rounded-xl flex flex-col shadow-lg overflow-hidden relative">
        <div className="p-3.5 border-b border-[#27272A] bg-[#0A0A0A] flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="w-2 h-2 rounded-full bg-purple-500 animate-pulse" />
            <span className="text-xs font-bold text-[#FAFAFA]">Viewport Canvas</span>
            <span className="text-[10px] font-mono text-zinc-500">· {activeMode.toUpperCase()} Output</span>
          </div>
          {lastImageResult && (
            <div className="text-[10px] font-mono text-purple-400">
              Generated in {lastImageResult.generation_time_ms} ms · Seed: {lastImageResult.seed}
            </div>
          )}
        </div>

        <div className="flex-1 flex items-center justify-center p-6 bg-[#08080a] relative overflow-hidden">
          {generating ? (
            <div className="text-center space-y-3">
              <div className="w-12 h-12 border-2 border-purple-500 border-t-transparent rounded-full animate-spin mx-auto" />
              <div className="text-xs font-mono text-purple-300 animate-pulse">
                Running Candle {activeMode === 'image' ? 'Diffusion UNet/DiT' : 'Video VAE'} Denoising Steps…
              </div>
              <div className="text-[10px] mono text-zinc-500">Zero Python / 100% In-Memory CUDA PTX</div>
            </div>
          ) : lastImageResult?.image_bytes_base64 ? (
            <div className="relative max-w-full max-h-full rounded-lg overflow-hidden border border-[#27272A] shadow-2xl">
              <img
                src={`data:image/png;base64,${lastImageResult.image_bytes_base64}`}
                alt="Synthesized Output"
                className="max-h-[70vh] object-contain rounded-lg"
              />
            </div>
          ) : (
            <div className="text-center space-y-3 max-w-sm">
              <div className="w-16 h-16 rounded-2xl bg-purple-500/10 border border-purple-500/20 flex items-center justify-center mx-auto text-purple-400">
                {activeMode === 'image' ? <ImageIcon className="w-8 h-8" /> : <Video className="w-8 h-8" />}
              </div>
              <h4 className="text-sm font-bold text-white">No Output Rendered Yet</h4>
              <p className="text-xs text-zinc-400">
                Enter a visual prompt or PCB description on the left and click Generate to execute native in-memory diffusion.
              </p>
            </div>
          )}

          {error && (
            <div className="absolute bottom-4 left-4 right-4 p-3 bg-red-500/10 border border-red-500/30 text-red-300 text-xs font-mono rounded-lg">
              ⚠️ {error}
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
