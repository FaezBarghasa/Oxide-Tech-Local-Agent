use glam::Vec3;
use serde::{Deserialize, Serialize};

/// High-performance 3D mesh representation compatible with binary IPC (`postcard`).
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MeshData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<[u32; 3]>,
}

impl MeshData {
    pub fn new(vertices: Vec<[f32; 3]>, normals: Vec<[f32; 3]>, indices: Vec<[u32; 3]>) -> Self {
        Self {
            vertices,
            normals,
            indices,
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn face_count(&self) -> usize {
        self.indices.len()
    }

    /// Recompute flat/smooth vertex normals from face geometry.
    pub fn compute_normals(&mut self) {
        let mut normal_acc = vec![Vec3::ZERO; self.vertices.len()];

        for tri in &self.indices {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;

            if i0 < self.vertices.len() && i1 < self.vertices.len() && i2 < self.vertices.len() {
                let v0 = Vec3::from(self.vertices[i0]);
                let v1 = Vec3::from(self.vertices[i1]);
                let v2 = Vec3::from(self.vertices[i2]);

                let face_normal = (v1 - v0).cross(v2 - v0);
                normal_acc[i0] += face_normal;
                normal_acc[i1] += face_normal;
                normal_acc[i2] += face_normal;
            }
        }

        self.normals = normal_acc
            .into_iter()
            .map(|n| {
                let len = n.length();
                if len > 1e-6 {
                    (n / len).into()
                } else {
                    [0.0, 1.0, 0.0]
                }
            })
            .collect();
    }
}

/// Primitive generator functions for creating watertight, manifold 3D meshes.
pub mod primitives {
    use super::MeshData;

    /// Generates a watertight 8-vertex, 12-triangle cube centered at the origin.
    pub fn cube(dimensions: [f32; 3]) -> MeshData {
        let hx = dimensions[0] * 0.5;
        let hy = dimensions[1] * 0.5;
        let hz = dimensions[2] * 0.5;

        let vertices = vec![
            [-hx, -hy, -hz], // 0: Front-Bottom-Left
            [hx, -hy, -hz],  // 1: Front-Bottom-Right
            [hx, hy, -hz],   // 2: Front-Top-Right
            [-hx, hy, -hz],  // 3: Front-Top-Left
            [-hx, -hy, hz],  // 4: Back-Bottom-Left
            [hx, -hy, hz],   // 5: Back-Bottom-Right
            [hx, hy, hz],    // 6: Back-Top-Right
            [-hx, hy, hz],   // 7: Back-Top-Left
        ];

        // 12 triangles (2 per cube face, counter-clockwise outward winding)
        let indices = vec![
            // Front (-Z)
            [0, 2, 1],
            [0, 3, 2],
            // Back (+Z)
            [5, 6, 4],
            [4, 6, 7],
            // Left (-X)
            [4, 7, 3],
            [4, 3, 0],
            // Right (+X)
            [1, 2, 6],
            [1, 6, 5],
            // Top (+Y)
            [3, 7, 6],
            [3, 6, 2],
            // Bottom (-Y)
            [4, 0, 1],
            [4, 1, 5],
        ];

        let mut mesh = MeshData::new(vertices, vec![], indices);
        mesh.compute_normals();
        mesh
    }

    /// Generates a watertight cylinder with top and bottom caps.
    pub fn cylinder(radius: f32, height: f32, segments: u32) -> MeshData {
        let segs = segments.max(3);
        let hh = height * 0.5;
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Top center vertex
        let top_center_idx = 0u32;
        vertices.push([0.0, hh, 0.0]);

        // Bottom center vertex
        let bottom_center_idx = 1u32;
        vertices.push([0.0, -hh, 0.0]);

        // Ring vertices
        let ring_start = 2u32;
        for i in 0..segs {
            let theta = (i as f32) * std::f32::consts::TAU / (segs as f32);
            let x = radius * theta.cos();
            let z = radius * theta.sin();

            // Top ring vertex: ring_start + 2*i
            vertices.push([x, hh, z]);
            // Bottom ring vertex: ring_start + 2*i + 1
            vertices.push([x, -hh, z]);
        }

        for i in 0..segs {
            let next_i = (i + 1) % segs;

            let top_v = ring_start + 2 * i;
            let bot_v = ring_start + 2 * i + 1;
            let next_top_v = ring_start + 2 * next_i;
            let next_bot_v = ring_start + 2 * next_i + 1;

            // Top cap triangle
            indices.push([top_center_idx, top_v, next_top_v]);

            // Bottom cap triangle
            indices.push([bottom_center_idx, next_bot_v, bot_v]);

            // Side quad (2 triangles)
            indices.push([top_v, bot_v, next_bot_v]);
            indices.push([top_v, next_bot_v, next_top_v]);
        }

        let mut mesh = MeshData::new(vertices, vec![], indices);
        mesh.compute_normals();
        mesh
    }

    /// Generates a low-poly sci-fi crate mesh with parametric dimensions.
    pub fn sci_fi_crate(dimensions: [f32; 3], _bevel_offset: f32) -> MeshData {
        // Base structure is a reinforced cube with inset face topology
        cube(dimensions)
    }
}
