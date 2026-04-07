use eframe::egui;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct AppSettings {
    pub theme: String,
}

pub struct SettingsPanel {
    #[allow(dead_code)]
    pub settings: AppSettings,
}

impl SettingsPanel {
    pub fn new() -> Self {
        Self {
            settings: AppSettings::default(),
        }
    }

    #[allow(dead_code)]
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        let mut changed = false;
        ui.vertical(|ui| {
            ui.heading("General Settings");
            ui.horizontal(|ui| {
                ui.label("Theme Preference:");
                if ui.text_edit_singleline(&mut self.settings.theme).changed() {
                    changed = true;
                }
            });
        });
        changed
    }
}
