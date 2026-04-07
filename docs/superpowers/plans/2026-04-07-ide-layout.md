# IDE-Inspired Workstation Layout Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refactor the workstation UI to a flexible, dockable, and toggleable IDE-like layout with a semantic token-based theming system and professional Lucide icons.

**Architecture:** 
1. **Theming**: Centralize all colors into a `Theme` struct using semantic tokens.
2. **Layout**: Replace fixed panels with a layout shell consisting of an `ActivityBar` (left), `Sidebar` (dynamic), and a `CentralPanel` (viewport).
3. **State**: Use a `UiState` struct to manage panel visibility and active tool states.

**Tech Stack:** Rust, egui, eframe, egui_extras (SVG support).

---

### Task 1: Semantic Theme System

**Files:**
- Modify: `crates/seisly_app/src/ui/style.rs`

- [ ] **Step 1: Define Theme struct and Semantic Tokens**
Implement a `Theme` struct that uses semantic tokens instead of absolute color constants.

```rust
pub struct Theme {
    pub name: String,
    pub activity_bar_bg: Color32,
    pub activity_bar_fg: Color32,
    pub sidebar_bg: Color32,
    pub sidebar_border: Color32,
    pub editor_bg: Color32,
    pub status_bar_bg: Color32,
    pub accent: Color32,
}
```

- [ ] **Step 2: Implement Theme provider**
Create a `ThemeProvider` that can switch between "Dark" and "Light" presets.

- [ ] **Step 3: Update apply_theme helper**
Update the existing `apply_theme` function to map these tokens to `egui::Visuals`.

- [ ] **Step 4: Commit**
```bash
git add crates/seisly_app/src/ui/style.rs
git commit -m "feat: implement semantic token-based theming system"
```

---

### Task 2: Icon Assets & Integration

**Files:**
- Create: `crates/seisly_app/assets/icons/files.svg`, `crates/seisly_app/assets/icons/search.svg`, `crates/seisly_app/assets/icons/settings.svg`
- Modify: `crates/seisly_app/src/app.rs`

- [ ] **Step 1: Download/Create Lucide SVG assets**
Add standard Lucide icons for Explorer, Search, Interpretation, and Settings to the assets folder.

- [ ] **Step 2: Test image loading**
Ensure `egui_extras` is correctly loading the new SVGs in `app.rs`.

- [ ] **Step 3: Commit**
```bash
git add crates/seisly_app/assets/icons/*.svg
git commit -m "feat: add Lucide icon assets for the activity bar"
```

---

### Task 3: Activity Bar & Side Container

**Files:**
- Modify: `crates/seisly_app/src/app.rs`
- Modify: `crates/seisly_app/src/ui/layout.rs`

- [ ] **Step 1: Implement ActivityBar widget**
Create a new method `render_activity_bar` in `app.rs` that draws the fixed vertical bar.

- [ ] **Step 2: Implement Sidebar toggle logic**
Add `show_sidebar: bool` and `active_sidebar_tab: SidebarTab` to `SeislyApp`.

- [ ] **Step 3: Refactor Main Shell**
Update `update()` in `app.rs` to arrange panels using `egui::SidePanel` for the ActivityBar and Sidebar.

- [ ] **Step 4: Commit**
```bash
git add crates/seisly_app/src/app.rs crates/seisly_app/src/ui/layout.rs
git commit -m "feat: implement activity bar and toggleable sidebars"
```

---

### Task 4: Bottom Panel & View Menu

**Files:**
- Modify: `crates/seisly_app/src/app.rs`

- [ ] **Step 1: Implement toggleable Bottom Panel**
Add `show_bottom_panel: bool` state and wrap the Logs/Terminal in a toggleable `egui::TopBottomPanel`.

- [ ] **Step 2: Update View Menu**
Add checkboxes to the `View` menu for each panel: `[x] Activity Bar`, `[x] Side Bar`, `[x] Bottom Panel`.

- [ ] **Step 3: Implement hotkeys**
Add `Ctrl+B` to toggle the sidebar and `Ctrl+J` to toggle the bottom panel.

- [ ] **Step 4: Commit**
```bash
git add crates/seisly_app/src/app.rs
git commit -m "feat: add view menu toggles and hotkeys for panels"
```

---

### Task 5: Final Polish & Persistence

**Files:**
- Modify: `crates/seisly_app/src/app.rs`

- [ ] **Step 1: Save UI state to storage**
Ensure `show_sidebar`, `show_bottom_panel`, and `current_theme` are serialized/deserialized in `save()` and `new()`.

- [ ] **Step 2: Verify Layout resizing**
Ensure the viewport correctly fills the center area regardless of which panels are visible.

- [ ] **Step 3: Final build & run**
Run `cargo build --release` and verify the full IDE experience.

- [ ] **Step 4: Commit**
```bash
git add .
git commit -m "feat: finalize IDE layout with persistence and resize handling"
```
