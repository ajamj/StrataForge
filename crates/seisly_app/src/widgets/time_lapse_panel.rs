use eframe::egui;

#[derive(Default)]
pub struct TimeLapsePanel {}

impl TimeLapsePanel {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.add_space(8.0);
            ui.label(egui::RichText::new("4D Monitoring").strong());
            ui.add_space(8.0);
            ui.separator();
            ui.add_space(16.0);
            ui.label("Survey difference and NRMS computation coming soon.");
        });
    }
}
