use petgraph::algo::toposort;
use petgraph::graph::{Graph, NodeIndex};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::ParametricError;
use crate::constraint_solver::GeometricConstraint;

/// Coordinate reference plane for 2D sketches.
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
#[serde(tag = "plane_type", rename_all = "snake_case")]
pub enum Plane {
    #[default]
    XY,
    XZ,
    YZ,
    Custom {
        origin: [f64; 3],
        normal: [f64; 3],
    },
}

/// 3D Boolean CSG operation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BooleanType {
    Union,
    Difference,
    Intersection,
}

/// 2D geometric entity inside a sketch.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "entity_type", rename_all = "snake_case")]
pub enum SketchEntity {
    Point {
        id: usize,
        x: f64,
        y: f64,
    },
    Line {
        p1: usize,
        p2: usize,
    },
    Arc {
        center: usize,
        radius: f64,
        start_angle: f64,
        end_angle: f64,
    },
}

/// Parametric CAD construction history operation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "op_type", rename_all = "snake_case")]
pub enum CadOperation {
    Sketch2D {
        id: Uuid,
        name: String,
        plane: Plane,
        points: Vec<[f64; 2]>,
        constraints: Vec<GeometricConstraint>,
        entities: Vec<SketchEntity>,
    },
    Extrude {
        id: Uuid,
        name: String,
        profile_id: Uuid,
        distance: f64,
        direction: [f64; 3],
    },
    Fillet {
        id: Uuid,
        name: String,
        target_op_id: Uuid,
        target_edges: Vec<Uuid>,
        radius: f64,
    },
    Chamfer {
        id: Uuid,
        name: String,
        target_op_id: Uuid,
        target_edges: Vec<Uuid>,
        distance: f64,
    },
    Boolean {
        id: Uuid,
        name: String,
        boolean_op: BooleanType,
        target_a: Uuid,
        target_b: Uuid,
    },
}

impl CadOperation {
    pub fn id(&self) -> Uuid {
        match self {
            CadOperation::Sketch2D { id, .. }
            | CadOperation::Extrude { id, .. }
            | CadOperation::Fillet { id, .. }
            | CadOperation::Chamfer { id, .. }
            | CadOperation::Boolean { id, .. } => *id,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            CadOperation::Sketch2D { name, .. }
            | CadOperation::Extrude { name, .. }
            | CadOperation::Fillet { name, .. }
            | CadOperation::Chamfer { name, .. }
            | CadOperation::Boolean { name, .. } => name.as_str(),
        }
    }
}

/// Dependency relationship edge between CAD operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyEdge {
    ParentChild,
    Reference,
}

/// The Parametric Feature DAG maintaining the sequential construction history.
#[derive(Debug, Clone, Default)]
pub struct FeatureDAG {
    pub graph: Graph<CadOperation, DependencyEdge, petgraph::Directed>,
    pub node_map: HashMap<Uuid, NodeIndex>,
}

impl FeatureDAG {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            node_map: HashMap::new(),
        }
    }

    /// Add a CAD operation to the DAG and automatically create dependency edges if referenced IDs exist.
    pub fn add_operation(&mut self, op: CadOperation) -> NodeIndex {
        let id = op.id();
        let parent_ids: Vec<Uuid> = match &op {
            CadOperation::Sketch2D { .. } => vec![],
            CadOperation::Extrude { profile_id, .. } => vec![*profile_id],
            CadOperation::Fillet { target_op_id, .. } => vec![*target_op_id],
            CadOperation::Chamfer { target_op_id, .. } => vec![*target_op_id],
            CadOperation::Boolean {
                target_a, target_b, ..
            } => vec![*target_a, *target_b],
        };

        let idx = self.graph.add_node(op);
        self.node_map.insert(id, idx);

        for parent_id in parent_ids {
            if let Some(&parent_idx) = self.node_map.get(&parent_id) {
                self.graph
                    .add_edge(parent_idx, idx, DependencyEdge::ParentChild);
            }
        }

        idx
    }

    /// Add a dependency edge from parent operation to child operation.
    pub fn add_dependency(
        &mut self,
        from_id: Uuid,
        to_id: Uuid,
        edge: DependencyEdge,
    ) -> Result<(), ParametricError> {
        let from_idx = self.find_node_by_uuid(from_id)?;
        let to_idx = self.find_node_by_uuid(to_id)?;
        self.graph.add_edge(from_idx, to_idx, edge);
        Ok(())
    }

    /// Find node index by UUID.
    pub fn find_node_by_uuid(&self, id: Uuid) -> Result<NodeIndex, ParametricError> {
        self.node_map
            .get(&id)
            .copied()
            .ok_or_else(|| ParametricError::NodeNotFound(id.to_string()))
    }

    /// Get operation reference by UUID.
    pub fn get_op(&self, id: Uuid) -> Option<&CadOperation> {
        self.node_map.get(&id).map(|&idx| &self.graph[idx])
    }

    /// Get mutable operation reference by UUID.
    pub fn get_op_mut(&mut self, id: Uuid) -> Option<&mut CadOperation> {
        if let Some(&idx) = self.node_map.get(&id) {
            Some(&mut self.graph[idx])
        } else {
            None
        }
    }

    /// Return topological ordering of operations from root features to leaves.
    pub fn topological_order(&self) -> Result<Vec<NodeIndex>, ParametricError> {
        toposort(&self.graph, None).map_err(|e| {
            ParametricError::CyclicDependency(format!(
                "Cyclic dependency detected at node {:?}",
                e.node_id()
            ))
        })
    }

    /// Generates a semantic summary of the CAD model for token-efficient LLM context.
    pub fn semantic_summary(&self) -> String {
        let mut out = String::from("## Parametric CAD Feature History\n");
        if let Ok(order) = self.topological_order() {
            for (step, &idx) in order.iter().enumerate() {
                let op = &self.graph[idx];
                match op {
                    CadOperation::Sketch2D {
                        id,
                        name,
                        plane,
                        points,
                        constraints,
                        ..
                    } => {
                        out.push_str(&format!(
                            "{}. Sketch2D '{}' [id={}] plane={:?} (points={}, constraints={})\n",
                            step + 1,
                            name,
                            id,
                            plane,
                            points.len(),
                            constraints.len()
                        ));
                    }
                    CadOperation::Extrude {
                        id,
                        name,
                        distance,
                        direction,
                        ..
                    } => {
                        out.push_str(&format!(
                            "{}. Extrude '{}' [id={}] depth={:.2}mm dir={:?}\n",
                            step + 1,
                            name,
                            id,
                            distance,
                            direction
                        ));
                    }
                    CadOperation::Fillet {
                        id,
                        name,
                        radius,
                        target_edges,
                        ..
                    } => {
                        out.push_str(&format!(
                            "{}. Fillet '{}' [id={}] radius={:.2}mm (edges={})\n",
                            step + 1,
                            name,
                            id,
                            radius,
                            target_edges.len()
                        ));
                    }
                    CadOperation::Chamfer {
                        id,
                        name,
                        distance,
                        target_edges,
                        ..
                    } => {
                        out.push_str(&format!(
                            "{}. Chamfer '{}' [id={}] dist={:.2}mm (edges={})\n",
                            step + 1,
                            name,
                            id,
                            distance,
                            target_edges.len()
                        ));
                    }
                    CadOperation::Boolean {
                        id,
                        name,
                        boolean_op,
                        ..
                    } => {
                        out.push_str(&format!(
                            "{}. Boolean '{}' [id={}] type={:?}\n",
                            step + 1,
                            name,
                            id,
                            boolean_op
                        ));
                    }
                }
            }
        }
        out
    }
}
