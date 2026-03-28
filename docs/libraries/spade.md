# spade

`spade` is the Delaunay triangulation library used in StrataForge for generating surface meshes from sparse point data.

## Overview

In StrataForge, we use `spade` to build 3D surfaces from scattered data points (e.g., horizon picks, top/base markers). It provides a robust, high-performance implementation of 2D Delaunay triangulation, which we utilize to generate triangular meshes from 3D points by projecting them onto the XY plane.

### Why spade?
- **Robustness:** Handles nearly-collinear points and degenerate cases gracefully.
- **Performance:** Efficient bulk loading for large datasets.
- **Flexibility:** Allows attaching custom data (like original indices) to the triangulation vertices.

## Usage in Project

The `sf_compute` crate uses `spade` for surface building.

### Triangulating Points
Located in `crates/sf_compute/src/triangulation.rs`:

```rust
use spade::{DelaunayTriangulation, Point2, HasPosition, Triangulation};

// Define a struct to hold our point and its original index
struct IndexedPoint {
    point: Point2<f64>,
    index: usize,
}

impl HasPosition for IndexedPoint {
    type Scalar = f64;
    fn position(&self) -> Point2<f64> {
        self.point
    }
}

pub fn triangulate_points(points: &[[f32; 3]]) -> Result<Mesh, TriangulationError> {
    // Project 3D points to 2D for triangulation
    let points_2d: Vec<IndexedPoint> = points.iter()
        .enumerate()
        .map(|(i, p)| IndexedPoint {
            point: Point2::new(p[0] as f64, p[1] as f64),
            index: i,
        })
        .collect();
    
    // Bulk load into the triangulation
    let triangulation = DelaunayTriangulation::<IndexedPoint>::bulk_load(points_2d)?;
    
    // Extract triangle indices from the resulting faces
    for face in triangulation.inner_faces() {
        let vertices = face.vertices();
        indices.push(vertices[0].data().index as u32);
        indices.push(vertices[1].data().index as u32);
        indices.push(vertices[2].data().index as u32);
    }
    // ...
}
```

## Key Concepts

- **Delaunay Triangulation:** A triangulation of points such that no point is inside the circumcircle of any triangle. This creates "well-behaved" triangles.
- **Bulk Loading:** A technique to create a triangulation from a large set of points much faster than inserting them one by one.
- **Face / Edge / Vertex:** The primary primitives in a triangulation. `spade` provides a rich API for traversing these elements.
- **HasPosition:** A trait that you must implement for your custom data types to allow `spade` to know their coordinates.

## Resources

- [spade Documentation (docs.rs)](https://docs.rs/spade)
- [Wikipedia: Delaunay triangulation](https://en.wikipedia.org/wiki/Delaunay_triangulation)
