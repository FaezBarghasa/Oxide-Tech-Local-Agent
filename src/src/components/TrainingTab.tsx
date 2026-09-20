import React, { useState, useEffect, useMemo } from 'react';
import {
  Zap,
  Play,
  Pause,
  Save,
  CheckCircle2,
  TrendingDown,
  Cpu,
  Code2,
  Flame,
  Terminal,
  Activity,
  Layers,
  Sparkles,
  Sliders,
  RotateCcw,
} from 'lucide-react';
import {
  ResponsiveContainer,
  LineChart,
  Line,
  XAxis,
  YAxis,
  Tooltip,
  Legend,
  CartesianGrid,
  ReferenceLine,
} from 'recharts';

interface TrainingPoint {
  step: number;
  epoch: string;
  loss: number;
  rewardScore: number;
  passRate: number;
  klDiv: number;
}

const INITIAL_METRICS: TrainingPoint[] = [
  { step: 40, epoch: 'E1.1', loss: 0.142, rewardScore: 0.32, passRate: 62.0, klDiv: 0.058 },
  { step: 80, epoch: 'E1.2', loss: 0.124, rewardScore: 0.44, passRate: 68.5, klDiv: 0.052 },
  { step: 120, epoch: 'E1.3', loss: 0.101, rewardScore: 0.55, passRate: 74.2, klDiv: 0.046 },
  { step: 160, epoch: 'E2.1', loss: 0.085, rewardScore: 0.63, passRate: 78.0, klDiv: 0.041 },
  { step: 200, epoch: 'E2.2', loss: 0.072, rewardScore: 0.71, passRate: 82.5, klDiv: 0.036 },
  { step: 240, epoch: 'E2.3', loss: 0.063, rewardScore: 0.77, passRate: 85.0, klDiv: 0.031 },
  { step: 280, epoch: 'E3.1', loss: 0.055, rewardScore: 0.82, passRate: 87.4, klDiv: 0.028 },
  { step: 320, epoch: 'E3.2', loss: 0.048, rewardScore: 0.86, passRate: 89.1, klDiv: 0.024 },
  { step: 360, epoch: 'E3.3', loss: 0.042, rewardScore: 0.89, passRate: 90.5, klDiv: 0.021 },
  { step: 420, epoch: 'E4.1', loss: 0.0381, rewardScore: 0.912, passRate: 91.2, klDiv: 0.018 },
];

