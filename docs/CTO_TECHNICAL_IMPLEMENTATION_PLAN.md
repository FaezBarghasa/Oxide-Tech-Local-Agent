# Oxide-Tech-Local-Agent: Technical Implementation & Engineering Execution Plan

**Document Version:** 2.0.0-PROD

**Author:** Chief Technology Officer / Principal Systems Architect

**Classification:** Engineering Specification & Execution Blueprint

**Gating Strategy:** Precondition-Driven Architectural Horizons (Zero Temporal Dependencies)

## 1. System Topology & Architectural Target State

The target architecture consolidates fragmented prototype crates into an orthogonal, five-tier dependency Directed Acyclic Graph (DAG). Circular dependencies, ambient file access, unauthenticated internal sockets, and duplicated inference abstractions are strictly eliminated.

```
┌────────────────────────────────────────────────────────────────────────┐
│                        TIER 4: SURFACE & USER INTERFACE               │
│         [tauri-desktop] ──(IPC)──► [surface-api]                       │
└────────────────────────────────────┬───────────────────────────────────┘
                                     │
┌────────────────────────────────────▼───────────────────────────────────┐
│                        TIER 3: ORCHESTRATION & GATEWAY                 │
│         [oxide-gateway] ◄──► [router] ◄──► [optio (DAG/Critic)]        │
│         [scheduler]     ◄──► [agent-journal]                           │
└────────────────────────────────────┬───────────────────────────────────┘
                                     │
┌────────────────────────────────────▼───────────────────────────────────┐
│                        TIER 2: COGNITION & DOMAIN FORGES               │
│  ┌───────────────────────┐ ┌────────────────────┐ ┌──────────────────┐ │
│  │     oxide-engines     │ │   circuit-forge    │ │    cad-forge     │ │
│  │(Candle/vLLM/LLaMA/MoE)│ │ (DRC/ERC/Netlists) │ │  (B-Rep/Voxels)  │ │
│  └───────────────────────┘ └────────────────────┘ └──────────────────┘ │
│  ┌───────────────────────┐ ┌────────────────────┐ ┌──────────────────┐ │
│  │       re-forge        │ │   formal-verify    │ │   self-evolver   │ │
│  │ (PTX/SASS/Decompiler) │ │   (SMT-LIB2/Kani)  │ │ (GRPO/Q-LoRA/Wasm│ │
│  └───────────────────────┘ └────────────────────┘ └──────────────────┘ │
└────────────────────────────────────┬───────────────────────────────────┘
                                     │
┌────────────────────────────────────▼───────────────────────────────────┐
│                        TIER 1: PERSISTENCE & SYSTEM FABRIC             │
│   [oxide-state] (SurrealDB/Working) ◄──► [qdrant-service] (Embeddings) │
│   [oxide-protocol] (Schemas/FlatBuffers)                               │
└────────────────────────────────────┬───────────────────────────────────┘
                                     │
┌────────────────────────────────────▼───────────────────────────────────┐
│                        TIER 0: HARDWARE KERNEL & MEMBRANE              │
│   [oxide-security] (mTLS/TPM) ◄──► [ebpf-sentinel] (Containment)       │
│   [oxide-kernels] (AVX-512/SIMD/RoPE)                                  │
└────────────────────────────────────────────────────────────────────────┘

```

## 2. Phase I: Cryptographic Sanitation, Hardware Identity & Zero-Trust Protocol

### 2.1 Git History Secret Expungement

Execute an immutable rewrite of the source control history to scrub all checked-in cryptographic material, certificate files, and development keys.

1. **Scrub Protocol:**

   ```
   git filter-repo --invert-paths \
     --path key.pem \
     --path cert.pem \
     --path crates/gateway/key.pem \
     --path crates/gateway/cert.pem \
     --path scripts/key.pem \
     --path scripts/cert.pem \
     --path bin/flatc \
     --force
   
   ```

2. **Post-Scrub Verification:**
   Execute AST and binary pattern scans across all reflog objects:

   ```
   trufflehog git file://. --since-commit HEAD --fail
   gitleaks detect --source . --verbose --no-git=false
   
   ```

3. **Cryptographic Revocation:**
   Emit a cryptographically signed revocation advisory across the development fleet. Mark all exposed public key fingerprints as permanently untrusted in the peer discovery registry.

