//! Fault-guided tracking algorithms

use seisly_compute::seismic::TraceProvider;

pub struct FaultGuidedTracker {
    #[allow(dead_code)]
    threshold: f32,
}

impl FaultGuidedTracker {
    pub fn new(threshold: f32) -> Self {
        Self { threshold }
    }

    pub fn track(&self, _provider: &dyn TraceProvider, seed: [f32; 3]) -> Vec<[f32; 3]> {
        // Placeholder for fault-guided tracking logic
        vec![seed]
    }
}
