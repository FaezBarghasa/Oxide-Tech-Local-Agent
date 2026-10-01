pub mod discovery;
pub mod packet;
pub mod pairing;
pub mod telemetry_sync;

pub use discovery::{DiscoveredNode, SwarmDiscoveryService, SwarmRole};
pub use packet::{ActuationCommand, HardwareTelemetryPacket};
pub use pairing::{PairingSession, SwarmPairingManager, TaskOffloadDecision};
pub use telemetry_sync::EdgeSwarmBridge;
