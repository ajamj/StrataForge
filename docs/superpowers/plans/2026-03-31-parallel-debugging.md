# Parallel Debugging Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Resolve all workspace compilation errors and high-priority warnings using a coordinated team of AI agents.

**Architecture:** Parallel execution of four specialized agents, each targeting a specific subsystem.

**Tech Stack:** Rust (cargo), ndarray, egui, candle, Python (ruff).

---

### Task 1: Codex - Fix ndarray usage in `seisly_fwi`

**Files:**
- Modify: `crates/seisly_fwi/src/gradient.rs`
- Test: `cargo check -p seisly_fwi`

- [ ] **Step 1: Execute Codex fix**
Run: `codex exec "In crates/seisly_fwi/src/gradient.rs, replace all occurrences of Array2::from_vec with Array2::from."`

- [ ] **Step 2: Verify fix**
Run: `cargo check -p seisly_fwi`

- [ ] **Step 3: Commit**

### Task 2: Gemini - Resolve Candle dependencies in `seisly_tracking`

**Files:**
- Modify: `crates/seisly_tracking/Cargo.toml`
- Modify: `crates/seisly_tracking/src/multi_horizon.rs`
- Modify: `crates/seisly_tracking/src/fault_guided.rs`
- Test: `cargo check -p seisly_tracking`

- [ ] **Step 1: Execute Gemini fix**
Run: `gemini -p "Add candle-core and candle-nn to crates/seisly_tracking/Cargo.toml. Then fix unresolved imports in src/multi_horizon.rs and src/fault_guided.rs."`

- [ ] **Step 2: Verify fix**
Run: `cargo check -p seisly_tracking`

- [ ] **Step 3: Commit**

### Task 3: OpenCode - Update egui deprecations in `seisly_app`

**Files:**
- Modify: `crates/seisly_app/src/widgets/viewport.rs`
- Test: `cargo check -p seisly_app`

- [ ] **Step 1: Execute OpenCode fix**
Run: `opencode run "In crates/seisly_app/src/widgets/viewport.rs, rename response.drag_released() to response.drag_stopped()."`

- [ ] **Step 2: Verify fix**
Run: `cargo check -p seisly_app`

- [ ] **Step 3: Commit**

### Task 4: Lead - Cleanup `seisly_ml_deep` and Workspace Sync

**Files:**
- Modify: `crates/seisly_ml_deep/src/training_dl.rs`
- Test: `cargo check --workspace`

- [ ] **Step 1: Lead fix**
Remove unnecessary parentheses in `training_dl.rs:108` and address unused field `config`.

- [ ] **Step 2: Final Workspace Validation**
Run: `cargo check --workspace`

- [ ] **Step 3: Commit**
