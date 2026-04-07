# SEGY Import & UI Polish Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix the non-responsive SEG-Y import by adding a metadata scan wizard, background loading with a visual spinner, a session-only log console, and fixing broken undo/redo icons with new SVGs.

**Architecture:** Use a state-driven approach for the import workflow (Scanning -> Scanned -> Importing). Implement a global thread-safe log buffer for the console and a central overlay for the loading animation.

**Tech Stack:** Rust, egui, eframe, egui_extras (SVG), tokio (background tasks).

---

### Task 1: SVG Icons & Style Updates

**Files:**
- Create: `crates/seisly_app/assets/icons/undo.svg`
- Create: `crates/seisly_app/assets/icons/redo.svg`
- Modify: `crates/seisly_app/src/ui/style.rs`

- [ ] **Step 1: Create undo.svg**
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
  <path d="M4 12V6M4 6H10M4 6C8 4 14 4 18 8C22 12 22 18 18 21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
</svg>
```

- [ ] **Step 2: Create redo.svg**
```svg
<svg width="24" height="24" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
  <path d="M20 12V6M20 6H14M20 6C16 4 10 4 6 8C2 12 2 18 6 21" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
</svg>
```

- [ ] **Step 3: Update style.rs to support SVG icons**
Modify `crates/seisly_app/src/ui/style.rs` to include helper functions for rendering these icons if needed, or simply ensure `currentColor` works with egui's tint.

- [ ] **Step 4: Commit**
```bash
git add crates/seisly_app/assets/icons/*.svg
git commit -m "feat: add undo and redo SVG icons"
```

---

### Task 2: Logging Infrastructure

**Files:**
- Create: `crates/seisly_app/src/diagnostics.rs`
- Modify: `crates/seisly_app/src/lib.rs` (to export diagnostics)

- [ ] **Step 1: Implement global log buffer**
Create `crates/seisly_app/src/diagnostics.rs` with a `LogEntry` struct and a static `Vec` protected by a `Mutex`.

- [ ] **Step 2: Add logging hook**
Implement a simple `tracing` layer or `log` logger that pushes to this buffer.

- [ ] **Step 3: Test logging**
Write a unit test to verify logs are captured in the buffer.

- [ ] **Step 4: Commit**
```bash
git add crates/seisly_app/src/diagnostics.rs
git commit -m "feat: add global log buffer and diagnostics module"
```

---

### Task 3: UI - Logs Tab & Icon Fix

**Files:**
- Modify: `crates/seisly_app/src/ui/layout.rs`
- Modify: `crates/seisly_app/src/app.rs`

- [ ] **Step 1: Add Logs tab to DockState**
Update `SeislyTabViewer` in `crates/seisly_app/src/ui/layout.rs` to handle a new `Tab::Logs`.

- [ ] **Step 2: Implement Logs UI**
In `crates/seisly_app/src/app.rs`, implement the rendering logic for the logs (filtering, clearing, auto-scroll).

- [ ] **Step 3: Replace ribbon icons**
Update `crates/seisly_app/src/app.rs` to use `egui::Image` with the new SVG paths instead of Unicode strings for Undo/Redo.

- [ ] **Step 4: Commit**
```bash
git add crates/seisly_app/src/ui/layout.rs crates/seisly_app/src/app.rs
git commit -m "feat: implement Logs tab and fix undo/redo icons"
```

---

### Task 4: Loading Overlay & Import State

**Files:**
- Modify: `crates/seisly_app/src/app.rs`

- [ ] **Step 1: Add state flags**
Add `is_loading: bool` and `loading_message: String` to `SeislyApp`.

- [ ] **Step 2: Implement Spinner Overlay**
Add a method `show_loading_overlay` that draws a spinning icon and message on top of everything.

- [ ] **Step 3: Commit**
```bash
git add crates/seisly_app/src/app.rs
git commit -m "feat: add global loading overlay and state"
```

---

### Task 5: SEGY Import Wizard

**Files:**
- Modify: `crates/seisly_app/src/app.rs`
- Modify: `crates/seisly_io/src/segy/parser.rs`

- [ ] **Step 1: Enhance SegyMetadata**
Ensure `SegyMetadata` has all fields needed for the scan summary.

- [ ] **Step 2: Implement Scan Wizard UI**
Add a modal window in `app.rs` that appears when `import_state` is `Scanned`.

- [ ] **Step 3: Connect Background Tasks**
Use `tokio::spawn` or a thread to run the scan/import without blocking the UI. Update `is_loading` accordingly.

- [ ] **Step 4: Final Verification**
Run the app and verify the full flow: Pick -> Scan Popup -> Confirm -> Loading Spinner -> Success.

- [ ] **Step 5: Commit**
```bash
git commit -a -m "feat: implement full SEGY import wizard with background scanning"
```
