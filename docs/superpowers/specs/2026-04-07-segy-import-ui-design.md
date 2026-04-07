# Design Doc: SEGY Import Wizard, Loading System, Log Console, and Icon Fix

Date: 2026-04-07
Topic: SEGY Import & UI Polish

## 1. Overview
This project addresses multiple gaps in the Seisly workstation UI:
- **SEGY Import**: Current placeholder `import_seismic` logic only picks a file. We need a "Scan" wizard that displays metadata before importing.
- **Loading UX**: The app lacks visual feedback for long-running operations.
- **Diagnostics**: Errors and info logs are hidden from the user.
- **Visuals**: Undo/Redo icons are broken (showing as Unicode squares).

## 2. Architecture & Components

### 2.1 SEGY Import Wizard (`SeislyApp`)
- **State**: `ImportState` enum: `Idle`, `Scanning(PathBuf)`, `Scanned(SegyMetadata)`, `Importing(PathBuf)`.
- **Scan Phase**: Background thread reads the EBCDIC/Binary header using `seisly_io::parse_metadata`.
- **Wizard UI**: A modal window showing the scan results (Trace count, samples, interval, format).
- **Final Import**: On "Confirm", the volume is added to `seismic_volumes` and `volume` state is updated.

### 2.2 Global Loading Overlay
- **Trigger**: Active when `SeislyApp::is_busy` is true or during `Scanning`/`Importing` states.
- **Visual**: A smooth CSS-style spinner drawn with `egui::Painter` in the center of the workspace.
- **Impact**: Blocks interaction while active to prevent race conditions.

### 2.3 Log Console Tab
- **Logging Layer**: A global thread-safe log buffer (limited to 1000 entries).
- **UI**: A new tab in the bottom panel with:
  - Level filters (Error, Warn, Info).
  - Search/filter capability.
  - "Clear" and "Copy to Clipboard" actions.

### 2.4 SVG Icons Integration
- **Assets**: Create `undo.svg` and `redo.svg` in `crates/seisly_app/assets/icons/`.
- **Rendering**: Replace Unicode characters with `egui::Image` widgets using existing image loaders.

## 3. Implementation Plan
- **Step 1**: Create the SVG icons and update `style.rs` to handle image loading for icons.
- **Step 2**: Implement the Log Capture buffer and the Logs UI tab.
- **Step 3**: Implement the `is_busy` flag and the loading spinner overlay.
- **Step 4**: Rewrite `import_seismic` to handle the Scan Wizard and Background Loading.

## 4. Success Criteria
- [ ] User can select a SEG-Y file, see its metadata in a popup, and confirm import.
- [ ] A spinner appears when scanning/loading files.
- [ ] Logs from the current session are visible in a dedicated tab.
- [ ] Undo and Redo icons render correctly as arrows.
