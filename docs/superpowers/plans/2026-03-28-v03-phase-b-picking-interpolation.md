# StrataForge v0.3 Phase B: Interactive Picking & RBF Surfaces Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement interactive horizon picking tools and geological surface modeling via RBF interpolation, including UI management and 3D visualization.

**Architecture:** Use a "Seed & Auto-Track" algorithm in `sf_compute` for event tracking. Implement a stateful `InterpretationManager` in `sf_app` to handle picks. Use RBF (Radial Basis Functions) for smooth surface generation and integrate with the `sf_render` crate for visualization.

**Tech Stack:** Rust, `nalgebra` (for RBF linear systems), `egui`, `wgpu`.

---

### Task 1: Implementing RBF Interpolation & Mesh Generation

**Files:**
- Create: `crates/sf_compute/src/interpolation.rs`
- Modify: `crates/sf_compute/src/lib.rs`

- [ ] **Step 1: Implement basic RBF system solver**
Given a set of (x, y, z) picks, solve the linear system for RBF weights using `nalgebra`.

- [ ] **Step 2: Implement mesh generation from RBF**
Evaluate the RBF on a regular grid and generate a `sf_core::domain::surface::Mesh` using the evaluated heights.

- [ ] **Step 3: Add unit tests with sparse points**

- [ ] **Step 4: Commit**

```bash
git add crates/sf_compute/src/interpolation.rs crates/sf_compute/src/lib.rs
git commit -m "feat: implement RBF interpolation and mesh generation"
```

---

### Task 2: Seed & Auto-Track Algorithm

**Files:**
- Create: `crates/sf_compute/src/tracking.rs`
- Modify: `crates/sf_compute/src/lib.rs`

- [ ] **Step 1: Implement 1D peak/trough snap**
Logic to refine a click to the nearest local extrema in a trace.

- [ ] **Step 2: Implement 2D event tracker**
Follow the reflector across adjacent traces based on similarity.

- [ ] **Step 3: Commit**

```bash
git add crates/sf_compute/src/tracking.rs crates/sf_compute/src/lib.rs
git commit -m "feat: implement Seed & Auto-Track algorithm"
```

---

### Task 3: Interpretation Management & UI Shell

**Files:**
- Create: `crates/sf_app/src/interpretation/mod.rs`
- Modify: `crates/sf_app/src/app.rs`

- [ ] **Step 1: Implement Horizon and Pick state models**

- [ ] **Step 2: Implement Interpretation Explorer (Tree-view)**
Add UI in the left panel to list horizons and their picks.

- [ ] **Step 3: Implement Picking Toolbar**
Add a toolbar in the `sf_app` to toggle picking modes.

- [ ] **Step 4: Commit**

```bash
git add crates/sf_app/src/interpretation/ crates/sf_app/src/app.rs
git commit -m "ui: implement interpretation management and toolbar"
```

---

### Task 4: Viewport Interaction & Surface Visualization

**Files:**
- Modify: `crates/sf_app/src/widgets/viewport.rs`
- Modify: `crates/sf_render/src/lib.rs`

- [ ] **Step 1: Add click-to-pick logic**
Handle mouse clicks, project to 3D space, and trigger the auto-tracker.

- [ ] **Step 2: Visualize picks and generated surfaces**
Render picks as points and the generated RBF horizon as a `sf_render` mesh in the viewport.

- [ ] **Step 3: Commit**

```bash
git add crates/sf_app/src/widgets/viewport.rs crates/sf_render/src/lib.rs
git commit -m "feat: add interactive picking and surface visualization to viewport"
```
