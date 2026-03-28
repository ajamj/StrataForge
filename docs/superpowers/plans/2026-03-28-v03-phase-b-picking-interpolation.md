# StrataForge v0.3 Phase B: Interactive Picking & RBF Surfaces Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Implement interactive horizon picking tools and geological surface modeling via RBF interpolation, including UI management and 3D visualization.

**Architecture:** Use a "Seed & Auto-Track" algorithm in `sf_compute` for event tracking. Implement a stateful `InterpretationManager` in `sf_app` to handle picks. Use RBF (Radial Basis Functions) for smooth surface generation and integrate with the `sf_render` crate for visualization.

**Tech Stack:** Rust, `nalgebra` (for RBF linear systems), `egui`, `wgpu`.

---

### Task 1: Implementing RBF Interpolation & Mesh Generation

**Files:**
- Create: `crates/sf_compute/src/interpolation.rs`
- Modify: `crates/sf_compute/src/lib.rs`

- [x] **Step 1: Implement basic RBF system solver**
- [x] **Step 2: Implement mesh generation from RBF**
- [x] **Step 3: Add unit tests with sparse points**
- [x] **Step 4: Commit**

---

### Task 2: Seed & Auto-Track Algorithm

**Files:**
- Create: `crates/sf_compute/src/tracking.rs`
- Modify: `crates/sf_compute/src/lib.rs`

- [x] **Step 1: Implement 1D peak/trough snap**
- [x] **Step 2: Implement 2D event tracker**
- [x] **Step 3: Commit**

---

### Task 3: Interpretation Management & UI Shell

**Files:**
- Create: `crates/sf_app/src/interpretation/mod.rs`
- Modify: `crates/sf_app/src/app.rs`

- [x] **Step 1: Implement Horizon and Pick state models**
- [x] **Step 2: Implement Interpretation Explorer (Tree-view)**
- [x] **Step 3: Implement Picking Toolbar**
- [x] **Step 4: Commit**

---

### Task 4: Viewport Interaction & Surface Visualization

**Files:**
- Modify: `crates/sf_app/src/widgets/viewport.rs`
- Modify: `crates/sf_render/src/lib.rs`

- [x] **Step 1: Add click-to-pick logic**
- [x] **x] Step 2: Visualize picks and generated surfaces**
- [x] **Step 3: Commit**
