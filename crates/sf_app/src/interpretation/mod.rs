use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sf_core::domain::surface::Mesh;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PickSource {
    Manual,
    AutoTracked,
    Seed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pick {
    pub id: Uuid,
    pub position: [f32; 3],
    pub confidence: f32,
    pub source: PickSource,
}

impl Pick {
    pub fn new(position: [f32; 3], source: PickSource) -> Self {
        Self {
            id: Uuid::new_v4(),
            position,
            confidence: 1.0,
            source,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Horizon {
    pub id: Uuid,
    pub name: String,
    pub picks: Vec<Pick>,
    pub color: [f32; 3],
    pub is_visible: bool,
    #[serde(skip)]
    pub mesh: Option<Mesh>,
}

impl Horizon {
    pub fn new(name: String, color: [f32; 3]) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            picks: Vec::new(),
            color,
            is_visible: true,
            mesh: None,
        }
    }

    pub fn add_pick(&mut self, pick: Pick) {
        self.picks.push(pick);
    }

    pub fn update_mesh(&mut self) {
        if self.picks.len() < 3 {
            return;
        }

        use sf_compute::interpolation::{RbfInterpolator, RbfType};

        let points: Vec<[f32; 3]> = self.picks.iter().map(|p| p.position).collect();
        if let Ok(interp) = RbfInterpolator::new(&points, RbfType::ThinPlateSpline) {
            // Find bounds
            let mut min_x = f32::MAX;
            let mut max_x = f32::MIN;
            let mut min_y = f32::MAX;
            let mut max_y = f32::MIN;

            for p in &points {
                min_x = min_x.min(p[0]);
                max_x = max_x.max(p[0]);
                min_y = min_y.min(p[1]);
                max_y = max_y.max(p[1]);
            }

            // Expand bounds slightly
            let dx = ((max_x - min_x) * 0.1).max(10.0);
            let dy = ((max_y - min_y) * 0.1).max(10.0);

            self.mesh = Some(interp.generate_mesh(
                min_x - dx,
                max_x + dx,
                min_y - dy,
                max_y + dy,
                20,
                20,
            ));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PickingMode {
    None,
    Seed,
    AutoTrack,
    Manual,
}

pub struct InterpretationState {
    pub horizons: Vec<Horizon>,
    pub active_horizon_id: Option<Uuid>,
    pub picking_mode: PickingMode,
}

impl InterpretationState {
    pub fn new() -> Self {
        Self {
            horizons: Vec::new(),
            active_horizon_id: None,
            picking_mode: PickingMode::None,
        }
    }

    pub fn add_horizon(&mut self, horizon: Horizon) {
        self.horizons.push(horizon);
    }

    pub fn active_horizon(&self) -> Option<&Horizon> {
        self.active_horizon_id.and_then(|id| self.horizons.iter().find(|h| h.id == id))
    }

    pub fn active_horizon_mut(&mut self) -> Option<&mut Horizon> {
        self.active_horizon_id.and_then(|id| self.horizons.iter_mut().find(|h| h.id == id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpretation_state_creation() {
        let mut state = InterpretationState::new();
        assert_eq!(state.horizons.len(), 0);
        assert!(state.active_horizon_id.is_none());

        let horizon = Horizon::new("H1".to_string(), [1.0, 0.0, 0.0]);
        let h_id = horizon.id;
        state.add_horizon(horizon);
        state.active_horizon_id = Some(h_id);

        assert_eq!(state.horizons.len(), 1);
        assert_eq!(state.active_horizon().unwrap().name, "H1");
    }

    #[test]
    fn test_picking_logic() {
        let mut horizon = Horizon::new("H1".to_string(), [1.0, 0.0, 0.0]);
        let pick = Pick::new([100.0, 200.0, 10.0], PickSource::Manual);
        horizon.add_pick(pick);
        
        assert_eq!(horizon.picks.len(), 1);
        assert_eq!(horizon.picks[0].position, [100.0, 200.0, 10.0]);
    }
}
