---
phase: Phase 2 (v0.4.0)
plan: phase-2-integration-01-foundation
type: feature
autonomous: true
wave:1
depends_on: []
files_modified: 6
requirements: ["v0.4.0-foundation"]
objective: Initialize GPU Acceleration and create UI stubs for QI and 4D Monitoring.
must_haves:
  truths:
    - "The application launches instantly without hanging during GPU discovery."
    - "UI spacing follows the 8px grid system."
  artifacts:
    - path: "crates/seisly_app/assets/icons/qi.svg"
      provides: "Lucide-compatible target icon for QI tab"
      min_lines: 3
    - path: "crates/seisly_app/assets/icons/time_lapse.svg"
      provides: "Lucide-compatible history icon for 4D tab"
      min_lines: 3
  key_links:
    - from: "crates/seisly_app/src/app.rs"
      to: "crates/seisly_attributes_gpu/src/compute.rs"
      via: "Background thread GPU initialization"
---

# Plan: Phase 2 Integration - 01 Foundation

## 1. Context and Objective
This plan establishes the foundation for Phase 2 by integrating the `seisly_attributes_gpu` core and preparing the UI shell for advanced interpretation tools.

## 2. Technical Strategy
- **Initialization:** Spawn a background thread in `SeislyApp::new` to initialize `GpuAttributeComputer`. Send the result via `tx`.
- **UI Architecture:** Instantiate `QiPanel` and `TimeLapsePanel` in `SeislyApp`. Update all `match` arms for `SidebarTab` to prevent compilation errors.
- **Dependencies:** Include `pollster` for blocking on the async init within the background thread.

## 3. Work Units

<task id="WU-010">
  <name>Dependency and Async GPU Initialization</name>
  <files>
    - crates/seisly_app/Cargo.toml
    - crates/seisly_app/src/app.rs
  </files>
  <action>
    - Add `seisly_attributes_gpu`, `seisly_qi`, `seisly_4d`, and `pollster` to `Cargo.toml`.
    - Update `AppMessage` enum with `GpuInitialized(Result<GpuAttributeComputer, String>)`.
    - In `SeislyApp::new`, spawn a thread that runs `pollster::block_on(GpuAttributeComputer::new())` and sends the result back.
    - Add `gpu_computer: Option<GpuAttributeComputer>` to `SeislyApp`.
  </action>
  <verify>
    `cargo check -p seisly_app` succeeds.
  </verify>
  <done>
    - Async GPU computer initialization logic implemented.
  </done>
</task>

<task id="WU-011">
  <name>UI Asset Creation</name>
  <files>
    - crates/seisly_app/assets/icons/qi.svg
    - crates/seisly_app/assets/icons/time_lapse.svg
  </files>
  <action>
    Create new SVG icons for QI and Time Lapse in the assets directory.
  </action>
  <verify>
    Icon files exist and are valid SVGs.
  </verify>
  <done>
    - Visual assets available.
  </done>
</task>

<task id="WU-012">
  <name>Sidebar and Activity Bar Extension (8px Grid)</name>
  <files>
    - crates/seisly_app/src/app.rs
  </files>
  <action>
    - Add `QI` and `TimeLapse` to `SidebarTab`.
    - Update `render_activity_bar` with new buttons and **change spacing to 8.0px**.
    - Update `activity_button` logic to handle the new variants.
  </action>
  <verify>
    Activity bar shows new icons with 8px spacing.
  </verify>
  <done>
    - Navigation shell expanded.
  </done>
</task>

<task id="WU-013">
  <name>QI and 4D Panel Integration</name>
  <files>
    - crates/seisly_app/src/widgets/qi_panel.rs
    - crates/seisly_app/src/widgets/time_lapse_panel.rs
    - crates/seisly_app/src/widgets/mod.rs
    - crates/seisly_app/src/app.rs
  </files>
  <action>
    - Create `qi_panel.rs` and `time_lapse_panel.rs` with `ui()` stubs.
    - Add `qi_panel: QiPanel` and `time_lapse_panel: TimeLapsePanel` fields to `SeislyApp`.
    - Update `render_sidebar` match arms to call `self.qi_panel.ui(ui)` and `self.time_lapse_panel.ui(ui)`.
  </action>
  <verify>
    Selecting QI or TimeLapse in the activity bar correctly renders the new panels in the sidebar.
  </verify>
  <done>
    - Phase 2 widgets wired into the application.
  </done>
</task>

## 4. Success Criteria
- [x] Project compiles with all Phase 2 dependencies.
- [x] GPU initialization is non-blocking.
- [x] All SidebarTab variants are handled in match statements.
- [x] UI follows the 8px grid.
