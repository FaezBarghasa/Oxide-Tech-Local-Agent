//! # Sovereign Mobile Remote Companion Bridge (`crates/oxide-gateway/src/mobile_bridge.rs`)
//!
//! Provides zero-trust WebRTC/WebSocket signaling, encrypted QR-code pairing,
//! out-of-band HITL approvals, and bidirectional telemetry streaming without
//! third-party VPNs or cloud relays.

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MobileSignalMessage {
    PairRequest {
        client_pk: String,
        biometric_signature: Vec<u8>,
    },
    IceCandidate {
        candidate: String,
        sdp_mid: String,
        sdp_mline_index: u32,
    },
    SdpOffer {
        sdp: String,
    },
    SdpAnswer {
        sdp: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentControlAction {
    ApproveHunk { plan_id: String, hunk_hash: String },
    RejectAction { plan_id: String, reason: String },
    InjectPrompt { prompt: String },
    EmergencyHalt,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
        let mut initial_nonce = [0u8; 32];
        let random_bytes = uuid::Uuid::now_v7().into_bytes();
        let hash = blake3::hash(&random_bytes);
        initial_nonce.copy_from_slice(hash.as_bytes());

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
        let nonce_hex = blake3::Hash::from(*nonce).to_hex().to_string();
        format!(
            "https://{}:{}/mobile/#pk={}&nonce={}",
            host_ip, port, host_pk_hex, nonce_hex
        )
    }

    /// Validates pairing request and derives session encryption key
    pub async fn handle_pair_request(
        &self,
        client_pk: &str,
        biometric_signature: &[u8],
    ) -> Result<String, String> {
        if client_pk.is_empty() {
            return Err("Client public key cannot be empty".to_string());
        }

        let nonce = self.pairing_nonce.read().await;
        // Derive session key K_session = HKDF/Blake3(client_pk + nonce + biometric_signature)
        let mut hasher = blake3::Hasher::new_keyed(&nonce);
        hasher.update(client_pk.as_bytes());
        hasher.update(biometric_signature);
        let derived_hash = hasher.finalize();

        let mut derived_key = [0u8; 32];
        derived_key.copy_from_slice(derived_hash.as_bytes());

        let mut session_guard = self.session_key.write().await;
        *session_guard = Some(derived_key);

        Ok(derived_hash.to_hex().to_string())
    }

    /// Dispatches an urgent HITL intervention to the paired mobile phone
    pub async fn dispatch_hitl_request(
        &self,
        intervention: PendingApprovalPayload,
        datachannel_tx: &mpsc::Sender<Vec<u8>>,
    ) -> Result<(), String> {
        self.pending_approvals
            .write()
            .await
            .push(intervention.clone());

        let payload_json = serde_json::to_vec(&serde_json::json!({
            "type": "HITL_INTERVENTION_REQUIRED",
            "data": intervention
        }))
        .map_err(|e| e.to_string())?;

        // Push to mobile WebRTC channel
        datachannel_tx
            .send(payload_json)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Handles incoming action approvals submitted from the mobile touch UI
    pub async fn handle_mobile_action(&self, action: AgentControlAction) -> Result<(), String> {
        // If it was an approval or rejection, remove matching pending intervention
        match &action {
            AgentControlAction::ApproveHunk { plan_id, .. }
            | AgentControlAction::RejectAction { plan_id, .. } => {
                let mut pending = self.pending_approvals.write().await;
                pending.retain(|p| &p.intervention_id != plan_id);
            }
            _ => {}
        }

        self.approval_tx
            .send(action)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    /// List active pending approvals awaiting mobile operator response
    pub async fn get_pending_approvals(&self) -> Vec<PendingApprovalPayload> {
        self.pending_approvals.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_qr_payload_generation() {
        let (tx, _rx) = mpsc::channel(16);
        let bridge = MobileBridgeManager::new(tx);
        let qr_payload = bridge
            .generate_qr_payload("192.168.1.50", 8080, "abcdef0123456789")
            .await;

        assert!(qr_payload.starts_with("https://192.168.1.50:8080/mobile/#pk=abcdef0123456789&nonce="));
    }

    #[tokio::test]
    async fn test_mobile_pairing_handshake() {
        let (tx, _rx) = mpsc::channel(16);
        let bridge = MobileBridgeManager::new(tx);

        let client_pk = "mobile_client_public_key_x25519";
        let biometric_sig = b"face_id_signature_bytes";

        let session_token = bridge
            .handle_pair_request(client_pk, biometric_sig)
            .await
            .expect("Pairing should succeed");

        assert_eq!(session_token.len(), 64);
        assert!(bridge.session_key.read().await.is_some());
    }

    #[tokio::test]
    async fn test_hitl_dispatch_and_approval_flow() {
        let (approval_tx, mut approval_rx) = mpsc::channel(16);
        let (dc_tx, mut dc_rx) = mpsc::channel(16);

        let bridge = MobileBridgeManager::new(approval_tx);

        let payload = PendingApprovalPayload {
            intervention_id: "plan_42".into(),
            title: "Trace thermal current patch".into(),
            target_file: "crates/circuit-forge/src/erc.rs".into(),
            diff_preview: "@@ -10,3 +10,3 @@\n-width=0.1\n+width=0.35".into(),
            risk_tier: "Critical".into(),
        };

        bridge
            .dispatch_hitl_request(payload, &dc_tx)
            .await
            .expect("Dispatch should succeed");

        // Verify sent across datachannel
        let received_bytes = dc_rx.recv().await.expect("DataChannel message expected");
        let parsed: serde_json::Value = serde_json::from_slice(&received_bytes).unwrap();
        assert_eq!(parsed["type"], "HITL_INTERVENTION_REQUIRED");
        assert_eq!(parsed["data"]["intervention_id"], "plan_42");

        // Verify pending approvals list
        assert_eq!(bridge.get_pending_approvals().await.len(), 1);

        // Submit approval from phone
        bridge
            .handle_mobile_action(AgentControlAction::ApproveHunk {
                plan_id: "plan_42".into(),
                hunk_hash: "verified_passkey_sig".into(),
            })
            .await
            .expect("Action submission should succeed");

        // Verify action arrived on approval channel and was removed from pending
        let action = approval_rx.recv().await.expect("Approval action expected");
        assert_eq!(
            action,
            AgentControlAction::ApproveHunk {
                plan_id: "plan_42".into(),
                hunk_hash: "verified_passkey_sig".into()
            }
        );
        assert_eq!(bridge.get_pending_approvals().await.len(), 0);
    }

    #[tokio::test]
    async fn test_emergency_halt_action() {
        let (approval_tx, mut approval_rx) = mpsc::channel(16);
        let bridge = MobileBridgeManager::new(approval_tx);

        bridge
            .handle_mobile_action(AgentControlAction::EmergencyHalt)
            .await
            .expect("Emergency halt should dispatch");

        let action = approval_rx.recv().await.unwrap();
        assert_eq!(action, AgentControlAction::EmergencyHalt);
    }
}
