# QI & AVO Analysis Guide

## Quick Start

```rust
use seisly_qi::{AvoAnalysis, VpVsRatio, PoissonsRatio};

// AVO Analysis
let angles = vec![0.0, 10.0, 20.0, 30.0, 40.0];
let amplitudes = vec![2.0, 1.5, 1.0, 0.5, 0.0];
let avo = AvoAnalysis::new(angles, amplitudes);

let gradient = avo.gradient();
let intercept = avo.intercept();
let class = avo.classify(); // Class 1, 2, 3, or 4

// Elastic Parameters
let vp = 2500.0;
let vs = 1200.0;
let vp_vs = VpVsRatio::compute(vp, vs);
let poisson = PoissonsRatio::from_vp_vs(vp, vs);
```

## Overview

Quantitative Interpretation (QI) uses seismic amplitude variations with offset/angle to infer subsurface rock and fluid properties. AVO (Amplitude Versus Offset) analysis is the primary QI technique for hydrocarbon detection and reservoir characterization.

---

## AVO Analysis

### Theory

AVO analysis examines how seismic reflection amplitudes change with source-receiver offset (or incidence angle). The amplitude response is governed by the contrast in elastic properties across an interface.

### Shuey Approximation

The simplified Shuey equation (valid for angles < 30°):

```
R(θ) = A + B·sin²(θ)

where:
  A = Intercept (zero-offset reflection coefficient)
  B = Gradient (rate of amplitude change with angle)
  θ = Incidence angle
```

### Usage

```rust
use seisly_qi::AvoAnalysis;

// Input data
let angles = vec![0.0, 10.0, 20.0, 30.0, 40.0];  // degrees
let amplitudes = vec![2.0, 1.5, 1.0, 0.5, 0.0];  // arbitrary units

// Create AVO analysis
let avo = AvoAnalysis::new(angles, amplitudes);

// Compute AVO attributes
let intercept = avo.intercept();
let gradient = avo.gradient();
let fit_quality = avo.r_squared();

println!("Intercept: {:.3}", intercept);
println!("Gradient: {:.3}", gradient);
println!("R²: {:.3}", fit_quality);
```

### AVO Classification

AVO responses are classified into four categories based on intercept and gradient polarity:

| Class | Intercept (A) | Gradient (B) | Response | Interpretation |
|-------|---------------|--------------|----------|----------------|
| **1** | Positive (+) | Negative (-) | Decreasing amplitude with offset | Gas sands (hard over soft) |
| **2** | ~Zero (0) | Negative (-) | Flat then decreasing | Soft sands (impedance contrast) |
| **3** | Negative (-) | Negative (-) | Increasing negative amplitude | Bright spots (gas over hard) |
| **4** | Negative (-) | Positive (+) | Increasing amplitude with offset | Hard over soft (non-hydrocarbon) |

```rust
use seisly_qi::AvoAnalysis;

let avo = AvoAnalysis::new(angles, amplitudes);
let class = avo.classify();

match class {
    seisly_qi::AvoClass::Class1 => println!("Class 1: Potential gas sand"),
    seisly_qi::AvoClass::Class2 => println!("Class 2: Soft sand indicator"),
    seisly_qi::AvoClass::Class3 => println!("Class 3: Bright spot - strong gas indicator"),
    seisly_qi::AvoClass::Class4 => println!("Class 4: Hard over soft - likely non-hydrocarbon"),
    seisly_qi::AvoClass::Unclassified => println!("Unable to classify"),
}
```

### AVO Crossplot

Visualize intercept vs gradient for classification:

```rust
use seisly_qi::{AvoAnalysis, AvoCrossplot};

let mut crossplot = AvoCrossplot::new();

for (angles, amplitudes) in all_traces {
    let avo = AvoAnalysis::new(angles, amplitudes);
    crossplot.add_point(avo.intercept(), avo.gradient());
}

// Generate crossplot image
crossplot.save("avo_crossplot.png")?;
```

---

## Elastic Parameters

### Vp/Vs Ratio

The ratio of compressional to shear wave velocity is a key lithology and fluid indicator.

