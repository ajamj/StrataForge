# StrataForge v0.4 Phase A: Structural Foundations Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Establish the foundational infrastructure for structural interpretation, including data persistence for faults and a baseline functional 3D renderer.

**Architecture:** Extend the SQLite schema to support Fault entities and Sticks. Transition the current dummy `PaintCallback` in `sf_app` into a basic `wgpu` render pass that can display primitive geometry.

**Tech Stack:** Rust, SQLite, wgpu.

---

### Task 1: Persistent Structural Schema

**Files:**
- Modify: `schemas/sqlite/0001_init.sql`
- Modify: `crates/sf_storage/src/sqlite/mod.rs`

- [ ] **Step 1: Add Faults and Sticks tables to the schema**

```sql
CREATE TABLE faults (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL,
    name TEXT NOT NULL,
    color_r REAL NOT NULL,
    color_g REAL NOT NULL,
    color_b REAL NOT NULL,
    is_visible INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY(project_id) REFERENCES projects(id)
);

CREATE TABLE fault_sticks (
    id TEXT PRIMARY KEY,
    fault_id TEXT NOT NULL,
    name TEXT NOT NULL,
    picks_blob BLOB NOT NULL, -- Serialized Vec<[f32; 3]>
    FOREIGN KEY(fault_id) REFERENCES faults(id)
);
```

- [ ] **Step 2: Update SQLite integration tests to verify new tables**

- [ ] **Step 3: Commit**

```bash
git add schemas/sqlite/0001_init.sql crates/sf_storage/src/sqlite/mod.rs
git commit -m "feat: add faults and sticks to SQLite schema"
```

---

### Task 2: Baseline 3D Renderer (wgpu)

**Files:**
- Modify: `crates/sf_render/src/renderer.rs`
- Modify: `crates/sf_app/src/widgets/viewport.rs`

- [ ] **Step 1: Implement a basic 3D render pass in sf_render**
Replace the dummy callback with a functional `wgpu` pass that clears the screen and can render a test triangle.

- [ ] **Step 2: Connect the render pass to the ViewportWidget**

- [ ] **Step 3: Run the app and verify visual output**

- [ ] **Step 4: Commit**

```bash
git add crates/sf_render/src/renderer.rs crates/sf_app/src/widgets/viewport.rs
git commit -m "feat: transition to a functional 3D wgpu renderer"
```

---

### Task 3: Structural interpretation Manager

**Files:**
- Modify: `crates/sf_app/src/interpretation/mod.rs`

- [ ] **Step 1: Implement Fault and FaultStick domain models**

```rust
pub struct FaultStick {
    pub id: uuid::Uuid,
    pub name: String,
    pub picks: Vec<[f32; 3]>,
}

pub struct Fault {
    pub id: uuid::Uuid,
    pub name: String,
    pub color: [f32; 3],
    pub sticks: Vec<FaultStick>,
    pub is_visible: bool,
}
```

- [ ] **Step 2: Update InterpretationState to manage Faults**

- [ ] **Step 3: Commit**

```bash
git add crates/sf_app/src/interpretation/mod.rs
git commit -m "feat: implement structural interpretation domain models"
```
