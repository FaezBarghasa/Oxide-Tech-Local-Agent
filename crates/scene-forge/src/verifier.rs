use glam::Vec3;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

use crate::mesh::MeshData;

/// Report from mathematical topological verification of 3D geometry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifoldReport {
    pub is_manifold: bool,
    pub is_closed: bool,
    pub euler_characteristic: i32,
    pub vertex_count: usize,
    pub edge_count: usize,
    pub face_count: usize,
    pub boundary_edges: Vec<[u32; 2]>,
    pub non_manifold_edges: Vec<[u32; 2]>,
}

impl ManifoldReport {
    /// Returns true if the mesh is topologically valid, watertight, and closed (genus-0 or genus-g).
    pub fn is_valid_solid(&self) -> bool {
        self.is_manifold
            && self.is_closed
            && (self.euler_characteristic == 2 || self.euler_characteristic <= 2)
    }
}

/// Structured topology error report for ReAct agent self-correction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyErrorMap {
    pub rule: String,
    pub severity: String,
    pub message: String,
    pub problematic_edges: Vec<[u32; 2]>,
}

/// Deterministic Geometric Verifier (Replacing probabilistic VLM vision checks).
#[derive(Debug, Default)]
pub struct GeometryVerifier;

impl GeometryVerifier {
    pub fn new() -> Self {
        Self
    }

    /// Verifies 2-manifoldness and watertightness using edge adjacency and the Euler-Poincaré formula:
    /// V - E + F = 2 (for genus-0 closed polyhedra)
    pub fn assert_manifold(&self, mesh: &MeshData) -> ManifoldReport {
        let mut edge_counts: HashMap<(u32, u32), usize> = HashMap::new();
        let mut used_vertices: HashSet<u32> = HashSet::new();

        for tri in &mesh.indices {
            let i0 = tri[0];
            let i1 = tri[1];
            let i2 = tri[2];

            used_vertices.insert(i0);
            used_vertices.insert(i1);
            used_vertices.insert(i2);

            let e0 = if i0 < i1 { (i0, i1) } else { (i1, i0) };
            let e1 = if i1 < i2 { (i1, i2) } else { (i2, i1) };
            let e2 = if i2 < i0 { (i2, i0) } else { (i0, i2) };

            *edge_counts.entry(e0).or_insert(0) += 1;
            *edge_counts.entry(e1).or_insert(0) += 1;
            *edge_counts.entry(e2).or_insert(0) += 1;
        }

        let mut boundary_edges = Vec::new();
        let mut non_manifold_edges = Vec::new();

        for (&(u, v), &count) in &edge_counts {
            if count == 1 {
                boundary_edges.push([u, v]);
            } else if count > 2 {
                non_manifold_edges.push([u, v]);
            }
        }

        let v = used_vertices.len();
        let e = edge_counts.len();
        let f = mesh.indices.len();
        let chi = (v as i32) - (e as i32) + (f as i32);

        let is_closed = boundary_edges.is_empty();
        let is_manifold = non_manifold_edges.is_empty();

        ManifoldReport {
            is_manifold,
            is_closed,
            euler_characteristic: chi,
            vertex_count: v,
            edge_count: e,
            face_count: f,
            boundary_edges,
            non_manifold_edges,
        }
    }

    /// Calculate the exact signed volume of a closed triangle mesh using the Divergence Theorem:
    /// Volume = 1/6 * sum( v0 . (v1 x v2) )
    pub fn calculate_signed_volume(&self, mesh: &MeshData) -> f32 {
        let mut total_vol = 0.0f32;

        for tri in &mesh.indices {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;

            if i0 < mesh.vertices.len() && i1 < mesh.vertices.len() && i2 < mesh.vertices.len() {
                let v0 = Vec3::from(mesh.vertices[i0]);
                let v1 = Vec3::from(mesh.vertices[i1]);
                let v2 = Vec3::from(mesh.vertices[i2]);

                // Signed volume of tetrahedron formed with origin
                total_vol += v0.dot(v1.cross(v2));
            }
        }

        (total_vol / 6.0).abs()
    }

    /// Assert that the calculated volume matches the expected volume within a tolerance.
    pub fn assert_volume(&self, mesh: &MeshData, expected_volume: f32, tolerance: f32) -> bool {
        let actual_volume = self.calculate_signed_volume(mesh);
        (actual_volume - expected_volume).abs() <= tolerance
    }

    /// Calculate the axis-aligned bounding box (AABB) (min, max) of the mesh.
    pub fn calculate_bounding_box(&self, mesh: &MeshData) -> ([f32; 3], [f32; 3]) {
        if mesh.vertices.is_empty() {
            return ([0.0, 0.0, 0.0], [0.0, 0.0, 0.0]);
        }

        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);

        for v in &mesh.vertices {
            let pos = Vec3::from(*v);
            min = min.min(pos);
            max = max.max(pos);
        }

        (min.into(), max.into())
    }

    /// Generate a structured topology error map if verification fails.
    pub fn generate_error_map(&self, report: &ManifoldReport) -> Option<TopologyErrorMap> {
        if !report.is_closed {
            Some(TopologyErrorMap {
                rule: "TOPOLOGY_OPEN_BOUNDARY".to_string(),
                severity: "ERROR".to_string(),
                message: format!(
                    "Mesh is not watertight: {} boundary edges detected (Euler chi = {}).",
                    report.boundary_edges.len(),
                    report.euler_characteristic
                ),
                problematic_edges: report.boundary_edges.clone(),
            })
        } else if !report.is_manifold {
            Some(TopologyErrorMap {
                rule: "TOPOLOGY_NON_MANIFOLD_EDGES".to_string(),
                severity: "ERROR".to_string(),
                message: format!(
                    "Mesh contains {} non-manifold edges shared by >2 faces.",
                    report.non_manifold_edges.len()
                ),
                problematic_edges: report.non_manifold_edges.clone(),
            })
        } else {
            None
        }
    }
}
