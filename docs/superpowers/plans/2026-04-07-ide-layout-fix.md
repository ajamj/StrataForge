# Refined IDE Layout Fixes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Resolve compiler errors (ownership/borrowing) in the new IDE layout and complete the shell refactor.

**Architecture:** 
- Fix `activity_button` ownership by cloning `Response`.
- Fix `render_sidebar` borrow collision by cloning theme fields before closure.
- Restore all core logic methods (`render_interpretation_panel`, etc.).

**Tech Stack:** Rust, egui.

---

### Task 1: Fix Ownership & Borrowing Errors

**Files:**
- Modify: `crates/seisly_app/src/app.rs`

- [ ] **Step 1: Fix activity_button ownership**
Use `response.on_hover_text` correctly or clone it.

- [ ] **Step 2: Fix render_sidebar borrow collision**
Clone the theme or extract header color before the `egui::SidePanel` closure.

- [ ] **Step 3: Commit fixes**
```bash
git add crates/seisly_app/src/app.rs
git commit -m "fix: resolve ownership and borrowing issues in IDE layout"
```

---

### Task 2: Restore & Complete Logic

**Files:**
- Modify: `crates/seisly_app/src/app.rs`

- [ ] **Step 1: Restore render_interpretation_panel full logic**
Implement the horizon and fault lists correctly in the sidebar.

- [ ] **Step 2: Implement remaining shell features**
Complete `View` menu toggles and persistence logic.

- [ ] **Step 3: Verify build**
Run `cargo build --release` to ensure everything passes.

- [ ] **Step 4: Commit**
```bash
git add .
git commit -m "feat: complete IDE shell refactor and restore core logic"
```
