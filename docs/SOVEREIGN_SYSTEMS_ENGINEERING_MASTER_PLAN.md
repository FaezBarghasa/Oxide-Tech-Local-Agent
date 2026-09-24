# Oxide-Tech-Local-Agent: Comprehensive Autonomous Systems Engineering Master Plan

**Document Classification:** Mission-Critical Architecture & Systems Engineering Blueprint  
**Subsystems Covered:** All Workspace Crates (`crates/*`), Kernel Substrates, Hardware Accelerators, Runtime Fabrics, and Client Surfaces  
**Execution Paradigm:** Precondition-Driven Capability Horizons (Strictly Time-Invariant; No Temporal Schedules or Milestones)

---

## 1. Architectural Topology & State-Transition Model

The evolution of `Oxide-Tech-Local-Agent` represents a systematic transition from an experimental multi-crate prototype into a sovereign, self-improving hardware-software co-design ecosystem. Progression through capability states is governed exclusively by formal proof invariants, memory safety bounds, test execution gates, and algorithmic convergence metrics.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                 CAPABILITY STATE VI: SOVEREIGN HARDWARE SWARM               │
│  - Neural Machine Code Lifting (PTX/SASS/Firmware -> Verified Safe Rust)    │
│  - Cross-Domain Autonomous Hardware Synthesis (KiCad + OpenCASCADE + QEMU)  │
│  - Distributed Workstation Mesh Clustering via CRDT & Gossip Telemetry      │
└──────────────────────────────────────▲──────────────────────────────────────┘
                                       │ Gate: Autonomous Cross-Domain Synthesis Invariant
┌──────────────────────────────────────┴──────────────────────────────────────┐
│             CAPABILITY STATE V: RECURSIVE AUTONOMY & POLICY DISTILLATION    │
│  - Autonomous GRPO Reinforcement Learning over Formally Verified Traces     │
│  - Episodic Semantic Compaction & Unified SurrealDB Knowledge Graphs        │
│  - Dynamic Sandboxed WebAssembly Tool Synthesis & In-Memory Hot-Mounting    │
└──────────────────────────────────────▲──────────────────────────────────────┘
                                       │ Gate: Empirical Policy Convergence & Verification Bound
┌──────────────────────────────────────┴──────────────────────────────────────┐
│           CAPABILITY STATE IV: FORMAL HARDWARE EDA & CLOSED-LOOP PHYSICS    │
│  - Closed-Loop Automated KiCad ERC/DRC Correction Harness                   │
│  - Symbolic SMT-LIB2 Constraint Proving via Z3 & Kani Model Checking        │
│  - Coupled Finite-Element / Graph Neural Network PCB Thermal Solvers        │
└──────────────────────────────────────▲──────────────────────────────────────┘
                                       │ Gate: Zero-DRC Invariant & Physics Equivalence Pass
┌──────────────────────────────────────┴──────────────────────────────────────┐
│           CAPABILITY STATE III: HIGH-PERFORMANCE COGNITIVE INFERENCE        │
│  - Paged DDR5 System RAM KV-Cache Offload (1,000,000 Token Buffer)          │
│  - Custom Fused Kernels (RoPE, Chunked Cross-Entropy, SwiGLU Backprop)      │
│  - Grammar-Constrained Decoding (CFG / GBNF Guided Token Selection)         │
│  - Native Surgical Unified Diff Patcher with AST-Preserving Context Window  │
└──────────────────────────────────────▲──────────────────────────────────────┘
                                       │ Gate: High-Context Saturation & Zero Grammar Failure
┌──────────────────────────────────────┴──────────────────────────────────────┐
│            CAPABILITY STATE II: RUNTIME RESILIENCE & ZERO-COPY IPC          │
│  - Polymorphic Inference Engine Interface (`crates/oxide-engines`)          │
│  - POSIX `memfd_create` Shared Memory Interconnect with Parent Supervisor   │
│  - Tri-Tier Sandboxing Membrane (Tracepoint eBPF -> Landlock -> WasmEdge)   │
└──────────────────────────────────────▲──────────────────────────────────────┘
                                       │ Gate: Memory Alignment & Watchdog Crash-Recovery Invariant