### 2.2 In-Memory Ephemeral mTLS Engine (`crates/oxide-security`)

Eliminate static certificate files. All internal HTTP/3, WebSockets, and gRPC bridges must negotiate mutual TLS with ephemeral, memory-only certificates bound to local machine entropy.

```
use rcgen::{CertificateParams, KeyPair, PKCS_ECDSA_P256_SHA256, DistinguishedName};
use ring::rand::SystemRandom;

pub struct EphemeralTlsContext {
    pub client_config: rustls::ClientConfig,
    pub server_config: rustls::ServerConfig,
    pub node_fingerprint: [u8; 32],
}

impl EphemeralTlsContext {
    pub fn bootstrap() -> Result<Self, SecurityError> {
        let rng = SystemRandom::new();
        let key_pair = KeyPair::generate_for(&PKCS_ECDSA_P256_SHA256)?;
        
        let mut params = CertificateParams::default();
        let mut dn = DistinguishedName::new();
        dn.push(rcgen::DnType::CommonName, "oxide-internal-mesh.local");
        params.distinguished_name = dn;
        params.key_pair = Some(key_pair);

        let cert = params.self_signed(&params.key_pair.as_ref().unwrap())?;
        let cert_der = cert.der().to_vec();

        // Calculate node fingerprint (SHA-256)
        let fingerprint = ring::digest::digest(&ring::digest::SHA256, &cert_der);
        let mut node_fingerprint = [0u8; 32];
        node_fingerprint.copy_from_slice(fingerprint.as_ref());

        // Construct strict mutual TLS configs with zero disk leakage
        let server_config = configure_server_tls(&cert_der, &params)?;
        let client_config = configure_client_tls(&cert_der, &params)?;

        Ok(Self {
            client_config,
            server_config,
            node_fingerprint,
        })
    }
}

```

### 2.3 Pre-Commit Invariant Automation

Install non-bypassable pre-commit hooks enforcing:

* **Gitleaks rule engine** preventing check-in of regex `-----BEGIN (RSA|EC|OPENSSH|PGP)? PRIVATE KEY-----`.

* **Binary file check** rejecting ELF, Mach-O, PE, or `.pyc` additions outside explicit target test fixtures.

* **Workspace dependency lock validation** ensuring `Cargo.lock` and `bun.lock` checksum synchronicity.

## 3. Phase II: Workspace Consolidation & Unified Engine Subsystem

### 3.1 Crate Fusion & Deduplication

To resolve runtime schisms and conflicting maintenance loops, execute the following crate structural migrations:

| Legacy Fragmented Crates | Target Unified Crate | Architectural Role | 
 | ----- | ----- | ----- | 
| `crates/engines`  `crates/vllm-client` | `crates/oxide-engines` | Polymorphic inference dispatch (Candle, LLaMA.cpp, SGLang, vLLM). | 
| `crates/gateway`  `crates/gateway-router` | `crates/oxide-gateway` | Edge ingress, HTTP/3, gRPC termination, rate limiting, and RBAC. | 
| `crates/memory`  `crates/oxide-state` | `crates/oxide-state` | Tiered memory: Working (RAM), Episodic (SurrealDB), Semantic (Qdrant). | 
| `crates/mcp-server`  `crates/oxide-mcp` | `crates/oxide-mcp` | Protocol-compliant Model Context Protocol server. | 

### 3.2 Polymorphic Inference Engine Interface (`crates/oxide-engines`)

Replace divergent execution code with a single, hardened trait supporting streaming backpressure, hardware fallback, and speculative decoding:

```
use async_trait::async_trait;
use tokio::sync::mpsc;

#[derive(Debug, Clone)]
pub struct InferenceRequest {
    pub prompt: String,
    pub max_tokens: usize,
    pub temperature: f32,
    pub top_p: f32,
    pub stop_sequences: Vec<String>,
    pub lora_adapter: Option<String>,
}

#[derive(Debug)]
pub enum InferenceChunk {
    Token(String),
    Usage { prompt_tokens: usize, completion_tokens: usize },
    Done,
}

#[async_trait]
pub trait InferenceProvider: Send + Sync {
    async fn infer_stream(
        &self,
        request: InferenceRequest,
        tx: mpsc::Sender<Result<InferenceChunk, EngineError>>,
    ) -> Result<(), EngineError>;

    fn capabilities(&self) -> EngineCapabilities;
    async fn load_lora(&self, adapter_path: &Path) -> Result<(), EngineError>;
    async fn unload_lora(&self, adapter_name: &str) -> Result<(), EngineError>;
}

```

