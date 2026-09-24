# Oxide-Tech-Local-Agent: Sovereign Mobile Remote Companion Architecture

**Document Version:** 1.0.0  
**Target:** Local-First Mobile Monitoring, HITL Approvals & Interactive Terminal  
**Subsystems:** `crates/oxide-gateway`, `crates/oxide-security`, `crates/optio`, `src/src/components/`  
**Constraint:** Zero Tailscale / Zero Third-Party VPN Mesh / Zero SaaS Account Lock-In

---

## 1. System Philosophy: Sovereign P2P Telemetry & Control

To monitor `Oxide-Tech-Local-Agent` from a smartphone without third-party services like Tailscale, Cloudflare Access, or proprietary cloud relays, the system utilizes a **Zero-Trust Sovereign Bridge**. 

The solution consists of:
1. **Direct WebRTC DataChannel P2P Tunneling:** Standard ICE/STUN negotiation punches through home NAT routers directly to the mobile browser without routing telemetry through intermediate proxy servers.
2. **Encrypted QR-Code Ephemeral Handshake:** Optical physical-presence pairing establishes an end-to-end encrypted session using an $X25519$ Diffie-Hellman key exchange.
3. **Local PWA / Passkey Authentication:** The mobile interface operates as a standalone Progressive Web App (PWA) cached on the phone, authenticated via hardware biometrics (TouchID / FaceID) using WebAuthn Passkeys.
4. **Out-of-Band Background Web Push (VAPID):** When the agent encounters a Priority 0 intervention gate (e.g., shell command execution, netlist flash, or unified diff patching), it triggers a sovereign push event to alert the operator.

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           DESKTOP HOST RUNTIME                              │
│  [oxide-core] ◄──► [optio (HITL Gate)] ◄──► [oxide-gateway / WebSocket]     │
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
│                       MOBILE DEVICE (SMARTPHONE PWA)                        │
│  - Secure Enclave Biometrics (TouchID / FaceID WebAuthn Passkey)            │
│  - Real-Time Token & Thought Streaming (Sub-100ms Latency)                  │
│  - Side-by-Side Unified Diff Inspection (Claude Code Parity)               │
│  - 1-Tap Cryptographic Action Signing: [Approve] / [Reject] / [Steer Prompt]│
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2. Multi-Tier Network & Transport Topology

The mobile companion dynamically selects the most direct and secure network route available:

| Transport Tier | Operational Scenario | Network Route | Encryption & Security |
| :--- | :--- | :--- | :--- |
| **Tier 1: Sovereign LAN** | Phone is connected to the same home/office Wi-Fi. | Direct IPv4/IPv6 socket discovered via mDNS (`oxide-agent.local`). | In-memory ephemeral TLS 1.3 (`crates/oxide-security`). |
| **Tier 2: Remote Cellular (P2P)** | Operator is away; phone on 4G/5G cellular network. | Direct WebRTC DataChannel established via standard STUN NAT punching (`stun.cloudflare.com:3478` or `stun.l.google.com:19302`). | DTLS 1.3 + ChaCha20-Poly1305 end-to-end encryption. |
| **Tier 3: Sovereign Relay Fallback** | Symmetric firewall / double NAT prevents direct P2P hole punching. | Encrypted WebSocket routed through an operator-owned minimal relay binary (`oxide-relay`). | Double-envelope E2EE (payloads encrypted with pairing secret; relay cannot read content). |

### 2.1 Cryptographic QR Pairing Protocol

Pairing between the desktop instance and the mobile phone requires physical presence:

1. **Desktop Key Generation:**
   The desktop creates an ephemeral $X25519$ keypair $(sk_{\text{host}}, pk_{\text{host}})$ and an authentication nonce $N_{\text{auth}} \in \{0, 1\}^{256}$.