┌──────────────────────────────────────┴──────────────────────────────────────┐
│           CAPABILITY STATE I: CRYPTOGRAPHIC SANITATION & HERMETIC TOOLCHAIN │
│  - Git Reflog Rewrite (Total Eradication of Committed `.pem` / `.key` Files)│
│  - In-Memory Ephemeral TLS 1.3 Bootstrapping (`rcgen` + `ring`)             │
│  - Hermetic FlatBuffers Compilation with Blake3 Checksum Cache Invalidation │
│  - Workspace Deduplication & Tree-Sitter Complete Grammar Compilation       │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Capability State I: Cryptographic Sanitation, Hermetic Closure & Substrate Hygiene

### 2.1 Cryptographic Remediation & In-Memory Ephemeral PKI

* **Objective:** Eliminate committed static keys across repository reflogs and replace all internal transport security with RAM-only ephemeral certificates.
* **Affected Subsystems:** Root, `crates/gateway/`, `crates/oxide-security/`, `scripts/`.

#### Detailed Implementation Tasks:
1. **Historical Secret Expungement:**
   * Execute immutable history rewriting utilizing `git-filter-repo` to permanently erase `key.pem`, `cert.pem`, `crates/gateway/key.pem`, `crates/gateway/cert.pem`, `scripts/key.pem`, and `scripts/cert.pem`.
   * Clear repository reflogs and force garbage collection:
     ```bash
     git filter-repo --invert-paths \
       --path key.pem --path cert.pem \
       --path crates/gateway/key.pem --path crates/gateway/cert.pem \
       --path scripts/key.pem --path scripts/cert.pem \
       --force
     git reflog expire --expire=now --all
     git gc --prune=now --aggressive
     ```
   * Enforce remote verification scan via TruffleHog and Gitleaks to assert zero remaining secret entropy.
2. **Runtime In-Memory PKI Engine (`crates/oxide-security/src/ephemeral_tls.rs`):**
   * Implement mutual TLS generation entirely within heap memory:
     ```rust
     use rcgen::{CertificateParams, DistinguishedName, KeyPair, PKCS_ECDSA_P256_SHA256};
     use ring::rand::SystemRandom;

     pub struct EphemeralTlsContext {
         pub server_config: rustls::ServerConfig,
         pub client_config: rustls::ClientConfig,
         pub fingerprint: [u8; 32],
     }

     impl EphemeralTlsContext {
         pub fn generate() -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
             let rng = SystemRandom::new();
             let key_pair = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256)?;
             let mut params = CertificateParams::default();
             let mut dn = DistinguishedName::new();
             dn.push(rcgen::DnType::CommonName, "oxide-mesh.internal");
             params.distinguished_name = dn;
             params.key_pair = Some(key_pair);

             let cert = params.self_signed(params.key_pair.as_ref().unwrap())?;
             let cert_der = cert.der().to_vec();
             let key_der = cert.key_pair().serialized_der().to_vec();

             let hash = ring::digest::digest(&ring::digest::SHA256, &cert_der);
             let mut fingerprint = [0u8; 32];
             fingerprint.copy_from_slice(hash.as_ref());

             // Construct rustls configs
             let server_config = configure_server_tls(&cert_der, &key_der)?;
             let client_config = configure_client_tls(&cert_der)?;

             Ok(Self { server_config, client_config, fingerprint })
         }
     }
     ```
   * Prohibit disk persistence of private keys under any circumstance.
3. **Pre-Commit Invariant Hooks:**
   * Configure Git hooks enforcing AST-level scans rejecting any commit with diffs matching:
     $$\text{Pattern} = \text{\texttt{-----BEGIN (RSA|EC|OPENSSH|PGP)? PRIVATE KEY-----}}$$

---

### 2.2 Hermetic FlatBuffers Compilation & Build Reproducibility

* **Objective:** Remove non-deterministic committed binaries (`bin/flatc`) and establish deterministic build-time schema binding generation.
* **Affected Subsystems:** `bin/`, `schemas/`, `crates/oxide-protocol/build.rs`.

#### Detailed Implementation Tasks:
1. **Purge Static Compiler:** Remove `bin/flatc` permanently from version control.
2. **Dynamic Checksum-Gated Compiler (`crates/oxide-protocol/build.rs`):**
   * Calculate the Blake3 hash of `schemas/pcb_layout.fbs`.
   * Compare against `schemas/pcb_layout.fbs.blake3`. If mismatched or if generated artifacts are missing:
     * Check `$PATH` for host-installed `flatc`.
     * If absent, invoke the `flatbuffers-build` crate or compile `flatbuffers` from source within `OUT_DIR`.
     * Regenerate `generated/pcb_layout_generated.rs` and update the recorded hash.
     * Enforce compiler flag `--gen-mutable` and `--strict-json` for deterministic deserialization.

