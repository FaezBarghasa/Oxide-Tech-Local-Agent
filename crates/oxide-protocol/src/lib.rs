use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum OxideProtocolError {
    #[error("Serialization failure: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Invalid DTX state transition: {0}")]
    InvalidState(String),

    #[error("Transaction timeout for DTX {0}")]
    TransactionTimeout(String),

    #[error("RPC Error [{code}]: {message}")]
    RpcError { code: i64, message: String },
}

/// A Distributed Transaction ID uniquely identifying multi-domain operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DtxId(pub Uuid);

impl DtxId {
    /// Generate a new UUIDv7 time-ordered Distributed Transaction ID
    pub fn new_v7() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for DtxId {
    fn default() -> Self {
        Self::new_v7()
    }
}

impl std::fmt::Display for DtxId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Domain target in the Oxide ecosystem
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainTarget {
    FirmwareIde,
    OxideEda,
    Oxide3d,
    CrossDomainVerifier,
}

/// Status of a Distributed Transaction across domains
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DtxStatus {
    Pending,
    Committed,
    RolledBack,
    Failed,
}

/// Distributed Transaction metadata record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DtxRecord {
    pub dtx_id: DtxId,
    pub title: String,
    pub initiator: String,
    pub domains: Vec<DomainTarget>,
    pub status: DtxStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DtxRecord {
    pub fn new(
        title: impl Into<String>,
        initiator: impl Into<String>,
        domains: Vec<DomainTarget>,
    ) -> Self {
        let now = Utc::now();
        Self {
            dtx_id: DtxId::new_v7(),
            title: title.into(),
            initiator: initiator.into(),
            domains,
            status: DtxStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }
}

/// Standard JSON-RPC 2.0 Request for Oxide-MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxideMcpRequest<T = serde_json::Value> {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: OxideMcpParams<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxideMcpParams<T = serde_json::Value> {
    pub name: String,
    pub arguments: T,
    pub dtx_id: Option<DtxId>,
}

/// Standard JSON-RPC 2.0 Response for Oxide-MCP
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxideMcpResponse<T = serde_json::Value> {
    pub jsonrpc: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<OxideMcpError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OxideMcpError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Arguments for `eda.place_component`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdaPlaceComponentArgs {
    pub ref_des: String,
    pub package: String,
    pub x_mm: f64,
    pub y_mm: f64,
    pub rotation_deg: f64,
    pub layer: String,
}

/// Arguments for `eda.route_differential_pair`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdaRouteDiffPairArgs {
    pub net_name: String,
    pub impedance_ohms: f64,
    pub layer: String,
}

/// Arguments for `cad.import_step`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadImportStepArgs {
    pub file_path: String,
    pub target_body: String,
}

/// Arguments for `cad.brep_boolean`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CadBrepBooleanArgs {
    pub operation: String, // "union", "difference", "intersection"
    pub tool_body: String,
    pub target_body: String,
}

/// Arguments for `sim.run_fea_thermal`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimThermalFeaArgs {
    pub power_sources_watts: std::collections::HashMap<String, f64>,
    pub ambient_temp_c: f64,
    pub enclosure_material: String,
}

/// Output of `sim.run_fea_thermal`
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimThermalFeaResult {
    pub max_temperature_c: f64,
    pub silicon_junction_temp_c: f64,
    pub hot_spots: Vec<ThermalHotSpot>,
    pub passes_threshold: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThermalHotSpot {
    pub location_name: String,
    pub temp_c: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dtx_id_v7_generation() {
        let id1 = DtxId::new_v7();
        let id2 = DtxId::new_v7();
        assert_ne!(id1, id2);
        assert!(!id1.to_string().is_empty());
    }

    #[test]
    fn test_mcp_request_serialization() {
        let dtx = DtxId::new_v7();
        let req = OxideMcpRequest {
            jsonrpc: "2.0".to_string(),
            id: "req_01".to_string(),
            method: "tools/call".to_string(),
            params: OxideMcpParams {
                name: "eda.route_differential_pair".to_string(),
                arguments: EdaRouteDiffPairArgs {
                    net_name: "USB_DP_DN".to_string(),
                    impedance_ohms: 90.0,
                    layer: "F.Cu".to_string(),
                },
                dtx_id: Some(dtx),
            },
        };

        let json = serde_json::to_string(&req).expect("Failed to serialize");
        assert!(json.contains("USB_DP_DN"));
        assert!(json.contains(&dtx.to_string()));
    }
}
