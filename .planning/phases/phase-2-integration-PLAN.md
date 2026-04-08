---
phase: Phase 2 (v0.4.0)
plan: phase-2-integration
type: feature
autonomous: true
wave: 1
objective: Integrate GPU Acceleration, Quantitative Interpretation (QI), and 4D Monitoring into the Seisly workstation UI.
must_haves:
  - GpuAttributeComputer initialization with CPU fallback.
  - QI Sidebar Panel with AVO Analysis.
  - 4D Monitoring Sidebar Panel with NRMS.
  - UI icons for new tabs.
  - Asynchronous computation polling to keep UI responsive.
---

# Plan: Phase 2 Integration (Revised)

## 1. Context and Objective
Phase 2 introduces advanced `seisly_qi`, `seisly_4d`, and `seisly_attributes_gpu` capabilities. This plan integrates these libraries into `seisly_app`, providing dedicated UI panels and handling the asynchronous nature of GPU-accelerated seismic processing.

## 2. Technical Strategy
- **Initialization:** Use a background thread or a non-blocking `Option` state to initialize `GpuAttributeComputer` since it is `async`.
- **UI Architecture:** Extend `SidebarTab` and the Activity Bar. Use `egui` channels to receive results from async GPU tasks.
- **Assets:** Generate Lucide-compatible SVGs for QI and 4D monitoring.

## 3. Work Units

<task id="WU-010" name="GPU Computer Integration">
  <action>
    - Add `seisly_attributes_gpu`, `seisly_qi`, and `seisly_4d` to `crates/seisly_app/Cargo.toml`.
    - Wrap `GpuAttributeComputer` in `Option` within `SeislyApp`.
    - Implement a thread-safe initialization strategy in `SeislyApp::new` (e.g., using `pollster::block_on` or a deferred init).
  </action>
  <verify>
    - `cargo check -p seisly_app` passes.
    - App logs "GPU Accelerator initialized" or a graceful fallback message.
  </verify>
</task>

<task id="WU-011" name="UI Asset Creation">
  <action>
    - Create `crates/seisly_app/assets/icons/qi.svg` (based on Lucide `target` pattern).
    - Create `crates/seisly_app/assets/icons/time_lapse.svg` (based on Lucide `history` pattern).
  </action>
  <verify>
    - SVG files exist and are valid XML.
  </verify>
</task>

<task id="WU-012" name="QI & 4D UI Extensions">
  <action>
    - Add `QI` and `TimeLapse` variants to `SidebarTab` in `app.rs`.
    - Update `render_activity_bar` to include toggles for these new tabs with the new icons.
    - Implement stub widgets `QiPanel` and `TimeLapsePanel` in `crates/seisly_app/src/widgets/`.
  </action>
  <verify>
    - Activity Bar shows new icons.
    - Clicking icons switches sidebar to the correct (stub) panels.
  </verify>
</task>

<task id="WU-013" name="QI Feature Implementation">
  <action>
    - Implement AVO Analysis UI in `QiPanel`.
    - Wire up `seisly_qi::AvoAnalysis` logic to UI inputs.
    - Implement Vp/Vs and Poisson's Ratio computation triggers.
  </action>
  <verify>
    - User can input angles/amplitudes and see AVO classification.
  </verify>
</task>

<task id="WU-014" name="4D Monitoring Implementation">
  <action>
    - Implement Volume selection (Base vs Monitor) in `TimeLapsePanel`.
    - Wire up `seisly_4d::TimeLapseMonitor` for NRMS computation.
    - Display 4D results in the status bar or a dedicated results label.
  </action>
  <verify>
    - User can select two surveys and compute NRMS.
  </verify>
</task>

<task id="WU-015" name="Async Attribute Polling">
  <action>
    - Update `AppMessage` to handle `GpuResult`.
    - Implement a "Compute RMS" button that dispatches an async task to `GpuAttributeComputer`.
    - Update `SeislyApp::update` to poll for results and update the volume state.
  </action>
  <verify>
    - UI remains responsive during GPU computation.
    - Results appear in the viewport/logs upon completion.
  </verify>
</task>

## 4. Success Criteria
- [x] Project compiles with Phase 2 dependencies.
- [x] `GpuAttributeComputer` initializes safely.
- [x] Activity Bar supports QI and 4D tabs with custom icons.
- [x] QI and 4D panels are functional.
- [x] Async attribute computation works without blocking the UI.