---

### 2.3 Workspace Consolidation & Elimination of Prototype Clones

* **Objective:** Resolve duplicate workspace crates and consolidate parallel implementations.
* **Action Matrix:**
  * Delete `crates/engines/` $\rightarrow$ Consolidate into `crates/oxide-engines/`.
  * Delete `crates/gateway/` and `crates/gateway-router/` $\rightarrow$ Consolidate into `crates/oxide-gateway/`.
  * Delete `crates/memory/` $\rightarrow$ Consolidate into `crates/oxide-state/`.
  * Delete `crates/mcp-server/` $\rightarrow$ Consolidate into `crates/oxide-mcp/`.
  * Remove deleted folders from root `Cargo.toml` `[workspace.members]`.

---

### 2.4 Complete Tree-Sitter Multi-Language Registry

* **Objective:** Replace placeholder stubs in `crates/tree-sitter-service/src/languages.rs` with compiled parsers.
* **Implementation:**
  ```rust
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub enum LanguageId {
      Rust,
      Python,
      C,
      Cpp,
      SystemVerilog,
      OpenScad,
  }

  impl LanguageId {
      pub fn grammar(&self) -> tree_sitter::Language {
          match self {
              Self::Rust => tree_sitter_rust::language(),
              Self::Python => tree_sitter_python::language(),
              Self::C => tree_sitter_c::language(),
              Self::Cpp => tree_sitter_cpp::language(),
              Self::SystemVerilog => tree_sitter_verilog::language(),
              Self::OpenScad => tree_sitter_openscad::language(),
          }
      }
  }
  ```

---

### Verification Gate I (State Transition Invariants)
```bash
cargo audit
gitleaks detect --source . --verbose
cargo check --workspace --all-targets
cargo test -p oxide-security -p oxide-protocol -p tree-sitter-service
```
* **Exit Gate Assertion:** Zero private key hashes present in Git tree; `flatc` binary absent; all duplicate crates deleted; AST grammar parsing functional across all six target languages.

---

## 3. Capability State II: Runtime Resilience, Memory Invariants & Containment Membrane

### 3.1 Hardened Aligned Tensor Memory Mapping (`crates/oxide-engines/src/mmap_tensor.rs`)

* **Objective:** Prevent `SIGBUS` faults and undefined behavior from unaligned raw pointer casts on mapped GGUF/Safetensors.
* **Mathematical Invariant:**
  $$\text{Memory Address } A \equiv 0 \pmod{\text{align\_of}::<T>()} \quad \text{where } \text{align\_of}::<T>() = 64 \text{ (AVX-512)}$$

```rust
use std::fs::File;
use std::path::Path;
use fs2::FileExt;
use memmap2::Mmap;

pub struct HardenedTensorMap {
    file: File,
    mmap: Mmap,
    aligned_offset: usize,
    element_count: usize,
}

impl HardenedTensorMap {
    pub fn load_aligned<T: Copy>(path: &Path, offset: usize, count: usize) -> Result<Self, String> {
        let file = File::open(path).map_err(|e| e.to_string())?;
        // Acquire advisory shared read lock to prevent concurrent file truncation
        file.lock_shared().map_err(|e| e.to_string())?;

        let mmap = unsafe { Mmap::map(&file).map_err(|e| e.to_string())? };
        let align_req = std::mem::align_of::<T>();
        let raw_ptr = unsafe { mmap.as_ptr().add(offset) };

        if (raw_ptr as usize) % align_req != 0 {
            return Err(format!("Unaligned memory mapping: pointer {:p} not aligned to {}", raw_ptr, align_req));
        }

        let required_bytes = count * std::mem::size_of::<T>();
        if offset + required_bytes > mmap.len() {
            return Err("Memory map slice exceeds file byte bounds".into());
        }

        Ok(Self { file, mmap, aligned_offset: offset, element_count: count })
    }

    pub fn as_slice<T>(&self) -> &[T] {
        unsafe {
            let ptr = self.mmap.as_ptr().add(self.aligned_offset) as *const T;
            std::slice::from_raw_parts(ptr, self.element_count)
        }
    }
}
```

---

### 3.2 Polyglot Supervised IPC Substrate (`crates/scene-forge/src/ipc_bridge.rs`)

