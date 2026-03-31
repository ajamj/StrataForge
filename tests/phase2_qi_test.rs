//! Phase 2 Integration Tests

use seisly_qi::{AvoAnalysis, VpVsRatio, PoissonsRatio, Dhi, Gassmann, FluidFactor};

#[test]
fn test_full_qi_workflow() {
    // Complete QI workflow
    let angles = vec![0.0, 10.0, 20.0, 30.0, 40.0];
    let amplitudes = vec![2.0, 1.5, 1.0, 0.5, 0.0];
    
    let avo = AvoAnalysis::new(angles, amplitudes);
    let gradient = avo.gradient();
    let intercept = avo.intercept();
    let class = avo.classify();
    
    // Compute elastic parameters
    let vp = 2500.0;
    let vs = 1200.0;
    let vp_vs = VpVsRatio::compute(vp, vs);
    let poisson = PoissonsRatio::from_vp_vs(vp, vs);
    
    // DHI scoring
    let dhi_score = Dhi::score(vp_vs, poisson, intercept, gradient);
    
    assert_eq!(class, seisly_qi::AvoClass::Class1);
    assert!(vp_vs > 2.0, "Gas sand should have high Vp/Vs");
    assert!(dhi_score > 0.5, "Should detect DHI");
}

#[test]
fn test_gassmann_fluid_substitution() {
    // Brine to gas substitution
    let vp_brine = 2800.0;
    let vs_brine = 1400.0;
    let rho_brine = 2.3;
    
    let k_brine = 15.0;
    let k_gas = 0.5;
    let porosity = 0.25;
    
    let vp_gas = Gassmann::vp_after_substitution(
        vp_brine, vs_brine, rho_brine, k_brine, k_gas, porosity,
    );
    
    assert!(vp_gas < vp_brine, "Vp should decrease with gas");
}

#[test]
fn test_elastic_parameters_consistency() {
    // Test multiple wells
    let wells = vec![
        ("Gas Sand", 2500.0, 1200.0, 2.2),
        ("Brine Sand", 2800.0, 1400.0, 2.3),
        ("Shale", 3200.0, 1600.0, 2.5),
    ];
    
    for (name, vp, vs, rho) in wells {
        let vp_vs = VpVsRatio::compute(vp, vs);
        let poisson = PoissonsRatio::from_vp_vs(vp, vs);
        
        // Verify calculations are finite and in reasonable ranges
        assert!(vp_vs.is_finite(), "{}: Vp/Vs should be finite", name);
        assert!(poisson >= 0.0 && poisson <= 0.5, "{}: Poisson's ratio out of range", name);
        
        println!("{}: Vp/Vs={:.2}, Poisson={:.2}", name, vp_vs, poisson);
    }
}

#[test]
fn test_fluid_factor_discrimination() {
    // Gas sand should have different fluid factor than brine
    let gas_sand_ff = FluidFactor::from_elastic(2500.0, 1200.0, 2.2);
    let brine_sand_ff = FluidFactor::from_elastic(2800.0, 1400.0, 2.3);
    
    // Gas should have higher fluid factor
    assert!(gas_sand_ff > brine_sand_ff, "Gas should have higher fluid factor");
}

#[test]
fn test_multi_trace_qi() {
    // Test QI on multiple traces (simulating volume)
    let num_traces = 10;
    let samples_per_trace = 100;
    
    for trace_idx in 0..num_traces {
        let angles = vec![0.0, 10.0, 20.0, 30.0];
        let amplitudes: Vec<f32> = (0..4)
            .map(|i| ((trace_idx + i) as f32 * 0.5).sin())
            .collect();
        
        let avo = AvoAnalysis::new(angles, amplitudes);
        let gradient = avo.gradient();
        let intercept = avo.intercept();
        
        // Verify results are finite
        assert!(gradient.is_finite(), "Trace {} gradient should be finite", trace_idx);
        assert!(intercept.is_finite(), "Trace {} intercept should be finite", trace_idx);
    }
}
