use eframe::egui;

use crate::widgets::viewport::ViewportWidget;
use crate::interpretation::{InterpretationState, Horizon, PickingMode};

pub struct StrataForgeApp {
    name: String,
    viewport: ViewportWidget,
    interpretation: InterpretationState,
}

impl StrataForgeApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut interpretation = InterpretationState::new();
        // Add a default horizon for demo
        interpretation.add_horizon(Horizon::new("Horizon A".to_string(), [0.0, 1.0, 0.0]));

        Self {
            name: "MyField".to_owned(),
            viewport: ViewportWidget::new(),
            interpretation,
        }
    }
}

impl eframe::App for StrataForgeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("StrataForge");
                ui.separator();
                ui.label(format!("Project: {}", self.name));

                ui.separator();
                ui.label("Picking:");
                ui.selectable_value(&mut self.interpretation.picking_mode, PickingMode::None, "None");
                ui.selectable_value(&mut self.interpretation.picking_mode, PickingMode::Seed, "Seed");
                ui.selectable_value(&mut self.interpretation.picking_mode, PickingMode::AutoTrack, "Auto-Track");
                ui.selectable_value(&mut self.interpretation.picking_mode, PickingMode::Manual, "Manual");
            });
        });

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Project Data");
            ui.separator();
            
            ui.collapsing("Seismic Volumes", |ui| {
                ui.label("None loaded");
            });

            ui.collapsing("Interpretation", |ui| {
                if ui.button("Add Horizon").clicked() {
                    let name = format!("Horizon {}", self.interpretation.horizons.len() + 1);
                    self.interpretation.add_horizon(Horizon::new(name, [1.0, 1.0, 0.0]));
                }
                ui.separator();
                let mut active_id = self.interpretation.active_horizon_id;
                for horizon in &mut self.interpretation.horizons {
                    ui.horizontal(|ui| {
                        let is_active = active_id == Some(horizon.id);
                        if ui.selectable_label(is_active, &horizon.name).clicked() {
                            active_id = Some(horizon.id);
                        }
                        ui.checkbox(&mut horizon.is_visible, "");
                        ui.label(format!("({} picks)", horizon.picks.len()));
                    });
                }
                self.interpretation.active_horizon_id = active_id;
            });
            
            ui.collapsing("Wells", |ui| {
                ui.label("Well-1");
            });
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("AI Analysis");
            ui.separator();
            if ui.button("Run Fault Detection").clicked() {
                println!("Fault detection requested");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.viewport.ui(ui);
        });
    }
}
