//! Clipping engine for mesh-surface and mesh-plane intersections.

use nalgebra::{Point3, Vector3};
use sf_core::domain::surface::{Mesh, Surface};

/// Geometric plane defined by a point and a normal vector.
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    pub origin: Point3<f32>,
    pub normal: Vector3<f32>,
}

impl Plane {
    pub fn new(origin: Point3<f32>, normal: Vector3<f32>) -> Self {
        Self { origin, normal: normal.normalize() }
    }

    /// Distance from point to plane (signed).
    pub fn distance_to_point(&self, p: Point3<f32>) -> f32 {
        let v = p - self.origin;
        v.dot(&self.normal)
    }
}

/// Intersection result between a triangle and a plane.
#[derive(Debug)]
pub enum TriangleIntersection {
    /// Triangle is entirely on one side of the plane.
    None,
    /// Triangle is entirely on the plane (coincident).
    Coincident,
    /// Triangle is split into a segment.
    Segment(Point3<f32>, Point3<f32>),
}

/// Computes the intersection of a triangle (defined by 3 points) and a plane.
pub fn intersect_triangle_plane(v0: Point3<f32>, v1: Point3<f32>, v2: Point3<f32>, plane: &Plane) -> TriangleIntersection {
    let d0 = plane.distance_to_point(v0);
    let d1 = plane.distance_to_point(v1);
    let d2 = plane.distance_to_point(v2);

    let epsilon = 1e-6;

    let points = [(v0, d0), (v1, d1), (v2, d2)];
    
    // Check if all points are on one side
    if (d0 > epsilon && d1 > epsilon && d2 > epsilon) || (d0 < -epsilon && d1 < -epsilon && d2 < -epsilon) {
        return TriangleIntersection::None;
    }

    // Check if all points are on the plane
    if d0.abs() < epsilon && d1.abs() < epsilon && d2.abs() < epsilon {
        return TriangleIntersection::Coincident;
    }

    // Find crossing edges
    let mut intersection_points = Vec::with_capacity(2);

    for i in 0..3 {
        let (p1, dist1) = points[i];
        let (p2, dist2) = points[(i + 1) % 3];

        if (dist1 > epsilon && dist2 < -epsilon) || (dist1 < -epsilon && dist2 > epsilon) {
            // Edge crosses the plane
            let t = dist1 / (dist1 - dist2);
            let p = p1 + (p2 - p1) * t;
            intersection_points.push(p);
        } else if dist1.abs() < epsilon {
            // Point is exactly on the plane
            intersection_points.push(p1);
        }
    }

    // Deduplicate points that are very close to each other
    intersection_points.dedup_by(|a, b| (*a - *b).norm_squared() < epsilon * epsilon);

    if intersection_points.len() == 2 {
        TriangleIntersection::Segment(intersection_points[0], intersection_points[1])
    } else {
        TriangleIntersection::None
    }
}

/// Computes the intersection lines between a mesh and a plane.
pub fn intersect_mesh_plane(mesh: &Mesh, plane: &Plane) -> Vec<Vec<[f32; 3]>> {
    let mut segments = Vec::new();

    for chunk in mesh.indices.chunks_exact(3) {
        let v0 = Point3::from(mesh.vertices[chunk[0] as usize]);
        let v1 = Point3::from(mesh.vertices[chunk[1] as usize]);
        let v2 = Point3::from(mesh.vertices[chunk[2] as usize]);

        match intersect_triangle_plane(v0, v1, v2, plane) {
            TriangleIntersection::Segment(p1, p2) => {
                segments.push((p1, p2));
            }
            _ => {}
        }
    }

    // TODO: Connect segments into polylines. For now, just return individual segments.
    segments.into_iter().map(|(p1, p2)| vec![p1.into(), p2.into()]).collect()
}

/// Updates the intersection lines for a surface based on a plane intersection.
pub fn update_surface_intersections(surface: &mut Surface, plane: &Plane) {
    surface.intersection_lines.clear();
    for mesh in &surface.meshes {
        let mut segments = intersect_mesh_plane(mesh, plane);
        surface.intersection_lines.append(&mut segments);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_plane_intersection() {
        let plane = Plane::new(Point3::origin(), Vector3::z());
        
        // Triangle crossing Z=0
        let v0 = Point3::new(0.0, 0.0, -1.0);
        let v1 = Point3::new(1.0, 0.0, 1.0);
        let v2 = Point3::new(0.0, 1.0, 1.0);

        let result = intersect_triangle_plane(v0, v1, v2, &plane);
        if let TriangleIntersection::Segment(p1, p2) = result {
            // Expected intersection points are on Z=0
            assert_eq!(p1.z, 0.0);
            assert_eq!(p2.z, 0.0);
        } else {
            panic!("Expected Segment intersection, got {:?}", result);
        }
    }

    #[test]
    fn test_vertical_intersection() {
        let plane = Plane::new(Point3::origin(), Vector3::x()); // Vertical plane X=0
        
        // Triangle crossing X=0
        let v0 = Point3::new(-1.0, 0.0, 0.0);
        let v1 = Point3::new(1.0, 1.0, 0.0);
        let v2 = Point3::new(1.0, -1.0, 0.0);

        let result = intersect_triangle_plane(v0, v1, v2, &plane);
        if let TriangleIntersection::Segment(p1, p2) = result {
            assert_eq!(p1.x, 0.0);
            assert_eq!(p2.x, 0.0);
        } else {
            panic!("Expected Segment intersection, got {:?}", result);
        }
    }
}
