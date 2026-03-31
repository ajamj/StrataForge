//! Elastic FWI Implementation (Vp + Vs inversion)

use ndarray::{Array2, s};
use crate::acoustic::{AcousticWaveSolver, Source};

/// Elastic Wave Solver (simplified)
pub struct ElasticWaveSolver {
    vp: Array2<f32>,
    vs: Array2<f32>,
    rho: Array2<f32>,
    dt: f32,
    dx: f32,
    dz: f32,
}

impl ElasticWaveSolver {
    pub fn new(vp: Array2<f32>, vs: Array2<f32>, rho: Array2<f32>, dt: f32, dx: f32, dz: f32) -> Self {
        Self { vp, vs, rho, dt, dx, dz }
    }
    
    /// Forward modeling for elastic wave equation
    pub fn forward(&self, source: &Source, nt: usize) -> (Array2<f32>, Array2<f32>) {
        // Simplified: return P-wave and S-wave components
        // In production: implement full elastic wave equation
        let acoustic = AcousticWaveSolver::new(self.vp.clone(), self.dt, self.dx, self.dz);
        let wavefield = acoustic.forward(source, nt);
        
        // Return a slice of the wavefield at surface (z=0) over time
        // nz is axis 1, nx is axis 2
        let p_wave = wavefield.slice(s![.., 0, ..]).to_owned();
        let s_wave = &p_wave * 0.5;
        
        (p_wave, s_wave)
    }
}

/// Elastic FWI - Invert for both Vp and Vs
pub struct ElasticFWI {
    solver: ElasticWaveSolver,
    observed_p: Array2<f32>,
    observed_s: Array2<f32>,
}

impl ElasticFWI {
    pub fn new(
        vp_init: Array2<f32>,
        vs_init: Array2<f32>,
        rho_init: Array2<f32>,
        dt: f32,
        dx: f32,
        dz: f32,
        observed_p: Array2<f32>,
        observed_s: Array2<f32>,
    ) -> Self {
        let solver = ElasticWaveSolver::new(vp_init, vs_init, rho_init, dt, dx, dz);
        Self {
            solver,
            observed_p,
            observed_s,
        }
    }
    
    /// Compute elastic misfit
    pub fn misfit(&self, _source: &Source, _nt: usize) -> f32 {
        // TODO: Implement full elastic misfit
        0.0
    }
    
    /// Compute gradient for Vp and Vs
    pub fn gradient(&self, _source: &Source, _nt: usize) -> (Array2<f32>, Array2<f32>) {
        // TODO: Implement elastic gradient
        (Array2::zeros(self.solver.vp.dim()), 
         Array2::zeros(self.solver.vs.dim()))
    }
    
    /// Update velocity models
    pub fn update(&mut self, _grad_vp: &Array2<f32>, _grad_vs: &Array2<f32>, _lr: f32) {
        // TODO: Implement velocity update
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array2;

    #[test]
    fn test_elastic_solver_creation() {
        let vp = Array2::from_elem((100, 100), 3000.0);
        let vs = Array2::from_elem((100, 100), 1500.0);
        let rho = Array2::from_elem((100, 100), 2.5);
        
        let solver = ElasticWaveSolver::new(vp, vs, rho, 0.001, 10.0, 10.0);
        assert_eq!(solver.vp.nrows(), 100);
    }
    
    #[test]
    fn test_elastic_forward() {
        let vp = Array2::from_elem((50, 50), 3000.0);
        let vs = Array2::from_elem((50, 50), 1500.0);
        let rho = Array2::from_elem((50, 50), 2.5);
        let solver = ElasticWaveSolver::new(vp, vs, rho, 0.001, 10.0, 10.0);
        let source = Source::new(25, 25, 25.0);
        
        let (p, s) = solver.forward(&source, 100);
        assert_eq!(p.dim(), (100, 50));
        assert_eq!(s.dim(), (100, 50));
    }
}
