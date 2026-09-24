# Oxide-Tech-Local-Agent: 1M-Token Paged DDR5 KV-Cache & Continuous GRPO Self-Training Specification

**Document Version:** 1.0.0  
**Target:** Local Workstation High-Context Inference, DDR5 Memory Tiering, and Autonomous GRPO Self-Distillation  
**Subsystems:** `crates/oxide-engines`, `crates/model-trainer`, `crates/oxide-kernels`, `crates/oxide-state`  
**Execution Constraint:** Local-First, Zero Cloud API Dependency, Zero Out-Of-Memory (OOM) on Single Workstation GPUs

---

## 1. Executive Architecture Topology

Consumer workstation GPUs (e.g., NVIDIA RTX 4090 with 24 GB VRAM) are fundamentally memory-bandwidth and capacity-constrained when servicing long-context inference ($S \ge 128\text{k}$ tokens) and full-parameter reinforcement learning. Standard autoregressive architectures exhaust VRAM during KV-cache allocation or backpropagation logit computation.

This specification details the dual-mechanism system solving these physical constraints:
1. **Paged DDR5 Host RAM KV-Cache Offload:** A hardware-aware memory tiering engine mapping intermediate attention key-value states between GPU VRAM and high-speed system DDR5 RAM over PCIe Gen 4/5.
2. **Autonomous Group Relative Policy Optimization (GRPO) Loop:** An on-workstation continuous fine-tuning loop updating local adapter weights from formally verified engineering trajectories during idle GPU cycles.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                          HIGH-CONTEXT MEMORY TOPOLOGY                       │
│                                                                             │
│  [Token Indices 0 .. 32]       ──► Tier 0: GPU VRAM (Attention Sinks, Pinned)
│  [Token Indices 32 .. S-4096]  ──► Tier 1: Host DDR5 RAM (Paged via DMA Prefetch)
│  [Token Indices S-4096 .. S]   ──► Tier 0: GPU VRAM (Sliding Working Window)
│                                                                             │
│                     Double-Buffered Asynchronous DMA                        │
│            GPU VRAM ◄=================================► Host DDR5           │
│                          `cudaMemcpyAsync` Stream                           │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Paged DDR5 System RAM KV-Cache Offload

### 2.1 Mathematical Allocation Model

The memory consumed by the key-value cache for sequence length $S$, layer count $L$, key-value head count $N_{\text{kv}}$, head dimension $d_{\text{head}}$, and element byte width $B_{\text{elem}}$ is governed by:

$$M_{\text{KV}}(S) = 2 \cdot L \cdot N_{\text{kv}} \cdot d_{\text{head}} \cdot S \cdot B_{\text{elem}}$$

For a representative 35B dense architecture:
- $L = 40$ transformer layers
- $N_{\text{kv}} = 8$ key-value heads (Grouped-Query Attention)
- $d_{\text{head}} = 128$ head dimensionality
- $B_{\text{elem}} = 1\text{ byte}$ ($8\text{-bit}$ quantized FP8 / INT8 cache)

At saturation horizons:
- $S = 32,768$ tokens: $M_{\text{KV}} \approx 2.68\text{ GB}$ (retained in GPU VRAM).
- $S = 131,072$ tokens: $M_{\text{KV}} \approx 10.74\text{ GB}$ (approaching VRAM exhaustion).
- $S = 1,000,000$ tokens: $M_{\text{KV}} \approx 81.92\text{ GB}$ (impossible in consumer VRAM).

### 2.2 Tiered Memory Layout & Paging State Machine

The KV cache is partitioned into three functional tiers:
1. **Attention Sinks ($t \in [0, 32)$):** Retained in pinned GPU VRAM to maintain softmax denominator stability and baseline attention entropy.
2. **Historical Eviction Pool ($t \in [32, S - W)$):** Evicted in $4\text{ KB}$ page clusters over PCIe Gen 4/5 into host DDR5 memory mapped via POSIX shared memory or pinned host buffers (`cudaHostAlloc`).
3. **Active Attention Window ($t \in [S - W, S)$, where $W = 4,096$):** Maintained strictly in GPU SRAM/VRAM for high-throughput FlashAttention kernel execution.

### 2.3 Asynchronous Double-Buffered DMA Pipeline

To eliminate PCIe transfer stalls during token generation:
- Host-to-Device prefetching executes concurrently with GEMM matrix multiplication of layer $l-1$:
  $$\text{Overlap: } \text{Compute}(L_{l}) \parallel \text{cudaMemcpyAsync}(\text{Pages}(L_{l+1}), \text{HostToDevice})$$
- Paged chunks are addressed via Virtual Page Tables (VPT) tracked in `crates/oxide-engines/src/tiered_kv_cache.rs`.

---

## 3. High-Efficiency Fused Compute Kernels

### 3.1 Chunked Cross-Entropy Loss Computation

Standard training loops materialize the complete logit matrix $Z \in \mathbb{R}^{B \times S \times V}$ in VRAM:
$$M_{\text{logits}} = B \cdot S \cdot V \cdot 4\text{ bytes}$$

For batch size $B=2$, sequence length $S=8,192$, and vocabulary $V=152,064$, $M_{\text{logits}} \approx 9.96\text{ GB}$, causing immediate Out-of-Memory (OOM) errors during backward passes on consumer GPUs.

