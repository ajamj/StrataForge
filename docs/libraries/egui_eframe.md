# egui / eframe

`egui` is the primary immediate-mode UI framework for the StrataForge desktop workstation. `eframe` provides the framework glue to run `egui` as a native desktop application.

## Overview

In StrataForge, we use `egui` because it allows for rapid UI development and seamless integration with our `wgpu` rendering engine. Its immediate-mode nature is particularly well-suited for high-performance visualization tools where the UI state changes frequently in sync with 3D data.

### Why egui?
- **Immediate Mode:** No need to manage complex widget hierarchies; the UI is rebuilt every frame.
- **Performance:** Lightweight and efficient, maintaining high framerates even with complex layouts.
- **Portable:** Runs on Windows, macOS, Linux, and even Web (Wasm).
- **WGPU Integration:** Excellent support for custom 3D viewports via `egui_wgpu`.

## Usage in Project

The core of the desktop application is defined in the `sf_app` crate. We implement the `eframe::App` trait to manage the application lifecycle and rendering.

### Basic App Implementation
Located in `crates/sf_app/src/app.rs`:

```rust
use eframe::egui;

pub struct StrataForgeApp {
    name: String,
    // ... other state
}

impl eframe::App for StrataForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("StrataForge");
                ui.separator();
                ui.label(format!("Project: {}", self.name));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Main Content Area");
        });
    }
}
```

### Custom 3D Viewport
We use `egui_wgpu` to render our 3D scene inside an `egui` panel. This is handled in `crates/sf_app/src/widgets/viewport.rs`:

```rust
pub fn ui(&mut self, ui: &mut egui::Ui) {
    let (rect, _response) = ui.allocate_at_least(ui.available_size(), egui::Sense::drag());
    
    // Create a paint callback for wgpu rendering
    let callback = egui_wgpu::Callback::new_paint_callback(
        rect,
        MyRenderCallback {}, // Implements CallbackTrait
    );
    ui.painter().add(callback);
}
```

## Key Concepts

- **Context (`egui::Context`):** The central state of the UI. You use it to request repaints, check input state, and manage visual settings.
- **Ui (`egui::Ui`):** Represents a region of the screen where you can add widgets.
- **Immediate Mode:** Widgets are functions that are called every frame. If you want a button, you call `ui.button()`. If it returns `clicked()`, you perform the action.
- **Panels:** `TopBottomPanel`, `SidePanel`, and `CentralPanel` are the primary layout primitives.
- **Response:** Most widget calls return a `Response` object containing information about interactions (hover, click, drag).

## Resources

- [Official egui Website](https://egui.rs/)
- [egui Documentation (docs.rs)](https://docs.rs/egui)
- [eframe Documentation (docs.rs)](https://docs.rs/eframe)
- [egui Web Demo](https://www.egui.rs/#demo)