```rust
use seisly_qi::VpVsRatio;

// From Vp and Vs
let vp = 2500.0;  // m/s
let vs = 1200.0;  // m/s
let vp_vs = VpVsRatio::compute(vp, vs);
println!("Vp/Vs: {:.2}", vp_vs);

// Interpretation
if vp_vs < 1.5 {
    println!("Low Vp/Vs - possible gas or hard rock");
} else if vp_vs < 2.0 {
    println!("Moderate Vp/Vs - typical sand/shale");
} else {
    println!("High Vp/Vs - possible unconsolidated or fluid-saturated");
}
```

### Poisson's Ratio

Poisson's ratio measures the lateral strain response to axial loading. It's derived from Vp/Vs:

```
ν = (0.5·(Vp/Vs)² - 1) / ((Vp/Vs)² - 1)
```

```rust
use seisly_qi::PoissonsRatio;

// From Vp and Vs
let vp = 2500.0;
let vs = 1200.0;
let poisson = PoissonsRatio::from_vp_vs(vp, vs);
println!("Poisson's Ratio: {:.3}", poisson);

// Direct computation
let poisson_direct = PoissonsRatio::compute(vp, vs);

// Interpretation
if poisson < 0.1 {
    println!("Very low - possible gas or tight rock");
} else if poisson < 0.25 {
    println!("Low - typical for gas sands");
} else if poisson < 0.35 {
    println!("Moderate - brine sands or shales");
} else {
    println!("High - unconsolidated or overpressured");
}
```

### Lambda-Rho (λρ) and Mu-Rho (μρ)

Incompressibility (λρ) and rigidity (μρ) are valuable fluid and lithology indicators:

```rust
use seisly_qi::{LambdaRho, MuRho};

let vp = 2500.0;  // m/s
let vs = 1200.0;  // m/s
let rho = 2.2;    // g/cc

// Lambda-Rho (incompressibility)
let lr = LambdaRho::compute(vp, vs, rho);
println!("λρ: {:.2} GPa", lr);

// Mu-Rho (rigidity)
let mr = MuRho::compute(vs, rho);
println!("μρ: {:.2} GPa", mr);

// Interpretation
// Low λρ + moderate μρ → Gas sand
// High λρ + high μρ → Wet sand or shale
```

---

## Fluid Factor

The fluid factor highlights deviations from the background Vp-Vs trend, often indicating hydrocarbons.

### Computation

```rust
use seisly_qi::FluidFactor;

// Background trend (from wet sands/shales)
let background_vp = 3000.0;
let background_vs = 1500.0;

// Measured values
let measured_vp = 2500.0;
let measured_vs = 1200.0;

let fluid_factor = FluidFactor::compute(
    measured_vp, measured_vs,
    background_vp, background_vs
);

println!("Fluid Factor: {:.3}", fluid_factor);

// Interpretation
if fluid_factor < -0.1 {
    println!("Strong negative - possible gas");
} else if fluid_factor < 0.0 {
    println!("Negative - possible hydrocarbon effect");
} else {
    println!("Neutral to positive - background trend");
}
```

### Fluid Factor from AVO

```rust
use seisly_qi::{AvoAnalysis, FluidFactor};

let avo = AvoAnalysis::new(&angles, &amplitudes);
let fluid_factor = FluidFactor::from_avo(&avo);

println!("AVO-derived Fluid Factor: {:.3}", fluid_factor);
```

---

## Gassmann Substitution

Gassmann's equations model the effect of fluid substitution on elastic properties.

### Fluid Replacement