### 3.3 Hermetic FlatBuffer Generation

1. Permanently remove the committed executable `bin/flatc`.

2. Construct a hermetic build script in `crates/oxide-protocol/build.rs`:

   * Detect if `flatc` is available in `$PATH`.

   * If missing, invoke the `flatc-build` crate or compile `flatbuffers/flatc` via `cc::Build` in `OUT_DIR`.

   * Enforce declarative recompilation of `schemas/pcb_layout.fbs` into `generated/pcb_layout_generated.rs` strictly upon schema checksum divergence.

## 4. Phase III: Resilient IPC, Zero-Copy Fabric & Memory Alignment

### 4.1 Safe Aligned Tensor Memory Mapping (`crates/oxide-engines/src/mmap_tensor.rs`)

Direct casts of memory-mapped file buffers to floating-point slices without offset validation introduce Undefined Behavior (UB) and `SIGBUS` faults. Implement strict alignment assertion and advisory reader file-locking:

```
use std::fs::File;
use std::os::unix::fs::FileExt;
use memmap2::MmapOptions;
use fs2::FileExt as Fs2FileExt;

pub struct AlignedTensorMap {
    _file: File,
    mmap: memmap2::Mmap,
    aligned_offset: usize,
    element_count: usize,
}

impl AlignedTensorMap {
    pub fn load_f32(path: &Path, offset: usize, count: usize) -> Result<Self, EngineError> {
        let file = File::open(path)?;
        
        // Enforce shared advisory read lock to prevent concurrent truncation
        file.lock_shared()?;

        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let align_req = std::mem::align_of::<f32>();

        // Ensure memory mapping meets SIMD vector bounds (AVX-512 = 64-byte alignment preferred)
        let ptr = unsafe { mmap.as_ptr().add(offset) };
        let misalign = (ptr as usize) % align_req;
        if misalign != 0 {
            return Err(EngineError::AlignmentFault {
                offset,
                alignment: align_req,
                misalignment: misalign,
            });
        }

        // Validate byte boundary bounds
        let required_bytes = count * std::mem::size_of::<f32>();
        if offset + required_bytes > mmap.len() {
            return Err(EngineError::OutOfBounds {
                requested: offset + required_bytes,
                available: mmap.len(),
            });
        }

        Ok(Self {
            _file: file,
            mmap,
            aligned_offset: offset,
            element_count: count,
        })
    }

    pub fn as_slice(&self) -> &[f32] {
        unsafe {
            let ptr = self.mmap.as_ptr().add(self.aligned_offset) as *const f32;
            std::slice::from_raw_parts(ptr, self.element_count)
        }
    }
}

```

### 4.2 Supervised Python Bridge & Zero-Copy POSIX Shared Memory

1. **Heartbeat Supervisor:**
   Add a bidirectional, non-blocking heartbeat to `crates/api/proto/bridge.proto`:

   ```
   service BridgeService {
     rpc Heartbeat (Ping) returns (Pong);
     rpc ProcessGeometry (GeometryRequest) returns (GeometryResponse);
   }
   message Ping { uint64 timestamp_epoch_ms = 1; }
   message Pong { uint64 timestamp_epoch_ms = 1; uint32 status_flags = 2; }
   
   ```

2. **Subprocess Watchdog Loop:**
   In `crates/scene-forge/src/ipc_bridge.rs`, implement a dedicated supervisor thread:

   * Poll `Heartbeat` on a 500ms interval.

   * If three consecutive heartbeats fail, immediately flag the child process as non-responsive, terminate via `SIGKILL`, reclaim POSIX shared memory segments (`shm_unlink`), and respawn the Python daemon without interrupting host system inference.

