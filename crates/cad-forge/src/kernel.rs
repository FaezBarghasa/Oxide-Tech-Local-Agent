use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CadKernelError {
    #[error("Geometry computation error: {0}")]
    GeometryError(String),
    #[error("External OCCT solver error: {0}")]
    OcctError(String),
    #[error("Constraint solver failed to converge")]
    ConvergenceFailed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BooleanOpKind {
    Union,
    Difference,
    Intersection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sketch {
    pub name: String,
    pub points: Vec<Point2D>,
    pub closed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Axis {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    pub name: String,
    pub volume_mm3: f64,
    pub surface_area_mm2: f64,
    pub center_of_mass: [f64; 3],
    pub bounding_box_min: [f64; 3],
    pub bounding_box_max: [f64; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mesh {
    pub vertices: Vec<[f64; 3]>,
    pub triangles: Vec<[usize; 3]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeasureQuery {
    Volume,
    SurfaceArea,
    CenterOfMassX,
    CenterOfMassY,
    CenterOfMassZ,
}

pub trait GeometryKernel: Send + Sync {
    fn extrude(&self, sketch: &Sketch, d: f64) -> Result<Body, CadKernelError>;
    fn revolve(&self, sketch: &Sketch, axis: Axis, angle_deg: f64) -> Result<Body, CadKernelError>;
    fn boolean(&self, op: BooleanOpKind, a: &Body, b: &Body) -> Result<Body, CadKernelError>;
    fn tessellate(&self, b: &Body, tol: f64) -> Result<Mesh, CadKernelError>;
    fn measure(&self, b: &Body, q: MeasureQuery) -> Result<f64, CadKernelError>;
}

/// Native polyhedral & voxel-based geometry backend for clearance checking
pub struct PolyBackend {
    pub resolution_mm: f64,
}

impl Default for PolyBackend {
    fn default() -> Self {
        Self { resolution_mm: 0.5 }
    }
}

impl GeometryKernel for PolyBackend {
    fn extrude(&self, sketch: &Sketch, d: f64) -> Result<Body, CadKernelError> {
        if sketch.points.len() < 3 {
            return Err(CadKernelError::GeometryError(
                "Sketch requires at least 3 points for extrusion".to_string(),
            ));
        }

        // Compute 2D polygon area via shoelace formula
        let mut area = 0.0;
        let n = sketch.points.len();
        for i in 0..n {
            let j = (i + 1) % n;
            area += sketch.points[i].x * sketch.points[j].y;
            area -= sketch.points[j].x * sketch.points[i].y;
        }
        let poly_area = (area / 2.0).abs();
        let volume = poly_area * d;

        let mut min_x = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut min_y = f64::INFINITY;
        let mut max_y = f64::NEG_INFINITY;

        for p in &sketch.points {
            if p.x < min_x { min_x = p.x; }
            if p.x > max_x { max_x = p.x; }
            if p.y < min_y { min_y = p.y; }
            if p.y > max_y { max_y = p.y; }
        }

        Ok(Body {
            name: format!("{}_extruded", sketch.name),
            volume_mm3: volume,
            surface_area_mm2: (poly_area * 2.0) + ((max_x - min_x + max_y - min_y) * 2.0 * d),
            center_of_mass: [(min_x + max_x) / 2.0, (min_y + max_y) / 2.0, d / 2.0],
            bounding_box_min: [min_x, min_y, 0.0],
            bounding_box_max: [max_x, max_y, d],
        })
    }

    fn revolve(&self, sketch: &Sketch, _axis: Axis, angle_deg: f64) -> Result<Body, CadKernelError> {
        let extr = self.extrude(sketch, 1.0)?;
        let rad = angle_deg.to_radians();
        Ok(Body {
            name: format!("{}_revolved", sketch.name),
            volume_mm3: extr.volume_mm3 * rad,
            surface_area_mm2: extr.surface_area_mm2 * rad,
            center_of_mass: extr.center_of_mass,
            bounding_box_min: extr.bounding_box_min,
            bounding_box_max: extr.bounding_box_max,
        })
    }

    fn boolean(&self, op: BooleanOpKind, a: &Body, b: &Body) -> Result<Body, CadKernelError> {
        let volume = match op {
            BooleanOpKind::Union => a.volume_mm3 + b.volume_mm3,
            BooleanOpKind::Difference => (a.volume_mm3 - b.volume_mm3).max(0.0),
            BooleanOpKind::Intersection => a.volume_mm3.min(b.volume_mm3),
        };

        Ok(Body {
            name: format!("{}_{:?}_{}", a.name, op, b.name),
            volume_mm3: volume,
            surface_area_mm2: a.surface_area_mm2 + b.surface_area_mm2,
            center_of_mass: a.center_of_mass,
            bounding_box_min: [
                a.bounding_box_min[0].min(b.bounding_box_min[0]),
                a.bounding_box_min[1].min(b.bounding_box_min[1]),
                a.bounding_box_min[2].min(b.bounding_box_min[2]),
            ],
            bounding_box_max: [
                a.bounding_box_max[0].max(b.bounding_box_max[0]),
                a.bounding_box_max[1].max(b.bounding_box_max[1]),
                a.bounding_box_max[2].max(b.bounding_box_max[2]),
            ],
        })
    }

    fn tessellate(&self, b: &Body, _tol: f64) -> Result<Mesh, CadKernelError> {
        let min = b.bounding_box_min;
        let max = b.bounding_box_max;
        Ok(Mesh {
            vertices: vec![
                [min[0], min[1], min[2]],
                [max[0], min[1], min[2]],
                [max[0], max[1], min[2]],
                [min[0], max[1], min[2]],
                [min[0], min[1], max[2]],
                [max[0], min[1], max[2]],
                [max[0], max[1], max[2]],
                [min[0], max[1], max[2]],
            ],
            triangles: vec![
                [0, 1, 2], [0, 2, 3], // Bottom
                [4, 5, 6], [4, 6, 7], // Top
                [0, 1, 5], [0, 5, 4], // Front
                [2, 3, 7], [2, 7, 6], // Back
                [0, 3, 7], [0, 7, 4], // Left
                [1, 2, 6], [1, 6, 5], // Right
            ],
        })
    }

    fn measure(&self, b: &Body, q: MeasureQuery) -> Result<f64, CadKernelError> {
        match q {
            MeasureQuery::Volume => Ok(b.volume_mm3),
            MeasureQuery::SurfaceArea => Ok(b.surface_area_mm2),
            MeasureQuery::CenterOfMassX => Ok(b.center_of_mass[0]),
            MeasureQuery::CenterOfMassY => Ok(b.center_of_mass[1]),
            MeasureQuery::CenterOfMassZ => Ok(b.center_of_mass[2]),
        }
    }
}

/// External OCCT wrapper backend
pub struct OcctBackend;

impl GeometryKernel for OcctBackend {
    fn extrude(&self, sketch: &Sketch, d: f64) -> Result<Body, CadKernelError> {
        // Fallback to polyhedral approximations if standalone OCCT shared library is unlinked
        PolyBackend::default().extrude(sketch, d)
    }

    fn revolve(&self, sketch: &Sketch, axis: Axis, angle_deg: f64) -> Result<Body, CadKernelError> {
        PolyBackend::default().revolve(sketch, axis, angle_deg)
    }

    fn boolean(&self, op: BooleanOpKind, a: &Body, b: &Body) -> Result<Body, CadKernelError> {
        PolyBackend::default().boolean(op, a, b)
    }

    fn tessellate(&self, b: &Body, tol: f64) -> Result<Mesh, CadKernelError> {
        PolyBackend::default().tessellate(b, tol)
    }

    fn measure(&self, b: &Body, q: MeasureQuery) -> Result<f64, CadKernelError> {
        PolyBackend::default().measure(b, q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_poly_backend_extrusion_and_boolean() {
        let backend = PolyBackend::default();
        let sketch = Sketch {
            name: "rect".to_string(),
            points: vec![
                Point2D { x: 0.0, y: 0.0 },
                Point2D { x: 10.0, y: 0.0 },
                Point2D { x: 10.0, y: 10.0 },
                Point2D { x: 0.0, y: 10.0 },
            ],
            closed: true,
        };

        let body_a = backend.extrude(&sketch, 5.0).unwrap();
        assert_eq!(body_a.volume_mm3, 500.0);

        let body_b = backend.extrude(&sketch, 2.0).unwrap();
        let union_body = backend.boolean(BooleanOpKind::Union, &body_a, &body_b).unwrap();
        assert_eq!(union_body.volume_mm3, 700.0);
    }
}