```rust
use seisly_qi::{GassmannSubstitution, FluidProperties};

// Rock frame properties
let vp_matrix = 5500.0;  // m/s (quartz)
let vs_matrix = 3300.0;  // m/s
let rho_matrix = 2.65;   // g/cc

let porosity = 0.25;     // 25% porosity

// Initial fluid (brine)
let brine = FluidProperties {
    bulk_modulus: 2.5,   // GPa
    density: 1.05,       // g/cc
};

// Replacement fluid (gas)
let gas = FluidProperties {
    bulk_modulus: 0.05,  // GPa
    density: 0.25,       // g/cc
};

// Perform substitution
let gassmann = GassmannSubstitution::new(
    vp_matrix, vs_matrix, rho_matrix,
    porosity, brine
);

let substituted = gassmann.substitute_fluid(gas);

println!("Original Vp: {:.0} m/s", gassmann.original_vp());
println!("Gas-saturated Vp: {:.0} m/s", substituted.vp);
println!("Vp reduction: {:.1}%", 
    100.0 * (gassmann.original_vp() - substituted.vp) / gassmann.original_vp());
```

### Saturation Modeling

```rust
use seisly_qi::{GassmannSubstitution, SaturationRange};

let gassmann = GassmannSubstitution::new(/* ... */);

// Model varying gas saturation
let saturation_range = SaturationRange {
    min: 0.0,   // 0% gas (100% brine)
    max: 1.0,   // 100% gas
    step: 0.1,  // 10% increments
};

let results = gassmann.model_saturation(saturation_range);

for result in results {
    println!("Sw={:.0}%, Vp={:.0} m/s, Vs={:.0} m/s, ρ={:.2} g/cc",
        result.water_saturation * 100.0,
        result.vp, result.vs, result.density);
}
```

---

## DHI Scoring

Direct Hydrocarbon Indicator (DHI) scoring combines multiple QI attributes into a single probability score.

### Multi-Attribute DHI

```rust
use seisly_qi::{DhiScore, DhiAttributes};

// Gather QI attributes at a location
let attributes = DhiAttributes {
    avo_class: seisly_qi::AvoClass::Class3,
    fluid_factor: -0.15,
    vp_vs_ratio: 1.45,
    poisson_ratio: 0.18,
    lambda_rho: 5.0,   // GPa·g/cc
    amplitude_anomaly: true,
};

// Compute DHI score (0.0 to 1.0)
let dhi = DhiScore::compute(&attributes);

println!("DHI Score: {:.2}", dhi.score);
println!("Confidence: {:.2}", dhi.confidence);

// Interpretation
if dhi.score > 0.8 {
    println!("High probability of hydrocarbons");
} else if dhi.score > 0.5 {
    println!("Moderate probability - further investigation recommended");
} else {
    println!("Low probability - likely non-hydrocarbon");
}
```

### Weighted DHI

```rust
use seisly_qi::{DhiScore, DhiWeights};

// Custom weights for your play
let weights = DhiWeights {
    avo_class: 0.3,
    fluid_factor: 0.25,
    vp_vs_ratio: 0.2,
    poisson_ratio: 0.15,
    lambda_rho: 0.1,
};

let dhi = DhiScore::with_weights(&attributes, weights);
println!("Weighted DHI Score: {:.2}", dhi.score);
```

---

## Workflows

### Basic AVO Workflow

```rust
use seisly_qi::{AvoAnalysis, AvoClass};

fn avo_workflow(segy_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load angle gathers
    let gathers = load_angle_gathers(segy_path)?;
    
    let mut results = Vec::new();
    
    for (trace_id, gather) in gathers.iter().enumerate() {
        let avo = AvoAnalysis::new(gather.angles.clone(), gather.amplitudes.clone());
        
        results.push(AvoResult {
            trace_id,
            intercept: avo.intercept(),
            gradient: avo.gradient(),
            class: avo.classify(),
            r_squared: avo.r_squared(),
        });
    }
    
    // Save results
    save_avo_results("avo_output.dat", &results)?;
    
    // Generate crossplot
    let mut crossplot = AvoCrossplot::new();
    for r in &results {
        crossplot.add_point(r.intercept, r.gradient);
    }
    crossplot.save("avo_crossplot.png")?;
    
    Ok(())
}
```

### Elastic Parameter Inversion

