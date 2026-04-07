use eframe::egui;

#[allow(dead_code)]
pub struct SyntheticDataWidget {
    pub freq: f32,
    pub noise: f32,
}

impl SyntheticDataWidget {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            freq: 25.0,
            noise: 0.1,
        }
    }

    #[allow(dead_code)]
    pub fn ui(&mut self, ui: &mut egui::Ui) -> Option<SyntheticDataResult> {
        let mut result = None;
        ui.vertical(|ui| {
            ui.add(egui::Slider::new(&mut self.freq, 5.0..=80.0).text("Frequency (Hz)"));
            ui.add(egui::Slider::new(&mut self.noise, 0.0..=1.0).text("Noise Level"));
            if ui.button("Generate").clicked() {
                result = Some(self.generate_seismic());
            }
        });
        result
    }

    #[allow(dead_code)]
    fn generate_seismic(&mut self) -> SyntheticDataResult {
        SyntheticDataResult::Success
    }
}

#[allow(dead_code)]
pub enum SyntheticDataResult {
    Success,
    Error(String),
}