* **Objective:** Eliminate serialization bottlenecks and hung processes during Python/Blender/KiCad daemon invocations.
* **Mechanism:**
  1. **Low-Latency POSIX `memfd_create` Shared Memory:**
     Meshes and geometry netlists $> 2\text{ MB}$ bypass gRPC serialization. High-volume vertex arrays and netlists are written directly into an anonymous RAM-backed file descriptor created via `memfd_create` and transmitted as `SCM_RIGHTS` ancillary data over Unix Domain Sockets.
  2. **Supervised Watchdog Thread:**
     * Issue periodic `Heartbeat` pings every $500\text{ ms}$.
     * If three consecutive heartbeats fail ($1500\text{ ms}$ silence):
       1. Emit `SIGKILL` to the unresponsive child process.
       2. Unlink any shared memory segments via `shm_unlink`.
       3. Re-spawn daemon sidecar automatically without disrupting active host inference loops.

---

### 3.3 Dynamic Environmental Containment Membrane (`crates/ebpf-sentinel`)

* **Objective:** Enable multi-tier security sandboxing that degrades gracefully based on host execution privileges.
* **Capability Matrix:**
  * **Privileged Tier (Root / `CAP_BPF`):** Intercept sys-calls via eBPF tracepoints (`sys_enter_execve`, `sys_enter_connect`). Enforce hard network and file path allow-lists at the kernel boundary.
  * **Unprivileged Linux Tier (Standard User):** Enforce `Landlock` Linux Security Module (LSM) restricting filesystem access strictly to workspace paths, coupled with `Seccomp-BPF` restricting syscalls to standard compute primitives.
  * **Restricted / Fallback Tier (macOS / Containers):** Fuel-metered WebAssembly sandbox isolation via `WasmEdge` runtime.

---

### Verification Gate II (State Transition Invariants)
```bash
RUSTFLAGS="-Zsanitizer=address" cargo test -p oxide-engines --test mmap_alignment
cargo test -p scene-forge --test watchdog_hang_recovery
cargo test -p ebpf-sentinel --test membrane_capability_negotiation
```
* **Exit Gate Assertion:** ASan and Miri confirm zero unaligned reads on mapped tensors; child processes killed mid-operation cleanly respawn within $\le 2000\text{ ms}$; unprivileged users gracefully initialize under Landlock without panic.

---

## 4. Capability State III: High-Efficiency Cognitive Kernels, DDR5 Paged Cache & Surgical Agentics

### 4.1 Fused Kernel Computation & Chunked Cross-Entropy (`crates/oxide-kernels`)

* **Objective:** Maximize workstation parameter throughput and achieve competitive parity with high-performance architectures (Unsloth).
* **Chunked Cross-Entropy Formulation:**
  Standard training loops materialize the complete logit matrix $Z \in \mathbb{R}^{B \times S \times V}$ in VRAM, consuming:
  $$M_{\text{logits}} = B \cdot S \cdot V \cdot 4 \text{ bytes}$$
  For batch size $B=2$, sequence length $S=8,192$, and vocabulary $V=152,064$, $M_{\text{logits}} \approx 9.96\text{ GB}$, causing immediate Out-of-Memory (OOM) errors during backward passes on consumer GPUs.
  
  **Chunked Kernel Implementation:** Compute cross-entropy iteratively over vocabulary slices $V_k$ of size $C = 4,096$:
  $$\mathcal{L} = -\sum_{i=1}^S \left[ z_{i, y_i} - \ln \left( \sum_{k=1}^{\lceil V/C \rceil} \sum_{j \in V_k} e^{z_{i, j} - m_i} \right) - m_i \right], \quad m_i = \max_j (z_{i, j})$$
  * Intermediate logit tensors never exceed $B \cdot S \cdot C \cdot 4\text{ bytes} \approx 268\text{ MB}$.
  * Write raw CUDA/Triton fused kernels for SwiGLU forward/backward:
    $$\text{SwiGLU}(x, W, V) = (xW \cdot \sigma(xW)) \odot xV$$
    eliminating redundant memory transfers between GPU global memory and compute registers.

---

### 4.2 Paged DDR5 System RAM KV-Cache Offloading (`crates/oxide-engines/src/tiered_kv_cache.rs`)

