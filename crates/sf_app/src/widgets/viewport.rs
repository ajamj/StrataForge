use eframe::egui;
use eframe::egui_wgpu;
use crate::interpretation::{InterpretationState, Pick, PickSource, PickingMode};
use sf_compute::seismic::SeismicVolume;
use sf_compute::tracking::{snap_to_extrema, track_event};

pub struct ViewportWidget {
    pub target_format: Option<eframe::wgpu::TextureFormat>,
}

impl ViewportWidget {
    pub fn new() -> Self {
        Self {
            target_format: None,
        }
    }

    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        interpretation: &mut InterpretationState,
        volume: Option<&SeismicVolume>,
    ) {
        let (rect, response) = ui.allocate_at_least(ui.available_size(), egui::Sense::click());
        
        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                if interpretation.picking_mode != PickingMode::None {
                    self.handle_click(pos, rect, interpretation, volume);
                }
            }
        }

        if let Some(format) = self.target_format {
            let callback = egui_wgpu::Callback::new_paint_callback(
                rect,
                ViewportCallback { format },
            );
            ui.painter().add(callback);
        }
        
        // Add a visual fallback to confirm the widget area is correctly allocated
        ui.painter().rect_stroke(rect, 0.0, (1.0, egui::Color32::DARK_GRAY));

        // 2D Overlay Visualization (Fallback for stub 3D renderer)
        self.draw_overlays(ui, rect, interpretation);
    }

    fn draw_overlays(&self, ui: &mut egui::Ui, rect: egui::Rect, interpretation: &InterpretationState) {
        let painter = ui.painter().with_clip_rect(rect);

        for horizon in &interpretation.horizons {
            if !horizon.is_visible { continue; }
            
            let color = egui::Color32::from_rgb(
                (horizon.color[0] * 255.0) as u8,
                (horizon.color[1] * 255.0) as u8,
                (horizon.color[2] * 255.0) as u8,
            );

            // Draw Picks
            for pick in &horizon.picks {
                let x = rect.min.x + (pick.position[0] / 500.0) * rect.width();
                let y = rect.min.y + (pick.position[1] / 500.0) * rect.height();
                painter.circle_filled(egui::pos2(x, y), 3.0, color);
            }

            // Draw Surface Mesh (as wireframe in 2D)
            if let Some(mesh) = &horizon.mesh {
                for chunk in mesh.indices.chunks(3) {
                    if chunk.len() == 3 {
                        let p1 = mesh.vertices[chunk[0] as usize];
                        let p2 = mesh.vertices[chunk[1] as usize];
                        let p3 = mesh.vertices[chunk[2] as usize];

                        let pts = [p1, p2, p3].map(|p| {
                            egui::pos2(
                                rect.min.x + (p[0] / 500.0) * rect.width(),
                                rect.min.y + (p[1] / 500.0) * rect.height(),
                            )
                        });

                        painter.line_segment([pts[0], pts[1]], (0.5, color.linear_multiply(0.3)));
                        painter.line_segment([pts[1], pts[2]], (0.5, color.linear_multiply(0.3)));
                        painter.line_segment([pts[2], pts[0]], (0.5, color.linear_multiply(0.3)));
                    }
                }
            }
        }
    }

    fn handle_click(
        &self,
        pos: egui::Pos2,
        rect: egui::Rect,
        interpretation: &mut InterpretationState,
        volume: Option<&SeismicVolume>,
    ) {
        // Simple projection: Map screen space to inline/xline/sample
        // viewport rect: min_inline=0, max_inline=500, min_xline=0, max_xline=500
        let rel_x = (pos.x - rect.min.x) / rect.width();
        let rel_y = (pos.y - rect.min.y) / rect.height();

        let iline = (rel_x * 500.0) as i32;
        let xline = (rel_y * 500.0) as i32;
        let mut sample = 250usize; // Default mid-depth

        if let Some(vol) = volume {
            if let Some(trace) = vol.provider.get_trace(iline, xline) {
                // Snap to nearest extrema
                sample = snap_to_extrema(&trace, sample, 20, true);
                
                if interpretation.picking_mode == PickingMode::AutoTrack {
                    let results = track_event(vol, iline, xline, sample, true, 0.5);
                    if let Some(horizon) = interpretation.active_horizon_mut() {
                        for (il, xl, s) in results {
                            horizon.add_pick(Pick::new([il as f32, xl as f32, s as f32], PickSource::AutoTracked));
                        }
                        horizon.update_mesh();
                    }
                    return;
                }
            }
        }

        // Manual or Seed pick
        let picking_mode = interpretation.picking_mode;
        if let Some(horizon) = interpretation.active_horizon_mut() {
            let source = match picking_mode {
                PickingMode::Seed => PickSource::Seed,
                _ => PickSource::Manual,
            };
            horizon.add_pick(Pick::new([iline as f32, xline as f32, sample as f32], source));
            horizon.update_mesh();
        }
    }
}

struct ViewportCallback {
    format: eframe::wgpu::TextureFormat,
}

impl egui_wgpu::CallbackTrait for ViewportCallback {
    fn prepare(
        &self,
        device: &egui_wgpu::wgpu::Device,
        _queue: &egui_wgpu::wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut egui_wgpu::wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<egui_wgpu::wgpu::CommandBuffer> {
        if !resources.contains::<sf_render::Renderer>() {
            resources.insert(sf_render::Renderer::new(device, self.format));
        }
        Vec::new()
    }

    fn paint<'a>(
        &'a self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut egui_wgpu::wgpu::RenderPass<'a>,
        resources: &'a egui_wgpu::CallbackResources,
    ) {
        if let Some(renderer) = resources.get::<sf_render::Renderer>() {
            renderer.render(render_pass);
        }
    }
}
