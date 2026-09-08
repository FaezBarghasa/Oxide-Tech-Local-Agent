use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use uuid::Uuid;

use crate::netlist::{CircuitGraph, NetlistNode};

/// Spacing constants for KiCad 8 schematic grid (in millimeters).
const GRID_ORIGIN_X: f64 = 50.8;
const GRID_ORIGIN_Y: f64 = 50.8;
const COMPONENT_SPACING_X: f64 = 38.1;
const COMPONENT_SPACING_Y: f64 = 30.48;
const COLS_PER_ROW: usize = 4;

/// Calculate collision-free grid coordinates (x, y) for all components in the circuit.
pub fn calculate_grid_layout(graph: &CircuitGraph) -> HashMap<NodeIndex, (f64, f64)> {
    let mut layout = HashMap::new();
    let mut comp_indices: Vec<NodeIndex> = graph
        .node_indices()
        .filter(|&idx| graph[idx].is_component())
        .collect();

    // Sort deterministically by ref_des
    comp_indices.sort_by_key(|&idx| graph[idx].ref_des().unwrap_or_default().to_string());

    for (i, &idx) in comp_indices.iter().enumerate() {
        let col = i % COLS_PER_ROW;
        let row = i / COLS_PER_ROW;
        let x = GRID_ORIGIN_X + (col as f64) * COMPONENT_SPACING_X;
        let y = GRID_ORIGIN_Y + (row as f64) * COMPONENT_SPACING_Y;
        layout.insert(idx, (x, y));
    }

    layout
}

/// Serialize a verified `CircuitGraph` into KiCad 8 `.kicad_sch` S-expression format.
pub fn serialize_to_kicad_sch(graph: &CircuitGraph) -> String {
    let mut out = String::new();
    out.push_str("(kicad_sch\n");
    out.push_str("  (version 20231120)\n");
    out.push_str("  (generator \"NexusForge_CircuitForge\")\n");
    out.push_str("  (generator_version \"8.0\")\n");
    out.push_str(&format!("  (uuid \"{}\")\n", Uuid::new_v4()));
    out.push_str("  (paper \"A4\")\n\n");

    let layout = calculate_grid_layout(graph);

    // 1. Serialize Components (Symbols)
    for node_idx in graph.node_indices() {
        if let NetlistNode::Component {
            lib_id,
            ref_des,
            value,
            footprint,
        } = &graph[node_idx]
        {
            let (x, y) = layout
                .get(&node_idx)
                .copied()
                .unwrap_or((GRID_ORIGIN_X, GRID_ORIGIN_Y));
            let sym_uuid = Uuid::new_v4();

            out.push_str(&format!(
                "  (symbol (lib_id \"{}\") (at {:.2} {:.2} 0) (unit 1)\n",
                lib_id, x, y
            ));
            out.push_str(&format!("    (uuid \"{}\")\n", sym_uuid));
            out.push_str(&format!(
                "    (property \"Reference\" \"{}\" (at {:.2} {:.2} 0)\n      (effects (font (size 1.27 1.27)))\n    )\n",
                ref_des, x, y - 5.08
            ));
            out.push_str(&format!(
                "    (property \"Value\" \"{}\" (at {:.2} {:.2} 0)\n      (effects (font (size 1.27 1.27)))\n    )\n",
                value, x, y + 5.08
            ));
            out.push_str(&format!(
                "    (property \"Footprint\" \"{}\" (at {:.2} {:.2} 0)\n      (effects (font (size 1.27 1.27) hide yes))\n    )\n",
                footprint, x, y
            ));
            out.push_str(&format!(
                "    (property \"Datasheet\" \"~\" (at {:.2} {:.2} 0)\n      (effects (font (size 1.27 1.27) hide yes))\n    )\n",
                x, y
            ));

            // Serialize pin references for this symbol
            for edge in graph.edges(node_idx) {
                let pin_name = &edge.weight().pin_name;
                out.push_str(&format!(
                    "    (pin \"{}\" (uuid \"{}\"))\n",
                    pin_name,
                    Uuid::new_v4()
                ));
            }

            out.push_str("  )\n\n");
        }
    }

    // 2. Serialize Nets (Wires & Net Labels)
    let mut net_pin_offset: HashMap<NodeIndex, usize> = HashMap::new();

    for node_idx in graph.node_indices() {
        if let NetlistNode::Net { name, id } = &graph[node_idx] {
            let edges: Vec<_> = graph.edges(node_idx).collect();

            for edge in edges {
                let comp_node = edge.target();
                let _pin_conn = edge.weight();
                if let Some(&(cx, cy)) = layout.get(&comp_node) {
                    let offset_idx = *net_pin_offset.entry(comp_node).or_insert(0);
                    net_pin_offset.insert(comp_node, offset_idx + 1);

                    let pin_y_offset = (offset_idx as f64) * 2.54;
                    let wire_start_x = cx + 7.62;
                    let wire_start_y = cy + pin_y_offset;
                    let wire_end_x = cx + 15.24;
                    let wire_end_y = wire_start_y;

                    // Write connection wire stub
                    out.push_str(&format!(
                        "  (wire (pts (xy {:.2} {:.2}) (xy {:.2} {:.2}))\n    (stroke (width 0) (type default))\n    (uuid \"{}\")\n  )\n",
                        wire_start_x, wire_start_y, wire_end_x, wire_end_y, Uuid::new_v4()
                    ));

                    // Write net label with pin information
                    out.push_str(&format!(
                        "  (label \"{}\" (at {:.2} {:.2} 0) (fields_autoplaced yes)\n    (effects (font (size 1.27 1.27)) (justify left bottom))\n    (uuid \"{}\")\n  )\n",
                        name, wire_end_x, wire_end_y, id
                    ));
                }
            }
        }
    }

    out.push_str(")\n");
    out
}