* **Objective:** Expand effective context windows to $1,000,000$ tokens on single workstation GPUs by tiering memory across GPU VRAM and high-speed DDR5 host memory.
* **Mathematical Allocation Model:**
  $$M_{\text{KV}}(S) = 2 \cdot L \cdot N_{\text{kv}} \cdot d_{\text{head}} \cdot S \cdot B_{\text{elem}}$$
  For a 35B model ($L = 40, N_{\text{kv}} = 8, d_{\text{head}} = 128$) with 8-bit quantized cache ($B_{\text{elem}} = 1\text{ byte}$):
  * At $S = 1,000,000$: $M_{\text{KV}} \approx 81.92\text{ GB}$.
* **Tiered Cache Structure:**
  ```
  [Token Indices 0 .. 32]       ──► Tier 0: GPU VRAM (Attention Sinks, pinned)
  [Token Indices 32 .. S-4096]  ──► Tier 1: Host DDR5 RAM (Paged via DMA Prefetch)
  [Token Indices S-4096 .. S]   ──► Tier 0: GPU VRAM (Sliding Working Window)
  ```
* **Asynchronous Prefetch Pipeline:**
  Double-buffered DMA host-to-device transfers (`cudaMemcpyAsync`) overlap PCIe bus traversal with current GEMM matrix multiplications, masking data transfer latencies completely.

---

### 4.3 Native AST-Aware Unified Diff Patching Engine (`crates/oxide-core/src/diff_patcher.rs`)

* **Objective:** Match Claude Code file modification performance by executing byte-minimal unidiff patches instead of full-file rewrites.
* **Implementation Algorithm:**
  1. Parse original target file into indexed line-hash trees using Tree-Sitter.
  2. Parse incoming patch hunks:
     ```
     @@ -14,6 +14,8 @@
     ```
  3. Locate optimal insertion context utilizing fuzzy Levenshtein line-distance matching if absolute line numbers have drifted.
  4. Assert hunk preconditions (context lines must match target file byte-for-byte).
  5. Apply minimal mutations directly into the file buffer.
  6. Re-parse modified buffer via Tree-Sitter: if syntax tree contains `ERROR` nodes, immediately reject patch, revert buffer, and propagate AST syntax error back into agent feedback channel.

---

### 4.4 Grammar-Constrained Context-Free Decoding (`crates/oxide-engines/src/grammar.rs`)

* **Objective:** Eliminate tool-calling JSON parse failures by compiling tool schemas into Context-Free Grammars (GBNF / CFG).
* **Mechanism:**
  * Convert tool JSON Schema directly into a terminal state machine:
    ```
    root        ::= ToolCall
    ToolCall    ::= "<tool_call>" "{" ws "\"name\":" ws String "," ws "\"arguments\":" ws Object "}" "</tool_call>"
    ```
  * During token generation in `oxide-engines`, mask out all vocabulary logits whose emission would violate the grammar state transition matrix. Zero-shot 100% compliant JSON emissions guaranteed.

---

### Verification Gate III (State Transition Invariants)
```bash
cargo test -p oxide-kernels --test chunked_loss_memory_bound
cargo test -p oxide-engines --test tiered_kv_cache_1m_rollover
cargo test -p oxide-core --test unified_diff_ast_reversion
cargo test -p oxide-engines --test gbnf_json_compliance
```
* **Exit Gate Assertion:** Peak training VRAM allocation reduced by $> 70\%$; KV cache spans $1,000,000$ tokens in DDR5 host RAM without GPU OOM; 10,000 randomized tool calls achieve $100.00\%$ JSON validation pass rate.

---

## 5. Capability State IV: Electronic Design Automation, Multi-Physics & Symbolic Verification

### 5.1 Closed-Loop Circuit Engineering (`crates/circuit-forge`)

* **Objective:** Eliminate hallucinated schematics by connecting netlist synthesis directly to KiCad's command-line Electronic Rules Checker (ERC) and Design Rules Checker (DRC).
* **Feedback Loop Topology:**
  ```
  [LLM Synthesizes Netlist] ──► [kicad_serializer.rs] ──► [.kicad_sch / .kicad_pcb]
                                                                  │
                                                                  ▼
  [Feedback Iteration Loop] ◄── [Error Parse AST]    ◄── [KiCad CLI Harness (ERC/DRC)]
  (Max 5 Attempts)                (Violations Found)             │
                                                                 ▼ (Zero Errors)
                                                        [SPICE Simulation Pass]
  ```
