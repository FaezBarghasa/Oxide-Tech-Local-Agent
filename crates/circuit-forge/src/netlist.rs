use petgraph::graph::Graph;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a node in the bipartite circuit netlist graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum NetlistNode {
    /// A physical/logical schematic component (e.g., Resistor, IC, Capacitor).
    Component {
        /// KiCad library identifier, e.g. "Device:R", "Regulator_Linear:AP2112K-3.3"
        lib_id: String,
        /// Reference designator, e.g. "R1", "U1", "C1"
        ref_des: String,
        /// Component value or part number, e.g. "10k", "3.3V", "10uF"
        value: String,
        /// Assigned PCB footprint, e.g. "Resistor_SMD:R_0805_2012Metric"
        footprint: String,
    },
    /// An electrical net connecting component pins.
    Net {
        /// Net name, e.g. "VCC", "GND", "+3.3V", "I2C1_SCL"
        name: String,
        /// Unique net identifier
        id: Uuid,
    },
}

impl NetlistNode {
    /// Returns true if this node is a component.
    #[inline]
    pub fn is_component(&self) -> bool {
        matches!(self, NetlistNode::Component { .. })
    }

    /// Returns true if this node is an electrical net.
    #[inline]
    pub fn is_net(&self) -> bool {
        matches!(self, NetlistNode::Net { .. })
    }

    /// Returns the reference designator if this node is a component.
    #[inline]
    pub fn ref_des(&self) -> Option<&str> {
        match self {
            NetlistNode::Component { ref_des, .. } => Some(ref_des.as_str()),
            NetlistNode::Net { .. } => None,
        }
    }

    /// Returns the net name if this node is a net.
    #[inline]
    pub fn net_name(&self) -> Option<&str> {
        match self {
            NetlistNode::Net { name, .. } => Some(name.as_str()),
            NetlistNode::Component { .. } => None,
        }
    }

    /// Returns the component value if this node is a component.
    #[inline]
    pub fn value(&self) -> Option<&str> {
        match self {
            NetlistNode::Component { value, .. } => Some(value.as_str()),
            NetlistNode::Net { .. } => None,
        }
    }

    /// Returns the KiCad lib_id if this node is a component.
    #[inline]
    pub fn lib_id(&self) -> Option<&str> {
        match self {
            NetlistNode::Component { lib_id, .. } => Some(lib_id.as_str()),
            NetlistNode::Net { .. } => None,
        }
    }

    /// Returns the PCB footprint if this node is a component.
    #[inline]
    pub fn footprint(&self) -> Option<&str> {
        match self {
            NetlistNode::Component { footprint, .. } => Some(footprint.as_str()),
            NetlistNode::Net { .. } => None,
        }
    }
}

/// Represents an edge in the netlist bipartite graph connecting a Component to a Net.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinConnection {
    /// Name or function of the pin (e.g., "1", "2", "VIN", "VOUT", "GND", "EN").
    pub pin_name: String,
    /// Optional physical pin number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_number: Option<String>,
}

impl PinConnection {
    /// Create a new pin connection with a pin name.
    pub fn new(pin_name: impl Into<String>) -> Self {
        Self {
            pin_name: pin_name.into(),
            pin_number: None,
        }
    }

    /// Create a new pin connection with both pin name and physical pin number.
    pub fn with_number(pin_name: impl Into<String>, pin_number: impl Into<String>) -> Self {
        Self {
            pin_name: pin_name.into(),
            pin_number: Some(pin_number.into()),
        }
    }
}

/// The Circuit Graph: Bipartite undirected connections between Components and Nets.
pub type CircuitGraph = Graph<NetlistNode, PinConnection, petgraph::Undirected>;
