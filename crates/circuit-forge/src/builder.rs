use petgraph::graph::NodeIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::CircuitError;
use crate::netlist::{CircuitGraph, NetlistNode, PinConnection};

/// Structured commands for programmatic or LLM-driven circuit creation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum CircuitCommand {
    /// Add a component to the schematic.
    AddComponent {
        ref_des: String,
        lib_id: String,
        value: String,
        #[serde(default)]
        footprint: Option<String>,
    },
    /// Connect two components' pins together via a named net.
    Connect {
        comp1: String,
        pin1: String,
        comp2: String,
        pin2: String,
        net_name: String,
    },
    /// Connect a single component's pin to a named net (e.g. "VCC", "GND").
    ConnectNet {
        comp: String,
        pin: String,
        net_name: String,
    },
}

/// A serialized sequence of circuit builder commands.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CircuitScript {
    pub commands: Vec<CircuitCommand>,
}

/// LLM-friendly fluent builder for constructing circuit topologies in memory.
#[derive(Debug, Default)]
pub struct CircuitBuilder {
    graph: CircuitGraph,
    component_map: HashMap<String, NodeIndex>,
    net_map: HashMap<String, NodeIndex>,
}

impl CircuitBuilder {
    /// Create a new empty `CircuitBuilder`.
    pub fn new() -> Self {
        Self {
            graph: CircuitGraph::new_undirected(),
            component_map: HashMap::new(),
            net_map: HashMap::new(),
        }
    }

    /// Add a component to the circuit graph with default auto footprint.
    pub fn add_component(&mut self, ref_des: &str, lib_id: &str, value: &str) -> &mut Self {
        self.add_component_with_footprint(ref_des, lib_id, value, "auto")
    }

    /// Add a component to the circuit graph with an explicit PCB footprint.
    pub fn add_component_with_footprint(
        &mut self,
        ref_des: &str,
        lib_id: &str,
        value: &str,
        footprint: &str,
    ) -> &mut Self {
        let node = self.graph.add_node(NetlistNode::Component {
            lib_id: lib_id.to_string(),
            ref_des: ref_des.to_string(),
            value: value.to_string(),
            footprint: footprint.to_string(),
        });
        self.component_map.insert(ref_des.to_string(), node);
        self
    }

    /// Get existing net node index or create a new one.
    pub fn get_or_create_net(&mut self, net_name: &str) -> NodeIndex {
        if let Some(&node_idx) = self.net_map.get(net_name) {
            node_idx
        } else {
            let node_idx = self.graph.add_node(NetlistNode::Net {
                name: net_name.to_string(),
                id: Uuid::new_v4(),
            });
            self.net_map.insert(net_name.to_string(), node_idx);
            node_idx
        }
    }

    /// Connect two components' pins via a named net.
    pub fn connect(
        &mut self,
        comp1: &str,
        pin1: &str,
        comp2: &str,
        pin2: &str,
        net_name: &str,
    ) -> Result<&mut Self, CircuitError> {
        let c1_idx = self
            .component_map
            .get(comp1)
            .copied()
            .ok_or_else(|| CircuitError::ComponentNotFound(comp1.to_string()))?;

        let c2_idx = self
            .component_map
            .get(comp2)
            .copied()
            .ok_or_else(|| CircuitError::ComponentNotFound(comp2.to_string()))?;

        let net_node = self.get_or_create_net(net_name);

        self.graph
            .add_edge(c1_idx, net_node, PinConnection::new(pin1));
        self.graph
            .add_edge(c2_idx, net_node, PinConnection::new(pin2));

        Ok(self)
    }

    /// Connect a component pin to a named net.
    pub fn connect_net(
        &mut self,
        comp: &str,
        pin: &str,
        net_name: &str,
    ) -> Result<&mut Self, CircuitError> {
        let c_idx = self
            .component_map
            .get(comp)
            .copied()
            .ok_or_else(|| CircuitError::ComponentNotFound(comp.to_string()))?;

        let net_node = self.get_or_create_net(net_name);

        self.graph
            .add_edge(c_idx, net_node, PinConnection::new(pin));

        Ok(self)
    }

    /// Build and return the final `CircuitGraph`.
    pub fn build(self) -> CircuitGraph {
        self.graph
    }

    /// Execute a sequence of `CircuitCommand`s to construct a `CircuitGraph`.
    pub fn from_commands(commands: &[CircuitCommand]) -> Result<CircuitGraph, CircuitError> {
        let mut builder = Self::new();
        for cmd in commands {
            match cmd {
                CircuitCommand::AddComponent {
                    ref_des,
                    lib_id,
                    value,
                    footprint,
                } => {
                    let fp = footprint.as_deref().unwrap_or("auto");
                    builder.add_component_with_footprint(ref_des, lib_id, value, fp);
                }
                CircuitCommand::Connect {
                    comp1,
                    pin1,
                    comp2,
                    pin2,
                    net_name,
                } => {
                    builder.connect(comp1, pin1, comp2, pin2, net_name)?;
                }
                CircuitCommand::ConnectNet {
                    comp,
                    pin,
                    net_name,
                } => {
                    builder.connect_net(comp, pin, net_name)?;
                }
            }
        }
        Ok(builder.build())
    }

    /// Parse a JSON string representing `CircuitScript` or a list of `CircuitCommand`s and build the graph.
    pub fn from_json(json_str: &str) -> Result<CircuitGraph, CircuitError> {
        if let Ok(script) = serde_json::from_str::<CircuitScript>(json_str) {
            Self::from_commands(&script.commands)
        } else if let Ok(commands) = serde_json::from_str::<Vec<CircuitCommand>>(json_str) {
            Self::from_commands(&commands)
        } else {
            Err(CircuitError::JsonParseError(
                "Invalid CircuitScript or CircuitCommand array JSON format".to_string(),
            ))
        }
    }
}