* **Hardware Physical Constraints Enforced:**
  1. **Current Carrying Capacity (IPC-2221):**
     Calculate trace width $W$ as a function of current $I$, permissible temperature rise $\Delta T$, and copper thickness $t$:
     $$I = k \cdot \Delta T^{\beta} \cdot A^{\gamma} \quad \text{where } A = W \cdot t$$
     Reject any trace where synthesized width is insufficient for rated component dissipation.
  2. **Contention Prevention:** Assert zero floating CMOS input pins; assert single-driver invariants on all shared digital buses ($I^2C$, SPI, UART).

---

### 5.2 Coupled GNN-FEM PCB Thermal Solver (`python-bridge/training/`)

* **Objective:** Replace placeholder scripts (`train_thermal_model.py`, `train_gnn.py`) with a physical thermal solver.
* **Implementation:**
  * Construct a Graph Neural Network utilizing PyTorch Geometric:
    * Nodes: Components, pads, and trace junctions with feature vectors:
      $$v_i = \left[ P_{\text{diss}}, \kappa_{\text{cu}}, \text{Area}_i, x_i, y_i, \text{Layer}_i \right]$$
    * Edges: Physical copper traces and FR4 thermal conductance paths.
  * Train network on finite-element transient thermal simulations (calculating thermal gradients across inner and outer copper planes).
  * Automatically identify hotspot thermal traps where $T_{\text{junction}} > 85^\circ\text{C}$ and inject routing keep-out zones back to the agent.

---

### 5.3 Symbolic Invariant Prover & Bounded Model Checking (`crates/formal-verify`)

* **Objective:** Mathematically prove hardware and firmware invariants using SMT solvers (Z3) and bounded model checkers (Kani).
* **SMT-LIB2 Hardware Prover (`crates/formal-verify/src/smt_solver.rs`):**
  Translate circuit finite state machines and power sequencing into formal logical formulas:
  $$\Phi_{\text{safe}} = \bigwedge_{s \in \text{States}} \left( V(s) \le V_{\text{max}} \land I(s) \le I_{\text{max}} \land (\text{Fault}(s) \implies \text{Isolated}(s)) \right)$$
* **Kani Rust Verification Harnesses:**
  Attach bounded model verification harnesses to all lock-free channels (`channel.rs`), raw slice casts (`mmap_tensor.rs`), and ring-buffer pointers to verify absence of integer overflows, null-pointer dereferences, and panics across all inputs.

---

### Verification Gate IV (State Transition Invariants)
```bash
cargo test -p circuit-forge --test kicad_drc_closed_loop
python3 python-bridge/training/train_thermal_model.py --verify-physics
cargo kani -p formal-verify
cargo kani -p oxide-core --harness verify_ring_buffer
```
* **Exit Gate Assertion:** Netlist emission achieves zero DRC violations; thermal solver converges with $< 5\%$ divergence from finite-element benchmarks; Kani proves zero panic paths in core FFI and concurrent primitives.

---

## 6. Capability State V: Sovereign Mobile Companion, P2P Mesh & Decentralized Telemetry

### 6.1 Tailscale-Free Sovereign Mobile Bridge (`crates/oxide-gateway/src/mobile_bridge.rs`)

* **Objective:** Monitor, steer, and provide Human-in-the-Loop (HITL) approval gates from any smartphone without third-party mesh VPNs (Tailscale), cloud proxies, or SaaS lock-in.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                             DESKTOP HOST RUNTIME                            │
│  [oxide-core] ◄──► [optio (HITL Gates)] ◄──► [oxide-gateway (WebSockets)]  │
│                                                     │                       │
│                                        ┌────────────┴─────────────┐         │
│                                        ▼                          ▼         │
│                                  [mDNS / LAN]              [WebRTC Server]  │
│                                  (Port 8080)             (DataChannel / ICE)│
└───────────────────────────────────────┬───────────────────────────┬─────────┘
                                        │                           │
                                  LAN Wi-Fi                 Public Cellular
                           (Zero Setup, Direct TLS)        (NAT Hole-Punching)
                                        │                           │
                                        ▼                           ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        MOBILE DEVICE (STANDALONE PWA)                       │