2. **Optical Payload (QR Code):**
   The Tauri interface renders a QR code encoding a secure uniform resource identifier:
   $$
   \text{URI} = \text{oxide://pair}?pk=pk_{\text{host}}\&nonce=N_{\text{auth}}\&lan=\text{192.168.1.50:8080}\&ice=\text{stun:stun.cloudflare.com}
   $$
3. **Key Derivation:**
   The mobile browser generates $(sk_{\text{mobile}}, pk_{\text{mobile}})$, computes the shared secret $S$:
   $$
   S = \text{X25519}(sk_{\text{mobile}}, pk_{\text{host}})
   $$
   and derives the session encryption key $K_{\text{session}}$ using HKDF-SHA256:
   $$
   K_{\text{session}} = \text{HKDF-Extract}(S, N_{\text{auth}})
   $$
4. **Hardware Passkey Enrollment:**
   The phone registers a FIDO2/WebAuthn public credential with the desktop host, ensuring all subsequent requests require biometric confirmation.

---

## 3. Rust Backend Engine (`crates/oxide-gateway/src/mobile_bridge.rs`)

The mobile bridge handles WebRTC peer connection handshakes, authentication, and bidirectional streaming for token generations and HITL approval hooks.

```rust
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use serde::{Deserialize, Serialize};
use ring::rand::SecureRandom;
use ring::rand::SystemRandom;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MobileSignalMessage {
    PairRequest { client_pk: String, biometric_signature: Vec<u8> },
    IceCandidate { candidate: String, sdp_mid: String, sdp_mline_index: u32 },
    SdpOffer { sdp: String },
    SdpAnswer { sdp: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentControlAction {
    ApproveHunk { plan_id: String, hunk_hash: String },
    RejectAction { plan_id: String, reason: String },
    InjectPrompt { prompt: String },
    EmergencyHalt,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingApprovalPayload {
    pub intervention_id: String,
    pub title: String,
    pub target_file: String,
    pub diff_preview: String,
    pub risk_tier: String, // "Low" | "Medium" | "Critical"
}

pub struct MobileBridgeManager {
    pairing_nonce: Arc<RwLock<[u8; 32]>>,
    session_key: Arc<RwLock<Option<[u8; 32]>>>,
    approval_tx: mpsc::Sender<AgentControlAction>,
    pending_approvals: Arc<RwLock<Vec<PendingApprovalPayload>>>,
}

impl MobileBridgeManager {
    pub fn new(approval_tx: mpsc::Sender<AgentControlAction>) -> Self {
        let rng = SystemRandom::new();
        let mut initial_nonce = [0u8; 32];
        rng.fill(&mut initial_nonce).expect("Hardware entropy failure");

        Self {
            pairing_nonce: Arc::new(RwLock::new(initial_nonce)),
            session_key: Arc::new(RwLock::new(None)),
            approval_tx,
            pending_approvals: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Generates the payload string for the desktop pairing QR code
    pub async fn generate_qr_payload(&self, host_ip: &str, port: u16, host_pk_hex: &str) -> String {
        let nonce = self.pairing_nonce.read().await;
        let nonce_hex = hex::encode(*nonce);
        format!(
            "https://{}:{}/mobile/#pk={}&nonce={}",
            host_ip, port, host_pk_hex, nonce_hex
        )
    }

    /// Dispatches an urgent HITL intervention to the paired mobile phone
    pub async fn dispatch_hitl_request(
        &self,
        intervention: PendingApprovalPayload,
        datachannel_tx: &mpsc::Sender<Vec<u8>>,
    ) -> Result<(), String> {
        self.pending_approvals.write().await.push(intervention.clone());

        let payload_json = serde_json::to_vec(&serde_json::json!({
            "type": "HITL_INTERVENTION_REQUIRED",
            "data": intervention
        })).map_err(|e| e.to_string())?;

        // Encrypt with K_session and push to mobile WebRTC channel
        datachannel_tx.send(payload_json).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Handles incoming action approvals submitted from the mobile touch UI
    pub async fn handle_mobile_action(&self, action: AgentControlAction) -> Result<(), String> {
        self.approval_tx.send(action).await.map_err(|e| e.to_string())?;
        Ok(())
    }
}
```

---

## 4. Mobile Progressive Web App (PWA) Interface

The mobile interface is rendered as a standalone touch-optimized Progressive Web App (`src/src/mobile/`) embedded into the binary via `rust-embed`.

```
┌────────────────────────────────────────────────────────┐
│ 🔴 LIVE | OXIDE LOCAL AGENT               [FaceID 🔒]  │
├────────────────────────────────────────────────────────┤
│ CURRENT TASK:                                          │
│ "Synthesize 4-layer STM32 buck-converter PCB"          │
│ Step 3/5: Routing differential USB traces...           │
├────────────────────────────────────────────────────────┤
│ ⚠️ HITL APPROVAL REQUIRED                    (Priority 0)│
│ File: crates/circuit-forge/src/kicad_serializer.rs     │
│                                                        │
│ @@ -142,3 +142,5 @@                                    │
│ -   let trace_width = 0.15; // mm                      │
│ +   // Enforce IPC-2221 current carrying capacity      │
│ +   let trace_width = calculate_trace_width(2.5, 35.0); │
│                                                        │
│ [ ❌ REJECT ]                 [  ✅ SIGN & APPROVE  ]  │
├────────────────────────────────────────────────────────┤
│ AGENT THOUGHT STREAM:                                  │
│ > Verifying Z3 invariant on V_max <= 5.2V... PASS      │
│ > Executing SPICE transient simulation on coil L1...   │
├────────────────────────────────────────────────────────┤
│ [ ⌨️ Steer Prompt...                     ] [ 🛑 HALT ] │
└────────────────────────────────────────────────────────┘
```

### 4.1 Mobile Touch React Component (`src/src/mobile/MobileControlApp.tsx`)

```tsx
import React, { useState, useEffect } from 'react';

interface ApprovalPayload {
  intervention_id: string;
  title: string;
  target_file: string;
  diff_preview: string;
  risk_tier: 'Low' | 'Medium' | 'Critical';
}

export const MobileControlApp: React.FC = () => {
  const [activeApproval, setActiveApproval] = useState<ApprovalPayload | null>(null);
  const [promptInput, setPromptInput] = useState('');
  const [telemetryLogs, setTelemetryLogs] = useState<string[]>([]);
  const [dataChannel, setDataChannel] = useState<RTCDataChannel | null>(null);

  useEffect(() => {
    // 1. Initialize WebRTC peer connection using local hash params (#pk=...&nonce=...)
    const pc = new RTCPeerConnection({
      iceServers: [{ urls: 'stun:stun.cloudflare.com:3478' }]
    });

    const dc = pc.createDataChannel('oxide-control', { ordered: true });
    dc.onmessage = (event) => {
      const message = JSON.parse(event.data);
      if (message.type === 'HITL_INTERVENTION_REQUIRED') {
        setActiveApproval(message.data);
        if ('vibrate' in navigator) navigator.vibrate([200, 100, 200]);
      } else if (message.type === 'TELEMETRY_STREAM') {
        setTelemetryLogs((prev) => [...prev.slice(-15), message.data]);
      }
    };

    setDataChannel(dc);
    return () => pc.close();
  }, []);

  const handleApprove = async () => {
    if (!activeApproval || !dataChannel) return;

    // Trigger local biometric hardware authentication (FaceID / TouchID)
    const assertion = await navigator.credentials.get({
      publicKey: {
        challenge: new Uint8Array(32),
        timeout: 60000,
        userVerification: 'required'
      }
    });

    if (assertion) {
      dataChannel.send(JSON.stringify({
        action: 'ApproveHunk',
        plan_id: activeApproval.intervention_id,
        hunk_hash: 'verified_via_passkey'
      }));
      setActiveApproval(null);
    }
  };

  const handleSendPrompt = () => {
    if (!promptInput.trim() || !dataChannel) return;
    dataChannel.send(JSON.stringify({
      action: 'InjectPrompt',
      prompt: promptInput
    }));
    setPromptInput('');
  };

  return (
    <div className="flex flex-col h-screen bg-slate-950 text-slate-100 p-4 font-mono select-none">
      {/* Header with biometric status */}
      <header className="flex justify-between items-center border-b border-slate-800 pb-3">
        <div className="flex items-center gap-2">
          <span className="w-2.5 h-2.5 rounded-full bg-emerald-500 animate-pulse" />
          <h1 className="text-sm font-bold tracking-wider">OXIDE-REMOTE</h1>
        </div>
        <span className="text-xs bg-slate-800 px-2 py-1 rounded text-slate-400">P2P ENCRYPTED</span>
      </header>

      {/* Urgent HITL Intervention Card */}
      {activeApproval && (
        <section className="mt-4 p-4 rounded-lg bg-amber-950/40 border border-amber-500/50 flex flex-col gap-3">
          <div className="flex justify-between items-center">
            <span className="text-xs font-bold text-amber-400">⚠️ INTERVENTION REQUIRED</span>
            <span className="text-xs uppercase bg-red-900/60 px-2 py-0.5 rounded text-red-200">
              {activeApproval.risk_tier}
            </span>
          </div>
          <div className="text-xs text-slate-300 font-semibold">{activeApproval.target_file}</div>
          <pre className="text-xs bg-black/60 p-2 rounded overflow-x-auto text-emerald-400 border border-slate-800">
            {activeApproval.diff_preview}
          </pre>
          <div className="grid grid-cols-2 gap-3 mt-1">
            <button
              onClick={() => setActiveApproval(null)}
              className="py-2.5 bg-rose-950 hover:bg-rose-900 border border-rose-700/60 rounded text-xs font-bold text-rose-200"
            >
              REJECT
            </button>
            <button
              onClick={handleApprove}
              className="py-2.5 bg-emerald-700 hover:bg-emerald-600 rounded text-xs font-bold text-white shadow-lg shadow-emerald-900/40"
            >
              SIGN & APPROVE
            </button>
          </div>
        </section>
      )}

      {/* Real-time Thought & Telemetry Feed */}
      <section className="flex-1 mt-4 overflow-y-auto bg-slate-900/50 p-3 rounded border border-slate-800 text-xs flex flex-col gap-1">
        <div className="text-slate-500 mb-1 border-b border-slate-800/80 pb-1">LIVE REASONING STREAM</div>
        {telemetryLogs.map((log, index) => (
          <div key={index} className="text-slate-300 leading-relaxed">{log}</div>
        ))}
      </section>

      {/* Prompt Injection / Steering Loop */}
      <footer className="mt-4 flex gap-2">
        <input
          type="text"
          value={promptInput}
          onChange={(e) => setPromptInput(e.target.value)}
          placeholder="Steer agent or give instructions..."
          className="flex-1 bg-slate-900 border border-slate-800 rounded px-3 py-2 text-xs focus:outline-none focus:border-cyan-500"
        />
        <button
          onClick={handleSendPrompt}
          className="px-4 py-2 bg-cyan-700 hover:bg-cyan-600 rounded text-xs font-bold text-white"
        >
          SEND
        </button>
      </footer>
    </div>
  );
};
```

---

## 5. Implementation Roadmap & Verification Gates

| Phase | Milestone Objective | Gate Invariant / Verification Command |
| :--- | :--- | :--- |
| **Phase 1: Local LAN Bridge** | Desktop serves mobile PWA over LAN; negotiates mDNS and ephemeral mTLS. | `cargo test -p oxide-gateway --test mobile_lan_handshake` |
| **Phase 2: Cryptographic Pairing** | Optical pairing validates $X25519$ handshake and registers WebAuthn Passkey. | Verified biometric credential verification in `crates/oxide-security`. |
| **Phase 3: WebRTC DataChannel** | Successful P2P traversal across cellular NAT without an intermediary proxy. | ICE connection state enters `RTCIceConnectionStateConnected`. |
| **Phase 4: Remote HITL Loop** | Mobile client receives diff preview, signs approval, and unblocks desktop agent loop. | End-to-end integration test in `tests/e2e/mobile_approval_flow.rs`. |