**Chunked Implementation:** Compute cross-entropy iteratively over vocabulary slices $V_k$ of size $C = 4,096$:
$$\mathcal{L} = -\sum_{i=1}^S \left[ z_{i, y_i} - \ln \left( \sum_{k=1}^{\lceil V/C \rceil} \sum_{j \in V_k} e^{z_{i, j} - m_i} \right) - m_i \right], \quad m_i = \max_j (z_{i, j})$$

- Peak intermediate logit allocation is bounded by:
  $$M_{\text{chunk}} = B \cdot S \cdot C \cdot 4\text{ bytes} \approx 268.4\text{ MB}$$
  representing a $>97\%$ reduction in peak transient VRAM consumption.

### 3.2 Fused SwiGLU Forward / Backward Kernels

Eliminate memory round-trips to GPU global memory by fusing SwiGLU activation into a single register-resident CUDA/Triton kernel:
$$\text{SwiGLU}(x, W, V) = (xW \cdot \sigma(xW)) \odot xV$$

---

## 4. Continuous Group Relative Policy Optimization (GRPO) Loop

### 4.1 Objective Formulation

The agent policy $\pi_\theta$ is continuously optimized against reference policy $\pi_{\text{ref}}$ without maintaining a separate critic model:

$$\mathcal{J}_{\text{GRPO}}(\theta) = \mathbb{E} \left[ \frac{1}{G} \sum_{i=1}^G \left( \min\left( \frac{\pi_\theta(o_i\vert{}q)}{\pi_{\text{ref}}(o_i\vert{}q)} \hat{A}_i, \; \text{clip}\left(\frac{\pi_\theta(o_i\vert{}q)}{\pi_{\text{ref}}(o_i\vert{}q)}, 1-\epsilon, 1+\epsilon\right) \hat{A}_i \right) - \beta D_{\text{KL}}(\pi_\theta \parallel \pi_{\text{ref}}) \right) \right]$$

where group size $G = 8$, clipping threshold $\epsilon = 0.2$, and KL coefficient $\beta = 0.04$.

### 4.2 Multi-Domain Composite Reward Function

The advantage $\hat{A}_i$ is normalized across the group outputs $o_i \in \{o_1, \dots, o_G\}$:
$$\hat{A}_i = \frac{R(o_i) - \mu_R}{\sigma_R + \delta}$$

where the scalar reward $R(o_i)$ combines formal compiler, symbolic verification, and hardware physical metrics:
$$R(o_i) = w_1 R_{\text{compile}} + w_2 R_{\text{formal\_verify}} + w_3 R_{\text{drc\_pass}} + w_4 R_{\text{sim\_continuity}} + w_5 R_{\text{thermal\_margin}}$$

| Metric | Domain | Value Space | Evaluation Oracle |
| :--- | :--- | :--- | :--- |
| $R_{\text{compile}}$ | Software | $\{0, 1\}$ | `cargo check --workspace` & `rustc` |
| $R_{\text{formal\_verify}}$ | Formal Logic | $[0, 1]$ | Z3 SMT-LIB2 proved assertion ratio |
| $R_{\text{drc\_pass}}$ | Electronics | $\{0, 1\}$ | KiCad CLI Electronic & Design Rules Check |
| $R_{\text{sim\_continuity}}$ | Circuits | $[0, 1]$ | ngspice electrical transient convergence |
| $R_{\text{thermal\_margin}}$ | Physics | $[0, 1]$ | GNN-FEM maximum surface temperature $\le 70^\circ\text{C}$ |

### 4.3 Rollout Buffer & Idle Workstation Distillation

- Successful trajectories with $R(o_i) \ge 0.95$ are committed to `crates/data/grpo_rollouts.jsonl`.
- When workstation telemetry indicates user inactivity ($> 15\text{ minutes}$) and GPU temperature $\le 55^\circ\text{C}$, the `model-trainer` background daemon initiates low-priority Q-LoRA gradient accumulation passes over the rollout buffer.

---

## 5. Non-Autoregressive Decision Engine (`crates/oxide-engines`)

### 5.1 Architecture

For sub-$25\text{ ms}$ operational decision latency, the platform provides a pure-Rust, strictly non-autoregressive decision engine:
- **Single Forward Pass:** Eliminates the token generation loop; outputs calibrated categorical decisions in a single inference step.
- **Fast-KAN Decision Head:** Kolmogorov-Arnold Network spline approximation replacing dense MLP layers, reducing parameter count while improving non-linear boundary separation.
- **Brier Score Calibration:** Calibrated decision confidence vectors asserting accurate uncertainty estimation under distribution shift.

---

## 6. Verification & Invariant Gates

```bash
cargo test -p oxide-kernels --test chunked_loss_memory_bound
cargo test -p oxide-engines --test tiered_kv_cache_1m_rollover
cargo test -p model-trainer --test grpo_multi_domain_reward
cargo test -p oxide-engines --test decision_engine_latency
```

* **Gate Invariants:**
  1. $1\text{M-token}$ KV-cache execution does not exceed $24\text{ GB}$ GPU VRAM allocation.
  2. Peak backprop VRAM consumption under chunked cross-entropy is reduced by $\ge 70\%$.
  3. Non-autoregressive decision latency remains $\le 25\text{ ms}$ at $p99$.
  4. GRPO policy advantage converges monotonically across synthetic test benchmarks.
