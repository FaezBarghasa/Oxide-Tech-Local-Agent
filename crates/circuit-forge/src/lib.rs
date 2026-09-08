use thiserror::Error;

pub mod builder;
pub mod erc;
pub mod kicad_serializer;
pub mod netlist;

pub use builder::{CircuitBuilder, CircuitCommand, CircuitScript};
pub use erc::{ErcReport, TopologyIssue, run_erc};
pub use kicad_serializer::{calculate_grid_layout, serialize_to_kicad_sch};
pub use netlist::{CircuitGraph, NetlistNode, PinConnection};

/// Errors encountered during circuit construction, validation, or serialization.
#[derive(Debug, Error)]
pub enum CircuitError {
    #[error("Component '{0}' was not found in the circuit graph")]
    ComponentNotFound(String),

    #[error("Net '{0}' was not found in the circuit graph")]
    NetNotFound(String),

    #[error("Failed to parse JSON circuit specification: {0}")]
    JsonParseError(String),

    #[error("Electrical Rule Check (ERC) failure: {0}")]
    ErcViolation(String),

    #[error("KiCad serialization failure: {0}")]
    SerializationError(String),
}