3. **Zero-Copy Shared Memory for Large Geometries:**
   Large mesh vertex buffers and KiCad netlist matrices greater than $2\text{ MB}$ must not be serialized over gRPC Protobuf bytes. Pass file descriptors backed by `memfd_create` or named shared memory (`/dev/shm/oxide_mesh_*`) to eliminate serialization latency.

## 5. Phase IV: Domain Synthesis & Formal Verification Pipeline

### 5.1 Closed-Loop Circuit Engineering (`crates/circuit-forge`)

Eliminate heuristic netlist generation by coupling netlist emission directly to an automated Electronic Rules Checker (ERC) and Design Rules Checker (DRC) loop.

```
┌──────────────────┐      Emit      ┌─────────────────────┐
│ Agent Netlist    ├───────────────►│ kicad_serializer.rs │
│ Synthesizer      │                └──────────┬──────────┘
└────────▲─────────┘                           │ Generates
         │                                     ▼
         │ Feedback Iteration       ┌─────────────────────┐
         │ (AST Error Propagation)  │ KiCad CLI Validation│
         │                          │ (ERC / DRC Harness) │
         └──────────────────────────┤                     │
                  Errors Detected   └──────────┬──────────┘
                                               │ Zero DRC Errors
                                               ▼
                                    ┌─────────────────────┐
                                    │ Formal SPICE Engine │
                                    │ Transient Simulation│
                                    └─────────────────────┘

```

1. **DRC / ERC Rule Implementation:**

   * Validate trace width constraints against copper weight and calculated current dissipation:
     

     $$
     I = k \cdot \Delta T^{\beta} \cdot A^{\gamma}
     $$

   * Enforce automated pull-up / pull-down assertion on all floating CMOS inputs.

   * Assert single-driver invariants across shared communication buses ($I^2C$, SPI, UART).

2. **Concrete Thermal Model Replacement:**
   Replace the placeholder `python-bridge/training/train_thermal_model.py` with a Graph Neural Network (GNN) thermal solver based on PyTorch Geometric, coupled directly to the component power dissipation array emitted by `circuit-forge`.

### 5.2 Formal Methods & Symbolic Constraints (`crates/formal-verify`)

Replace ad-hoc test validation with symbolic theorem proving using Z3:

1. **Translation Pipeline:**
   Translate circuit state machines and digital logic models into SMT-LIB2 problems solved via `z3::Solver`:
   

   $$
   \Phi_{\text{safe}} = \bigwedge_{s \in \text{States}} \left( V(s) \le V_{\text{max}} \land I(s) \le I_{\text{max}} \land (\text{Fault}(s) \implies \text{Isolated}(s)) \right)
   $$

2. **Kani Bounded Model Checking:**
   Attach verification harnesses to all `unsafe` pointer operations, lock-free ring buffers in `crates/oxide-core/src/channel.rs`, and parser token loops in `crates/tree-sitter-service/src/parser.rs`.

## 6. Phase V: Autonomous Evolution, RL & Layered Containment

### 6.1 Reinforcement Learning Pipeline (GRPO Implementation)

Transform `crates/model-trainer/src/rl_engine.rs` into a functional Group Relative Policy Optimization loop:

1. **Objective Function:**
   Optimize policy parameter $\theta$ against reference model $\pi_{\text{ref}}$ over output group size $G$:
   

   $$
   \mathcal{J}_{\text{GRPO}}(\theta) = \mathbb{E} \left[ \frac{1}{G} \sum_{i=1}^G \left( \min\left( \frac{\pi_\theta(o_i\vert{}q)}{\pi_{\text{ref}}(o_i\vert{}q)} \hat{A}_i, \; \text{clip}\left(\frac{\pi_\theta(o_i\vert{}q)}{\pi_{\text{ref}}(o_i\vert{}q)}, 1-\epsilon, 1+\epsilon\right) \hat{A}_i \right) - \beta D_{\text{KL}}(\pi_\theta \parallel \pi_{\text{ref}}) \right) \right]
   $$

