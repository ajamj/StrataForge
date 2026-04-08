# Design Doc: IDE-Inspired Workstation Layout (Seisly Workstation Pro)

Date: 2026-04-07
Topic: UI/UX Refactor - Modern IDE Layout & Flexible Theming

## 1. Overview
Refactor the Seisly workstation UI from a fixed-panel layout to a flexible, dockable, and toggleable layout inspired by modern IDEs (e.g., VS Code). This version introduces a **Semantic Token System** for theming, allowing for complete color flexibility and future custom theme support.

## 2. Architecture & Components

### 2.1 Main Shell Structure
The application window will be divided into the following functional areas:
- **Activity Bar (Fixed Left)**: A thin vertical strip (approx. 48px) containing icons for primary tool categories.
- **Side Container (Dynamic Left/Right)**: Toggleable sidebars for Project Explorer, Properties, and Interpretation tools.
- **Editor Area (Center)**: The primary 3D/2D seismic viewport. Supports tabbed views for different surveys or crossplots.
- **Panel Area (Bottom)**: Toggleable bottom panel for Logs, Console, and Problems.
- **Status Bar (Fixed Bottom)**: Essential state information (coordinates, TWT, system status).

### 2.2 Semantic Theming System (VS Code Style)
- **Token Mapping**: Instead of hardcoded colors, components will use semantic tokens:
  - `activityBar.background`, `activityBar.foreground`
  - `sideBar.background`, `sideBar.border`
  - `editor.background`, `statusBar.background`
  - `icon.active`, `icon.inactive`
- **Theme Definition**: Themes are defined as structs/JSON mapping these tokens to `egui::Color32`.
- **Flexibility**: Switching themes replaces the entire token map, instantly updating the UI.

### 2.3 Activity Bar & Navigation
- **Icons**: Monochromatic SVG icons from the **Lucide** icon library.
- **Primary Actions**:
  - **Explorer**: Toggle Project Explorer sidebar.
  - **Interpretation**: Toggle Horizon/Fault management sidebar.
  - **Diagnostics**: Toggle Logs/Console bottom panel.
  - **Settings**: Open modal settings dialog.
- **Behavior**: Clicking an active tool icon collapses the sidebar; clicking an inactive one switches the sidebar content.

### 2.4 Layout State Management
- **Persistence**: Layout state (panel visibility, widths/heights) and the active theme selection will be saved to `eframe` storage.
- **Toggle Mechanism**:
  - Global hotkeys (e.g., `Ctrl+B` for sidebar).
  - **View Menu**: Explicit "Show [Panel Name]" toggle items.
  - Activity Bar clicks.

### 2.5 Technical Constraints (Rust/egui)
- **Ownership**: `egui::Response::on_hover_text` consumes the response. Must use `response.clone().on_hover_text(...)` or call it last.
- **Borrowing**: Closure capturing `self` while holding a reference to `self.theme_manager` is prohibited. Must clone the theme or extract necessary fields before the closure.
- **Step 1: Theme Engine**: Implement the `ThemeToken` system and a `ThemeProvider` in `style.rs`.
- **Step 2: Icon Integration**: Integrate Lucide SVG assets into `assets/icons/`.
- **Step 3: Shell Refactor**: Update `app.rs` and `layout.rs` to replace the current `TopBottomPanel` ribbon with the `ActivityBar` shell.
- **Step 4: View Menu & State**: Implement the "Show/Hide" logic and state persistence for all panels.

## 4. Success Criteria
- [x] User can toggle the left sidebar and bottom panel.
- [x] UI colors are entirely driven by the `ThemeToken` system.
- [x] A "Theme Switcher" (e.g., Dark/Light) is functional in Settings.
- [x] Icons are consistent, monochromatic Lucide SVGs.
- [x] Layout and Theme state persist across application restarts.
