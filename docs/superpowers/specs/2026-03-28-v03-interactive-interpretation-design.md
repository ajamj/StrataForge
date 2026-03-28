# StrataForge v0.3: Interactive Interpretation Workstation Design

**Date:** 2026-03-28  
**Phase:** v0.3 - SEG-Y + Slice + Picks  
**Status:** Approved by User

## 1. Overview
StrataForge v0.3 evolves the platform from a data viewer into an active interpretation workstation. This phase focuses on high-performance access to large seismic volumes and interactive tools for geological interpretation (picking) and surface modeling.

## 2. Key Components

### 2.1. High-Performance Seismic I/O (`sf_io`)
- **Memory-Mapped Access:** Implement `memmap2` for SEG-Y files to enable near-instant extraction of arbitrary slices without loading the entire volume into RAM.
- **Trace Indexing:** Efficiently map SEG-Y traces to Inline/Crossline/Z coordinates for fast random access.

### 2.2. Interactive Picking Engine (`sf_app` + `sf_compute`)
- **Seed & Auto-Track:** 
    - User clicks a "seed" point on a seismic event (peak/trough).
    - Algorithm (in `sf_compute`) automatically follows the event across the 2D slice based on amplitude correlation or phase.
- **Pick Management:** A dedicated state manager in the Rust app to store, edit, and delete sparse 2D/3D picks.

### 2.3. Smooth Surface Generation (`sf_compute`)
- **RBF Interpolation:** Implement Radial Basis Function (RBF) or similar interpolation to turn sparse picks into continuous, smooth 3D geological horizons.
- **Comparison with Delaunay:** While Delaunay (v0.1) is used for literal triangulation, the RBF approach will provide the "smooth" geological look required for interpretation.

### 2.4. Desktop UI Enhancements (`sf_app`)
- **Picking Toolbar:** Toggles for "Seed & Auto-Track" mode, pick size, and color.
- **Interpretation Explorer:** Tree-view to manage multiple horizons, each with its own set of picks and generated surfaces.

## 3. Data Flow
1. **Mmap:** Rust app maps a SEG-Y volume into memory.
2. **Slice:** Slices are extracted instantly as the user scrolls.
3. Seed: User clicks a point; sf_compute tracks the event and returns a sequence of picks.
4. **Model:** User triggers "Generate Surface"; picks are sent to the RBF engine.
5. **Visualize:** The smooth 3D horizon is rendered in the `wgpu` viewport.

## 4. Success Criteria
- [ ] Memory-mapped SEG-Y access allows smooth scrolling through a 1GB+ volume.
- [ ] Seed & Auto-Track correctly follows a clear reflector across a 2D slice.
- [ ] Turning picks into a smooth 3D surface using RBF interpolation.
- [ ] Support for multiple interpretation horizons in the Project Data panel.