2. **Multi-Domain Objective Reward Formulation:**
   The advantage $\hat{A}_i$ is computed from normalized rewards:
   

   $$
   \text{Reward}(o_i) = w_1 R_{\text{compile}} + w_2 R_{\text{formal\_verify}} + w_3 R_{\text{drc\_pass}} + w_4 R_{\text{sim\_continuity}}
   $$

   * $R_{\text{compile}} \in \{0, 1\}$: Binary compilation success from `cargo check` / `rustc`.

   * $R_{\text{formal\_verify}} \in [0, 1]$: Percentage of symbolic assertions proven by Z3.

   * $R_{\text{drc\_pass}} \in \{0, 1\}$: DRC clean verification from KiCad engine.

   * $R_{\text{sim\_continuity}} \in [0, 1]$: Functional equivalence under SPICE transient analysis.

### 6.2 Adaptive Security Membrane (`crates/ebpf-sentinel`)

Resolve privilege assumption failures by implementing a capability negotiation matrix:

```
pub enum IsolationTier {
    PrivilegedEbpf,     // Full kernel ring-buffer interception (requires CAP_BPF)
    LandlockSeccomp,    // Unprivileged Linux LSM sandboxing
    RestrictedWasm,     // WasmEdge runtime isolation
}

pub struct SystemMembrane {
    active_tier: IsolationTier,
}

impl SystemMembrane {
    pub fn negotiate() -> Self {
        if caps::has_cap(None, caps::CapSet::Effective, caps::Capability::CAP_BPF).unwrap_or(false) {
            Self { active_tier: IsolationTier::PrivilegedEbpf }
        } else if landlock::Ruleset::default().is_supported() {
            Self { active_tier: IsolationTier::LandlockSeccomp }
        } else {
            Self { active_tier: IsolationTier::RestrictedWasm }
        }
    }

    pub fn apply_execution_boundary(&self, pid: u32) -> Result<(), SecurityError> {
        match self.active_tier {
            IsolationTier::PrivilegedEbpf => attach_tracepoint_membrane(pid),
            IsolationTier::LandlockSeccomp => apply_landlock_sandbox(pid),
            IsolationTier::RestrictedWasm => enforce_wasm_fuel_limits(pid),
        }
    }
}

```

## 7. CTO Operational Gates & Verification Criteria

Progression between technical phases cannot proceed on estimates; each transition is gated strictly by executable assertions:

```
  ┌────────────────────────────────────────────────────────┐
  │  GATE 1: Cryptographic Sanitization & Build Determinism│
  │  - Zero committed credentials in git history           │
  │  - Hermetic FlatBuffers build (zero static binaries)   │
  │  - Dynamic mTLS local negotiation verified             │
  └───────────────────────────┬────────────────────────────┘
                              │ PASS
                              ▼
  ┌────────────────────────────────────────────────────────┐
  │  GATE 2: Engine Fusion & IPC Reliability               │
  │  - Duplicate crates pruned; workspace builds clean     │
  │  - Subprocess watchdog survives synthetic deadlocks    │
  │  - Zero unaligned memory reads under ASan & Miri       │
  └───────────────────────────┬────────────────────────────┘
                              │ PASS
                              ▼
  ┌────────────────────────────────────────────────────────┐
  │  GATE 3: Formal Methods & Domain Correctness           │
  │  - Closed-loop DRC/ERC prevents invalid netlist saves  │
  │  - SMT solvers prove hardware safety invariants        │
  │  - Tree-Sitter language registry fully populated       │
  └───────────────────────────┬────────────────────────────┘
                              │ PASS
                              ▼
  ┌────────────────────────────────────────────────────────┐
  │  GATE 4: Autonomous Adaptation & Containment           │
  │  - GRPO reward loop converges on compilation benchmarks│
  │  - Membrane seamlessly downgrades without CAP_BPF      │
  │  - Synthesized Wasm tools mount dynamically            │
  └────────────────────────────────────────────────────────┘

```

### Mandatory CI Verification Pipeline Commands

Before approving any merge into the release branch:

```
# 1. Supply Chain & Secrets Audit
cargo audit
gitleaks detect --source . --verbose
cargo deny check bans licenses sources

# 2. Memory Safety & Undefined Behavior Verification
cargo clean
RUSTFLAGS="-Zsanitizer=address" cargo test -p oxide-engines -p oxide-kernels --target x86_64-unknown-linux-gnu
cargo miri test -p oxide-core --lib channel::tests

# 3. Formal Constraints & Verification Harnesses
cargo kani -p formal-verify
cargo test --workspace --all-targets --exclude placeholder-crates

```