│  - Secure Enclave WebAuthn Passkeys (TouchID / FaceID Hardware Attestation) │
│  - Real-Time Token & Reasoning Telemetry (Sub-100ms Latency)                │
│  - Unified Diff Approval Deck: [Reject] / [Sign & Approve] / [Steer Prompt] │
└─────────────────────────────────────────────────────────────────────────────┘
```

#### Detailed Protocol Specifications:
1. **Physical Optical QR Handshake:**
   * Desktop runtime creates ephemeral $X25519$ keypair $(sk_{\text{host}}, pk_{\text{host}})$ and cryptographic nonce $N_{\text{auth}} \in \{0, 1\}^{256}$.
   * Renders pairing URI as QR code on Tauri interface:
     $$\text{URI} = \text{oxide://pair}?pk=pk_{\text{host}}\&nonce=N_{\text{auth}}\&lan=\text{192.168.1.50:8080}\&ice=\text{stun:stun.cloudflare.com}$$
   * Mobile client scans QR code, derives shared secret:
     $$S = \text{X25519}(sk_{\text{mobile}}, pk_{\text{host}})$$
     and derives session key $K_{\text{session}} = \text{HKDF-Extract}(S, N_{\text{auth}})$.
2. **WebRTC DataChannel Peer Traversal:**
   * Negotiate STUN NAT hole-punching (`stun.cloudflare.com:3478` or `stun.l.google.com:19302`).
   * Open encrypted, ordered DataChannel `oxide-control` using DTLS 1.3 with ChaCha20-Poly1305.
3. **Hardware Biometric Interventions:**
   * Prior to approving high-risk agent interventions (unified diff patches, KiCad file writes, terminal shell execution), the mobile PWA triggers local hardware biometrics via the WebAuthn API:
     ```javascript
     const assertion = await navigator.credentials.get({
       publicKey: { challenge: randomNonce, userVerification: "required" }
     });
     ```
   * Approval actions are cryptographically signed by the Secure Enclave and transmitted over the WebRTC DataChannel to release the desktop execution lock.

---

### 6.2 Workstation Swarm Clustering & Federated Consensus (`crates/edge-swarm`)

* **Objective:** Enable heterogeneous workstations (e.g., Apple Silicon M-series + NVIDIA RTX x86_64) to form a coordinated cluster without a centralized master server.
* **Mechanism:**
  * **Peer Discovery:** Zero-configuration local discovery over mDNS / DNS-SD.
  * **Capability-Based Task Routing:**
    Specialist agent handoffs automatically route subtasks to the node with optimal physical hardware:
    * High-VRAM node receives KV-cache indexing and dense embeddings.
    * High-TFLOPS NVIDIA GPU node executes fused Triton kernels and SPICE simulations.
    * High-core-count ARM/Apple Silicon node executes parallel Kani bounded model checks.
  * **Conflict-Free Replicated Data Types (CRDT):**
    Shared project state, netlists, and memory graphs synchronize using state-based PN-Counters and Observed-Remove Sets (OR-Sets) to eliminate merge conflicts during concurrent multi-agent collaboration.

---

### Verification Gate V (State Transition Invariants)
```bash
cargo test -p oxide-gateway --test mobile_p2p_datachannel_handshake
cargo test -p edge-swarm --test crdt_netlist_synchronization
tests/e2e/run_mobile_biometric_signing_flow.sh
```
* **Exit Gate Assertion:** WebRTC DataChannel traverses symmetric cellular NAT without intermediate proxies; biometric hardware signature unlocks blocked desktop agent gate; CRDT state synchronizes deterministically across three heterogeneous workstation nodes.

---

## 7. Capability State VI: Autonomous Recursive Evolution & Epistemic Synthesis

### 7.1 Continuous Group Relative Policy Optimization (GRPO) Loop (`crates/model-trainer/src/self_distillation.rs`)

* **Objective:** Close the learning loop so the local agent autonomously fine-tunes its own weights from successful engineering trajectories during idle GPU cycles.
* **GRPO Objective Formulation:**
  $$\mathcal{J}_{\text{GRPO}}(\theta) = \mathbb{E} \left[ \frac{1}{G} \sum_{i=1}^G \left( \min\left( \frac{\pi_\theta(o_i\vert{}q)}{\pi_{\text{ref}}(o_i\vert{}q)} \hat{A}_i, \; \text{clip}\left(\frac{\pi_\theta(o_i\vert{}q)}{\pi_{\text{ref}}(o_i\vert{}q)}, 1-\epsilon, 1+\epsilon\right) \hat{A}_i \right) - \beta D_{\text{KL}}(\pi_\theta \parallel \pi_{\text{ref}}) \right) \right]$$
* **Rigorous Multi-Domain Composite Reward:**
  $$\hat{A}_i = \text{Normalize}\left( \text{Reward}(o_i) \right)$$
  $$\text{Reward}(o_i) = w_1 R_{\text{compile}} + w_2 R_{\text{formal\_verify}} + w_3 R_{\text{drc\_pass}} + w_4 R_{\text{sim\_continuity}} + w_5 R_{\text{thermal\_margin}}$$
  * $R_{\text{compile}} \in \{0, 1\}$: Binary compilation pass (`rustc` / `cargo check`).
  * $R_{\text{formal\_verify}} \in [0, 1]$: Ratio of symbolic assertions proved by Z3.
  * $R_{\text{drc\_pass}} \in \{0, 1\}$: Zero-error output from KiCad DRC engine.
  * $R_{\text{sim\_continuity}} \in [0, 1]$: Circuit electrical stability under SPICE transient solver.
  * $R_{\text{thermal\_margin}} \in [0, 1]$: Maximum board surface temperature $\le 70^\circ\text{C}$.
* **Rollout Buffer Governance:**
  Only trajectories achieving $\text{Reward}(o_i) \ge 0.95$ are committed to `crates/data/grpo_rollouts.jsonl`. When the system detects idle GPU and user inactivity, background Q-LoRA gradient accumulation passes update adapter weights.

---

### 7.2 Dynamic Micro-Tool Synthesis (`crates/self-evolver` + `crates/wasm-forge`)

* **Objective:** Enable the agent to create its own tools when encountering repetitive, computationally expensive tasks.
* **Synthesis Lifecycle:**
  ```
  [Agent Detects Computational Bottleneck]
                     │
                     ▼
  [Emits High-Performance Rust Micro-Tool]
                     │
                     ▼
  [Compiles to WebAssembly via wasm_emitter.rs]
                     │
                     ▼
  [Validates Fuel Limits & Landlock Constraints]
                     │
                     ▼ (Verification Passed)
  [Hot-Mounts into Runtime Tool Registry without Daemon Reboot]
  ```

---

### 7.3 Neural Machine Code Lifting & Binary Transmutation (`crates/re-forge`)

* **Objective:** Decompile closed-source binary drivers, proprietary firmware, and compiled GPU machine code (PTX/SASS) into safe, verified, idiomatic Rust.
* **Lifting Pipeline:**
  1. **Disassembly & CFG Recovery:**
     Parse raw ELF binaries and CUDA PTX into a structured Control Flow Graph (CFG) using `crates/re-forge/src/cfg.rs`.
  2. **High-Level Intermediate Representation (HIR):**
     Abstract architecture-specific register allocations, stack frames, and memory pointers into a typed SSA (Static Single Assignment) intermediate representation.
  3. **Neural Code Reconstruction:**
     Reconstruct safe Rust implementations that replace unsafe pointer arithmetic with safe abstractions (`Option`, `Result`, typed slices).
  4. **Equivalence Proving:**
     Verify functional input/output equivalence between lifted Rust code and original binary assembly across synthetic test vectors.

---

### Verification Gate VI (Ultimate Architectural Invariants)
```bash
cargo test -p self-evolver --test grpo_reward_convergence
cargo test -p wasm-forge --test dynamic_tool_mounting
cargo test -p re-forge --test ptx_to_rust_translation_equivalence
```
* **Exit Gate Assertion:** Policy advantage converges positively over 100 consecutive rollout batches; synthesized WebAssembly tools execute within strict memory and fuel bounds; lifted PTX algorithms reproduce mathematical outputs with bit-level floating-point precision ($100\%$ numerical parity).

---

## 8. Summary of Completed Architectural Artifacts

The engineering blueprints detailing each capability layer are formalized in the workspace documentation suite:

1. **`docs/SOVEREIGN_SYSTEMS_ENGINEERING_MASTER_PLAN.md`:** This master blueprint defining the state-transition model and capability epochs.
2. **`docs/SOVEREIGN_MOBILE_COMPANION_SPEC.md`:** The complete WebRTC P2P DataChannel, optical pairing, and biometric Passkey monitoring specification.
3. **`docs/DDR5_TIERED_KV_AND_SELF_TRAINING_SPEC.md`:** The technical implementation for 1M-token tiered DDR5 KV caching, context compaction, and self-distillation.
4. **`docs/QA_AUDIT_V2_AND_COMPETITIVE_ROADMAP.md`:** The Senior QA audit, defect register, and competitive technical analysis against Claude Code, Unsloth, and Nous Hermes.
5. **`docs/CTO_TECHNICAL_IMPLEMENTATION_PLAN.md`:** Low-level implementation specifications, FFI boundaries, and crate consolidation mappings.
