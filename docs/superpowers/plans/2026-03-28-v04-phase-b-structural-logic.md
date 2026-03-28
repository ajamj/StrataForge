# StrataForge v0.4 Phase B: Structural Logic & Visuals Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the core structural interpretation logic, including 3D RBF fault modeling and semi-transparent surface rendering.

**Architecture:** Extend the `RbfInterpolator` to support 3D point cloud inputs. Update `sf_render` to support alpha blending and primitive depth sorting for transparent surfaces.

**Tech Stack:** Rust, nalgebra, wgpu.

---

### Task 1: 3D RBF Interpolation

**Files:**
- Modify: `crates/sf_compute/src/interpolation.rs`

- [ ] **Step 1: Adapt RbfInterpolator for 3D inputs**
Implement logic to handle high-angle planes (e.g., by rotating coordinates or using true 3D RBF kernels).

- [ ] **Step 2: Add unit tests for vertical plane interpretation**

- [ ] **Step 3: Commit**

```bash
git add crates/sf_compute/src/interpolation.rs
git commit -m "feat: enhance RBF engine for 3D fault modeling"
```

---

### Task 2: Transparent Surface Rendering

**Files:**
- Modify: `crates/sf_render/src/mesh.rs`
- Modify: `crates/sf_render/src/renderer.rs`

- [ ] **Step 1: Configure Alpha Blending in wgpu pipeline**

```rust
// In RenderPipelineDescriptor fragment state:
blend: Some(wgpu::BlendState::ALPHA_BLENDING),
```

- [ ] **Step 2: Implement basic depth-sorting for meshes**
Add a mechanism to sort renderable objects based on their distance from the camera before drawing.

- [ ] **Step 3: Commit**

```bash
git add crates/sf_render/src/
git commit -m "feat: implement semi-transparent mesh rendering with depth sorting"
```

---

### Task 3: Interactive Fault Picking

**Files:**
- Modify: `crates/sf_app/src/widgets/viewport.rs`
- Modify: `crates/sf_app/src/app.rs`

- [ ] **Step 1: Implement "Sketch Mode" for Fault Sticks**
Allow users to click-and-drag to create sticks on seismic sections.

- [ ] **Step 2: Connect picking events to real-time RBF updates**

- [ ] **Step 3: Final end-to-end verification**

- [ ] **Step 4: Commit**

```bash
git add crates/sf_app/src/
git commit -m "feat: implement interactive fault picking and real-time modeling"
```
