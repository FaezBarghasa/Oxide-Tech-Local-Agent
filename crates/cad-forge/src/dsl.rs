use glam::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Primitive {
    Box {
        center: Vec3,
        size: Vec3,
    },
    Cylinder {
        center: Vec3,
        radius: f32,
        height: f32,
    },
    Sphere {
        center: Vec3,
        radius: f32,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BoolOp {
    Union(usize, usize),
    Subtract(usize, usize),
    Intersect(usize, usize),
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CadScript {
    pub primitives: Vec<Primitive>,
    pub operations: Vec<BoolOp>,
}

#[derive(Debug, Default)]
pub struct CadBuilder {
    script: CadScript,
}

impl CadBuilder {
    pub fn new() -> Self {
        Self {
            script: CadScript::default(),
        }
    }

    pub fn add_box(mut self, center: Vec3, size: Vec3) -> (Self, usize) {
        let idx = self.script.primitives.len();
        self.script.primitives.push(Primitive::Box { center, size });
        (self, idx)
    }

    pub fn add_cylinder(mut self, center: Vec3, radius: f32, height: f32) -> (Self, usize) {
        let idx = self.script.primitives.len();
        self.script.primitives.push(Primitive::Cylinder { center, radius, height });
        (self, idx)
    }

    pub fn add_sphere(mut self, center: Vec3, radius: f32) -> (Self, usize) {
        let idx = self.script.primitives.len();
        self.script.primitives.push(Primitive::Sphere { center, radius });
        (self, idx)
    }

    pub fn subtract(mut self, target_idx: usize, tool_idx: usize) -> Self {
        self.script.operations.push(BoolOp::Subtract(target_idx, tool_idx));
        self
    }

    pub fn union(mut self, a: usize, b: usize) -> Self {
        self.script.operations.push(BoolOp::Union(a, b));
        self
    }

    pub fn intersect(mut self, a: usize, b: usize) -> Self {
        self.script.operations.push(BoolOp::Intersect(a, b));
        self
    }

    pub fn build(self) -> CadScript {
        self.script
    }
}