```rust
use seisly_qi::{VpVsRatio, PoissonsRatio, LambdaRho, MuRho};

fn elastic_inversion(vp_volume: &[f64], vs_volume: &[f64], 
                     rho_volume: &[f64]) -> Result<ElasticAttributes, Box<dyn std::error::Error>> {
    let mut vp_vs = Vec::new();
    let mut poisson = Vec::new();
    let mut lr = Vec::new();
    let mut mr = Vec::new();
    
    for i in 0..vp_volume.len() {
        vp_vs.push(VpVsRatio::compute(vp_volume[i], vs_volume[i]));
        poisson.push(PoissonsRatio::from_vp_vs(vp_volume[i], vs_volume[i]));
        lr.push(LambdaRho::compute(vp_volume[i], vs_volume[i], rho_volume[i]));
        mr.push(MuRho::compute(vs_volume[i], rho_volume[i]));
    }
    
    Ok(ElasticAttributes { vp_vs, poisson, lambda_rho: lr, mu_rho: mr })
}
```

### DHI Mapping

```rust
use seisly_qi::{DhiScore, DhiAttributes, AvoClass};

fn dhi_mapping(avo_results: &[AvoResult], 
               elastic: &ElasticAttributes) -> Result<DhiMap, Box<dyn std::error::Error>> {
    let mut dhi_scores = Vec::new();
    
    for (i, avo) in avo_results.iter().enumerate() {
        let attributes = DhiAttributes {
            avo_class: avo.class,
            fluid_factor: compute_fluid_factor(&avo),
            vp_vs_ratio: elastic.vp_vs[i],
            poisson_ratio: elastic.poisson[i],
            lambda_rho: elastic.lambda_rho[i],
            amplitude_anomaly: is_amplitude_anomaly(i),
        };
        
        let dhi = DhiScore::compute(&attributes);
        dhi_scores.push(dhi.score);
    }
    
    Ok(DhiMap { scores: dhi_scores })
}
```

---

## Quality Control

### AVO Fit Quality

```rust
use seisly_qi::AvoAnalysis;

let avo = AvoAnalysis::new(&angles, &amplitudes);

// Check R-squared
let r_squared = avo.r_squared();
if r_squared < 0.7 {
    eprintln!("Warning: Poor AVO fit (R² = {:.2})", r_squared);
}

// Check residuals
let residuals = avo.residuals();
let max_residual = residuals.iter().fold(0.0f64, |a, &b| a.max(b.abs()));
if max_residual > 0.5 {
    eprintln!("Warning: Large residuals detected");
}
```

### Outlier Detection

```rust
use seisly_qi::AvoOutlierDetector;

let mut detector = AvoOutlierDetector::new();

for avo_result in &all_results {
    detector.add_point(avo_result.intercept, avo_result.gradient);
}

let outliers = detector.find_outliers(2.0); // 2 sigma
println!("Found {} outliers", outliers.len());
```

---

## Best Practices

1. **Angle Range**: Use angles < 40° for Shuey approximation validity
2. **Data Quality**: Ensure proper amplitude preservation in processing
3. **Background Trend**: Establish local Vp-Vs trend from wet zones
4. **Calibration**: Tie to well logs where available
5. **Multi-Attribute**: Combine multiple QI attributes for robust interpretation
6. **Uncertainty**: Always assess fit quality and uncertainty

---

## Troubleshooting

| Issue | Possible Cause | Solution |
|-------|----------------|----------|
| Poor R² | Noisy data, wrong angle range | Apply noise attenuation, check angle mute |
| Unrealistic Vp/Vs | Picking errors, cycle skip | Review horizon picks |
| Inconsistent DHI | Wrong background trend | Recalibrate from wet zone |
| Negative Poisson's | Data quality issue | Check Vp/Vs computation |

---

## References

- Shuey, R. T. (1985). A simplification of the Zoeppritz equations.
- Castagna, J. P., & Swan, H. W. (1997). Principles of AVO crossplotting.
- Gassmann, F. (1951). Über die Elastizität poröser Medien.
- Avseth, P., et al. (2005). Quantitative Seismic Interpretation.

---

## See Also

- [Phase 2 Features Overview](PHASE_2_FEATURES.md)
- [4D Monitoring Guide](4D_MONITORING.md)
- [GPU Acceleration Guide](GPU_ACCELERATION.md)
- [API Documentation](../api/seisly_qi/)
