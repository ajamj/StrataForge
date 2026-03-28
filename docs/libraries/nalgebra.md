# nalgebra

`nalgebra` is the primary linear algebra library for StrataForge. It provides the mathematical foundation for geometric computations, 3D transformations, and surface interpolation.

## Overview

In StrataForge, we use `nalgebra` for performance-critical vector and matrix operations. While currently utilized in `sf_render` for camera and scene transformations, its most critical upcoming use (in v0.3) is for solving the large linear systems required for Radial Basis Function (RBF) surface interpolation.

### Why nalgebra?
- **Performance:** Excellent optimization for both small (fixed-size) and large (dynamically-sized) matrices.
- **Type Safety:** Strongly-typed vectors and matrices prevent common errors like multiplying incompatible shapes.
- **Rich Feature Set:** Built-in support for transformations (translations, rotations), decompositions (SVD, LU, QR), and linear solvers.

## Usage in Project

`nalgebra` is used across the codebase where geometric or mathematical computation is required.

### Scene Transformations
Located in the `sf_render` crate (planned integration):

```rust
use nalgebra::{Matrix4, Vector3, Point3};

// Create a transformation matrix for a mesh
let translation = Matrix4::new_translation(&Vector3::new(100.0, 0.0, 0.0));
let rotation = Matrix4::from_euler_angles(0.0, 3.14 / 2.0, 0.0);
let model_matrix = translation * rotation;
```

### RBF Interpolation (Planned v0.3)
As defined in the `docs/superpowers/plans/2026-03-28-v03-phase-b-picking-interpolation.md`:

```rust
use nalgebra::{DMatrix, DVector};

// Solving a linear system A * x = B to find RBF weights
pub fn solve_rbf(points: &[[f64; 2]], values: &[f64]) -> DVector<f64> {
    let n = points.len();
    let mut a = DMatrix::zeros(n, n);
    let mut b = DVector::from_column_slice(values);

    // Build the RBF matrix (e.g., using Gaussian kernel)
    for i in 0..n {
        for j in 0..n {
            a[(i, j)] = gaussian_rbf(points[i], points[j]);
        }
    }

    // Solve for weights
    a.full_piv_lu().solve(&b).expect("Linear system must be solvable")
}
```

## Key Concepts

- **Matrix (`DMatrix` / `SMatrix`):** The primary data structure for mathematical grids. `DMatrix` is for dynamic sizes, `SMatrix` is for small, fixed sizes.
- **Vector (`DVector` / `SVector`):** Column vectors.
- **Point (`Point3`):** Represents a position in space (conceptually different from a displacement vector).
- **Transformation:** High-level abstractions for moving, rotating, and scaling objects in 3D.
- **Decomposition:** Methods like LU, QR, or SVD used to solve linear equations and find eigenvalues.

## Resources

- [Official nalgebra Website](https://nalgebra.org/)
- [nalgebra Documentation (docs.rs)](https://docs.rs/nalgebra)
- [Linear Algebra in Rust (Guide)](https://nalgebra.org/docs/user_guide/getting_started)