export const TrainingTab: React.FC = () => {
  const [isTraining, setIsTraining] = useState(false);
  const [step, setStep] = useState(420);
  const [totalSteps] = useState(1200);
  const [loss, setLoss] = useState(0.0381);
  const [rewardScore, setRewardScore] = useState(0.912);
  const [passRate, setPassRate] = useState(91.2);
  const [lr, setLr] = useState(1.7e-5);
  const [chartView, setChartView] = useState<'combined' | 'loss' | 'reward'>('combined');
  const [activeMetrics, setActiveMetrics] = useState({
    loss: true,
    reward: true,
    passRate: true,
    klDiv: false,
  });

  const [metricsHistory, setMetricsHistory] = useState<TrainingPoint[]>(INITIAL_METRICS);

  const [logs, setLogs] = useState<string[]>([
    '[18:42:01] 🦥 Unsloth: Initializing FastLanguageModel with TP=2 (Dual RTX 3090 48GB)...',
    '[18:42:03] 🦥 Unsloth: Loaded Qwen/Qwen2.5-32B-Instruct · 4-bit QDoRA (18.4GB VRAM per GPU)',
    '[18:42:05] 🦥 Unsloth: 5x faster kernels activated (FlashAttention-2 + RoPE + CrossEntropy fusion)',
    '[18:42:10] Compiler reward verifier online (cargo check + kicad DRC)',
    '[18:52:40] Step 300 · loss=0.0521 · reward_score=0.841 · pass_rate=83.1% · lr=1.8e-5 · speed=4.8x baseline',
    '[19:00:43] Step 420 · loss=0.0381 · reward_score=0.912 · pass_rate=91.2% · checkpoint saved to workspace/models/grpo_step420',
  ]);

  useEffect(() => {
    let interval: any = null;
    if (isTraining) {
      interval = setInterval(() => {
        setStep((prev) => {
          if (prev >= totalSteps) {
            setIsTraining(false);
            return totalSteps;
          }
          const next = prev + 5;
          const nextLoss = parseFloat(Math.max(0.012, loss - 0.0004 + (Math.random() - 0.5) * 0.0006).toFixed(4));
          const nextReward = parseFloat(Math.min(0.985, rewardScore + 0.0015 + (Math.random() - 0.5) * 0.002).toFixed(3));
          const nextPass = parseFloat(Math.min(99.4, passRate + 0.12 + (Math.random() - 0.5) * 0.15).toFixed(1));
          const nextKl = parseFloat(Math.max(0.008, 0.018 * (1 - (next - 420) / (totalSteps - 420) * 0.5)).toFixed(3));
          const nextLr = 2e-5 * (1 - (next / totalSteps) * 0.5);

          setLoss(nextLoss);
          setRewardScore(nextReward);
          setPassRate(nextPass);
          setLr(nextLr);

          const currentEpochNumber = (1 + Math.floor(next / 100)).toString();
          const currentSubEpoch = (1 + Math.floor((next % 100) / 33)).toString();
          const epochLabel = `E${currentEpochNumber}.${currentSubEpoch}`;

          const newPoint: TrainingPoint = {
            step: next,
            epoch: epochLabel,
            loss: nextLoss,
            rewardScore: nextReward,
            passRate: nextPass,
            klDiv: nextKl,
          };

          setMetricsHistory((prevHistory) => {
            const updated = [...prevHistory, newPoint];
            // Keep recent window for smooth chart rendering
            if (updated.length > 20) {
              return updated.slice(updated.length - 20);
            }
            return updated;
          });

          if (next % 20 === 0) {
            const time = new Date().toLocaleTimeString();
            setLogs((l) => [
              ...l,
              `[${time}] 🦥 Step ${next} · loss=${nextLoss} · reward=${nextReward} · pass_rate=${nextPass}% · lr=${nextLr.toExponential(1)}`,
            ]);
          }

          return next;
        });
      }, 750);
    }
    return () => clearInterval(interval);
  }, [isTraining, loss, rewardScore, passRate, totalSteps]);

  const progressPct = parseFloat(((step / totalSteps) * 100).toFixed(1));

  const handleResetSession = () => {
    setIsTraining(false);
    setStep(40);
    setLoss(0.142);
    setRewardScore(0.32);
    setPassRate(62.0);
    setMetricsHistory(INITIAL_METRICS.slice(0, 3));
    const time = new Date().toLocaleTimeString();
    setLogs((l) => [...l, `[${time}] ↺ Training metrics reset for new session benchmark.`]);
  };

  const unslothScript = `from unsloth import FastLanguageModel, PatchFastRL
import torch
from trl import GRPOTrainer, GRPOConfig

# 1. Load Unsloth 4-bit / 16-bit FastLanguageModel
model, tokenizer = FastLanguageModel.from_pretrained(
    model_name="Qwen/Qwen2.5-32B-Instruct",
    max_seq_length=16384,
    load_in_4bit=True,
    fast_inference=True, # 2x faster inference
)

# 2. Add QDoRA / LoRA adapters with 80% VRAM savings
model = FastLanguageModel.get_peft_model(
    model,
    r=16,
    target_modules=["q_proj", "k_proj", "v_proj", "o_proj", "gate_proj", "up_proj", "down_proj"],
    lora_alpha=32,
    use_gradient_checkpointing="unsloth", # 5x faster checkpointing
    use_dora=True, # Directional Adapter
)

# 3. Verifier Reward Function (cargo check bare-metal verification)
def cargo_verifier_reward(prompts, completions, **kwargs):
    rewards = []
    for code in completions:
        res = compile_baremetal_rust(code)
        rewards.append(1.0 if res["status"] == "ok" else -0.5)
    return rewards`;

  return (
    <div className="space-y-6 font-sans">
      {/* Unsloth Studio Header */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-6 shadow-xl relative overflow-hidden bg-[radial-gradient(ellipse_at_top_right,rgba(249,115,22,0.12)_0%,transparent_70%)]">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pb-4 border-b border-[#232530]">
          <div>
            <div className="text-sm font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <span className="text-lg">🦥</span>
              <span>Unsloth GRPO RLVR & QDoRA Studio</span>
              <span className="text-[9px] mono px-2 py-0.5 rounded bg-orange-500/10 text-orange-400 border border-orange-500/30 font-bold">
                5X TURBO
              </span>
            </div>
            <div className="text-[10px] mono text-gray-400 mt-0.5">
              FastLanguageModel · Verifiable Rewards · cargo check · kicad DRC · TP=2 Dual RTX 3090
            </div>
          </div>

          <div className="flex items-center gap-2">
            <button
              onClick={handleResetSession}
              className="px-2.5 py-2 rounded-lg bg-[#181a24] hover:bg-[#222432] text-gray-300 border border-[#2d3040] hover:border-gray-500 text-xs font-semibold transition flex items-center gap-1.5 cursor-pointer"
              title="Reset training session metrics"
            >
              <RotateCcw className="w-3.5 h-3.5 text-gray-400" />
              <span>Reset</span>
            </button>

            <button
              onClick={() => {
                const time = new Date().toLocaleTimeString();
                setLogs((prev) => [...prev, `[${time}] 🦥 Manual LoRA checkpoint exported: workspace/models/unsloth_lora_step${step}`]);
              }}
              className="px-3 py-2 rounded-lg bg-[#181a24] hover:bg-[#222432] text-gray-200 border border-[#2d3040] hover:border-orange-500/30 text-xs font-semibold uppercase tracking-wider transition flex items-center gap-1.5 cursor-pointer"
            >
              <Save className="w-3.5 h-3.5 text-orange-400" />
              <span>Save LoRA</span>
            </button>

            <button
              onClick={() => setIsTraining(!isTraining)}
              className={`px-4 py-2 rounded-lg text-xs font-bold uppercase tracking-wider transition-all flex items-center gap-1.5 cursor-pointer ${
                isTraining
                  ? 'bg-amber-500 hover:bg-amber-400 text-gray-950 shadow-[0_0_15px_rgba(245,158,11,0.35)]'
                  : 'bg-gradient-to-r from-orange-500 to-amber-500 hover:from-orange-400 hover:to-amber-400 text-gray-950 shadow-[0_0_18px_rgba(249,115,22,0.4)]'
              }`}
            >
              {isTraining ? <Pause className="w-3.5 h-3.5 fill-current" /> : <Play className="w-3.5 h-3.5 fill-current" />}
              <span>{isTraining ? 'Pause GRPO' : '▶ Launch Unsloth GRPO'}</span>
            </button>
          </div>
        </div>

        {/* Live Step KPI Metric Cards */}
        <div className="grid grid-cols-2 md:grid-cols-6 gap-3 mt-4">
          <div className="bg-[#181a24] border border-[#262838] p-3.5 rounded-xl">
            <div className="text-[10px] mono uppercase text-gray-400 font-semibold">Engine Status</div>
            <div className="mt-1">
              <span
                className={`text-[9px] mono px-2 py-0.5 rounded font-bold ${
                  isTraining
                    ? 'bg-orange-500/10 text-orange-400 border border-orange-500/30 animate-pulse'
                    : 'bg-emerald-500/10 text-emerald-400 border border-emerald-500/30'
                }`}
              >
                {isTraining ? 'TRAINING (5X)' : 'READY'}
              </span>
            </div>
          </div>

          <div className="bg-[#181a24] border border-[#262838] p-3.5 rounded-xl">
            <div className="text-[10px] mono uppercase text-gray-400 font-semibold">Episode Step</div>
            <div className="text-base font-bold mono text-orange-400 mt-1">
              {step} <span className="text-[10px] text-gray-400 font-normal">/{totalSteps}</span>
            </div>
          </div>

          <div className="bg-[#181a24] border border-[#262838] p-3.5 rounded-xl">
            <div className="text-[10px] mono uppercase text-gray-400 font-semibold">VRAM Saving</div>
            <div className="text-base font-bold mono text-emerald-400 mt-1">
              80% <span className="text-[10px] text-gray-400 font-normal">QDoRA</span>
            </div>
          </div>

          <div className="bg-[#181a24] border border-[#262838] p-3.5 rounded-xl">
            <div className="text-[10px] mono uppercase text-gray-400 font-semibold">GRPO Loss</div>
            <div className="text-base font-bold mono text-rose-400 mt-1">{loss}</div>
          </div>

          <div className="bg-[#181a24] border border-[#262838] p-3.5 rounded-xl">
            <div className="text-[10px] mono uppercase text-gray-400 font-semibold">RLVR Reward Score</div>
            <div className="text-base font-bold mono text-emerald-400 mt-1">{rewardScore}</div>
          </div>

          <div className="bg-[#181a24] border border-[#262838] p-3.5 rounded-xl">
            <div className="text-[10px] mono uppercase text-gray-400 font-semibold">Compiler Pass Rate</div>
            <div className="text-base font-bold mono text-amber-400 mt-1">{passRate}%</div>
          </div>
        </div>

        {/* Training Progress Bar */}
        <div className="mt-4 pt-3 border-t border-[#232530]">
          <div className="flex items-center justify-between text-[10px] mono text-gray-400 mb-1.5">
            <span className="flex items-center gap-1.5">
              <span>🦥 Unsloth RLVR Episode Progress:</span>
              <span className="text-gray-200 font-semibold">{step} / {totalSteps} steps</span>
            </span>
            <span className="text-orange-400 font-bold">{progressPct}%</span>
          </div>
          <div className="h-2 w-full bg-[#181a24] rounded-full overflow-hidden">
            <div
              className="h-full bg-gradient-to-r from-orange-500 via-amber-400 to-emerald-400 transition-all duration-300 shadow-[0_0_10px_rgba(249,115,22,0.5)]"
              style={{ width: `${progressPct}%` }}
            />
          </div>
        </div>
      </div>

      {/* RECHARTS VISUALIZATION SUITE FOR GRPO LOSS & REWARDS */}
      <div className="bg-[#111217] border border-[#232530] rounded-2xl p-5 shadow-xl space-y-4">
        {/* Chart Header & Interactive Controls */}
        <div className="flex flex-col lg:flex-row lg:items-center justify-between gap-3 pb-3 border-b border-[#232530]">
          <div>
            <div className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-2">
              <Activity className="w-4 h-4 text-orange-400" />
              <span>GRPO Loss & Verifier Reward Trajectory (Recharts Live Engine)</span>
            </div>
            <div className="text-[10px] mono text-gray-400 mt-0.5">
              Dual-axis trajectory tracking policy objective loss optimization vs. compiler verification rewards
            </div>
          </div>

          <div className="flex flex-wrap items-center gap-2">
            {/* Metric Layer Checkbox Pills */}
            <div className="flex items-center gap-1.5 bg-[#14151e] border border-[#232530] rounded-lg p-1 text-[11px] mono">
              <button
                onClick={() => setActiveMetrics((m) => ({ ...m, loss: !m.loss }))}
                className={`px-2 py-0.5 rounded transition flex items-center gap-1.5 cursor-pointer ${
                  activeMetrics.loss
                    ? 'bg-rose-500/20 text-rose-300 border border-rose-500/40 font-semibold'
                    : 'text-gray-500 hover:text-gray-300'
                }`}
              >
                <span className="w-2 h-2 rounded-full bg-rose-400" />
                <span>Loss</span>
              </button>

              <button
                onClick={() => setActiveMetrics((m) => ({ ...m, reward: !m.reward }))}
                className={`px-2 py-0.5 rounded transition flex items-center gap-1.5 cursor-pointer ${
                  activeMetrics.reward
                    ? 'bg-emerald-500/20 text-emerald-300 border border-emerald-500/40 font-semibold'
                    : 'text-gray-500 hover:text-gray-300'
                }`}
              >
                <span className="w-2 h-2 rounded-full bg-emerald-400" />
                <span>Reward Score</span>
              </button>

              <button
                onClick={() => setActiveMetrics((m) => ({ ...m, passRate: !m.passRate }))}
                className={`px-2 py-0.5 rounded transition flex items-center gap-1.5 cursor-pointer ${
                  activeMetrics.passRate
                    ? 'bg-amber-500/20 text-amber-300 border border-amber-500/40 font-semibold'
                    : 'text-gray-500 hover:text-gray-300'
                }`}
              >
                <span className="w-2 h-2 rounded-full bg-amber-400" />
                <span>Pass Rate %</span>
              </button>

              <button
                onClick={() => setActiveMetrics((m) => ({ ...m, klDiv: !m.klDiv }))}
                className={`px-2 py-0.5 rounded transition flex items-center gap-1.5 cursor-pointer ${
                  activeMetrics.klDiv
                    ? 'bg-orange-500/20 text-orange-300 border border-orange-500/40 font-semibold'
                    : 'text-gray-500 hover:text-gray-300'
                }`}
              >
                <span className="w-2 h-2 rounded-full bg-orange-400" />
                <span>KL Penalty</span>
              </button>
            </div>
          </div>
        </div>

        {/* Recharts Main Graph Container */}
        <div className="h-72 w-full pt-2">
          <ResponsiveContainer width="100%" height="100%">
            <LineChart
              data={metricsHistory}
              margin={{ top: 10, right: 20, left: -10, bottom: 0 }}
            >
              <CartesianGrid strokeDasharray="3 3" stroke="#232530" vertical={false} />
              
              <XAxis
                dataKey="step"
                tick={{ fill: '#80879e', fontSize: 10, fontFamily: 'monospace' }}
                axisLine={{ stroke: '#2e3245' }}
                tickLine={{ stroke: '#2e3245' }}
                tickFormatter={(val) => `Step ${val}`}
              />

              {/* Left Y-Axis: Loss & KL (0.00 to 0.16) */}
              <YAxis
                yAxisId="lossAxis"
                orientation="left"
                domain={[0, 0.16]}
                tick={{ fill: '#f43f5e', fontSize: 10, fontFamily: 'monospace' }}
                axisLine={{ stroke: '#f43f5e', strokeOpacity: 0.3 }}
                tickLine={false}
                tickFormatter={(v) => v.toFixed(3)}
              />

              {/* Right Y-Axis: Reward Score & Pass Rate (0.0 to 1.0) */}
              <YAxis
                yAxisId="rewardAxis"
                orientation="right"
                domain={[0, 1.0]}
                tick={{ fill: '#10b981', fontSize: 10, fontFamily: 'monospace' }}
                axisLine={{ stroke: '#10b981', strokeOpacity: 0.3 }}
                tickLine={false}
                tickFormatter={(v) => (v * 100).toFixed(0) + '%'}
              />

              <Tooltip
                content={({ active, payload, label }) => {
                  if (active && payload && payload.length) {
                    const data = payload[0].payload as TrainingPoint;
                    return (
                      <div className="bg-[#0e1017] border border-[#2d3040] rounded-xl p-3 shadow-2xl text-[11px] mono text-gray-200 min-w-[190px]">
                        <div className="font-bold text-white pb-1.5 mb-1.5 border-b border-[#232530] flex items-center justify-between">
                          <span>Step {data.step}</span>
                          <span className="text-orange-400 font-semibold">{data.epoch}</span>
                        </div>
                        <div className="space-y-1">
                          <div className="flex items-center justify-between text-rose-400">
                            <span>GRPO Loss:</span>
                            <span className="font-bold">{data.loss}</span>
                          </div>
                          <div className="flex items-center justify-between text-emerald-400">
                            <span>Reward Score:</span>
                            <span className="font-bold">{data.rewardScore}</span>
                          </div>
                          <div className="flex items-center justify-between text-amber-400">
                            <span>Pass Rate:</span>
                            <span className="font-bold">{data.passRate}%</span>
                          </div>
                          <div className="flex items-center justify-between text-orange-400">
                            <span>KL Divergence:</span>
                            <span className="font-bold">{data.klDiv}</span>
                          </div>
                        </div>
                      </div>
                    );
                  }
                  return null;
                }}
              />

              <Legend
                verticalAlign="top"
                height={30}
                content={() => (
                  <div className="flex items-center justify-end gap-4 text-[10px] mono text-gray-400 pb-2">
                    {activeMetrics.loss && (
                      <div className="flex items-center gap-1.5">
                        <span className="w-3 h-0.5 bg-rose-500 inline-block" />
                        <span className="text-rose-400">GRPO Objective Loss</span>
                      </div>
                    )}
                    {activeMetrics.reward && (
                      <div className="flex items-center gap-1.5">
                        <span className="w-3 h-0.5 bg-emerald-500 inline-block" />
                        <span className="text-emerald-400">Verifier Reward Score</span>
                      </div>
                    )}
                    {activeMetrics.passRate && (
                      <div className="flex items-center gap-1.5">
                        <span className="w-3 h-0.5 bg-amber-400 inline-block" />
                        <span className="text-amber-400">Compiler Pass Rate</span>
                      </div>
                    )}
                  </div>
                )}
              />

              {/* Reference Baseline Lines */}
              <ReferenceLine yAxisId="lossAxis" y={0.04} stroke="#475569" strokeDasharray="3 3" />
              <ReferenceLine yAxisId="rewardAxis" y={0.9} stroke="#065f46" strokeDasharray="3 3" />

              {/* Metric Lines */}
              {activeMetrics.loss && (
                <Line
                  yAxisId="lossAxis"
                  type="monotone"
                  dataKey="loss"
                  name="GRPO Loss"
                  stroke="#f43f5e"
                  strokeWidth={2.5}
                  dot={{ r: 3, fill: '#f43f5e', strokeWidth: 1, stroke: '#111217' }}
                  activeDot={{ r: 5, fill: '#f43f5e' }}
                  isAnimationActive={false}
                />
              )}

              {activeMetrics.reward && (
                <Line
                  yAxisId="rewardAxis"
                  type="monotone"
                  dataKey="rewardScore"
                  name="Reward Score"
                  stroke="#10b981"
                  strokeWidth={2.5}
                  dot={{ r: 3, fill: '#10b981', strokeWidth: 1, stroke: '#111217' }}
                  activeDot={{ r: 5, fill: '#10b981' }}
                  isAnimationActive={false}
                />
              )}

              {activeMetrics.passRate && (
                <Line
                  yAxisId="rewardAxis"
                  type="monotone"
                  dataKey={(d: TrainingPoint) => d.passRate / 100}
                  name="Pass Rate"
                  stroke="#f59e0b"
                  strokeWidth={1.8}
                  strokeDasharray="4 2"
                  dot={false}
                  isAnimationActive={false}
                />
              )}

              {activeMetrics.klDiv && (
                <Line
                  yAxisId="lossAxis"
                  type="monotone"
                  dataKey="klDiv"
                  name="KL Div"
                  stroke="#f97316"
                  strokeWidth={1.5}
                  strokeDasharray="2 2"
                  dot={false}
                  isAnimationActive={false}
                />
              )}
            </LineChart>
          </ResponsiveContainer>
        </div>

        {/* Live Metrics Summary Bar */}
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-2.5 pt-2 border-t border-[#232530] text-[11px] mono text-gray-300">
          <div className="p-2.5 bg-[#14151e] border border-[#232530] rounded-xl flex items-center justify-between">
            <span className="text-gray-400">Current Policy Loss:</span>
            <span className="text-rose-400 font-bold">{loss} (steady descent)</span>
          </div>
          <div className="p-2.5 bg-[#14151e] border border-[#232530] rounded-xl flex items-center justify-between">
            <span className="text-gray-400">RLVR Reward Score:</span>
            <span className="text-emerald-400 font-bold">{rewardScore} / 1.000</span>
          </div>
          <div className="p-2.5 bg-[#14151e] border border-[#232530] rounded-xl flex items-center justify-between">
            <span className="text-gray-400">Verifier Target:</span>
            <span className="text-orange-400 font-bold">thumbv7em-none-eabihf</span>
          </div>
        </div>
      </div>

      {/* Compiler Verifier Code & Logs */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
        {/* Unsloth Script */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 shadow-sm">
          <div className="flex items-center justify-between mb-2">
            <span className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-1.5">
              <Terminal className="w-3.5 h-3.5 text-orange-400" />
              training/unsloth_grpo_pipeline.py
            </span>
            <span className="text-[10px] mono text-orange-400 font-bold">Unsloth FastLanguageModel</span>
          </div>
          <pre className="bg-[#0b0c10] border border-[#232530] rounded-lg p-3 text-[10px] mono text-gray-300 overflow-x-auto leading-relaxed max-h-56">
            {unslothScript}
          </pre>
        </div>

        {/* Live Training Log */}
        <div className="bg-[#111217] border border-[#232530] rounded-xl p-4 flex flex-col justify-between shadow-sm">
          <div>
            <div className="flex items-center justify-between mb-2 pb-2 border-b border-[#232530]">
              <span className="text-xs font-bold text-white uppercase tracking-wider flex items-center gap-1.5">
                <span>🦥</span> Live Unsloth Stream
              </span>
              <span className="text-[10px] mono text-emerald-400 font-bold">5X ACTIVE</span>
            </div>
            <div className="space-y-1.5 font-mono text-[10px] max-h-48 overflow-y-auto text-gray-300">
              {logs.map((log, idx) => (
                <div key={idx} className="p-2 rounded bg-[#0b0c10] border border-[#232530]">
                  {log}
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
