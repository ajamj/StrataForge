use eframe::egui;

#[derive(Default)]
pub struct QiPanel {}

impl QiPanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Quantitative Interpretation").strong());
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(16.0);
            ui.label("AVO and Elastic Parameter tools coming soon.");
        });
    }
}
