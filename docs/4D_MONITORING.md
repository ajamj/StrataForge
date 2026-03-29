# 4D Time-Lapse Monitoring Guide

## Quick Start

```rust
use sf_4d::{TimeLapseMonitor, BaseMonitorPair};

// Load base and monitor surveys
let base = load_seismic("base_survey.segy");
let monitor = load_seismic("monitor_survey.segy");

// Create 4D monitor
let monitor_4d = TimeLapseMonitor::new(&base, &monitor);

// Compute difference
let difference = monitor_4d.compute_difference();

// Compute NRMS
let nrms = monitor_4d.compute_nrms();
println!("NRMS: {:.2}%", nrms);
```

## Overview

4D (time-lapse) seismic monitoring compares seismic surveys acquired at different times to detect changes in the subsurface caused by production, injection, or fluid movement.

## Core Concepts

### Base and Monitor Surveys

- **Base Survey**: The initial seismic survey (time zero)
- **Monitor Survey**: Subsequent surveys acquired during production

### Key Attributes

| Attribute | Description | Use Case |
|-----------|-------------|----------|
| Difference | Simple amplitude difference | Quick look analysis |
| RMS Difference | RMS of difference window | Energy change quantification |
| NRMS | Normalized RMS difference | Repeatability metric |
| Time Shift | Cross-correlation time shift | Velocity change detection |

## Difference Computation

### Simple Difference

The most basic 4D attribute is the simple amplitude difference:

```rust
use sf_4d::TimeLapseMonitor;

let monitor_4d = TimeLapseMonitor::new(&base, &monitor);
let difference = monitor_4d.compute_difference();

// difference[i] = monitor[i] - base[i]
```

### Windowed Difference

Compute difference within a time window:

```rust
use sf_4d::{TimeLapseMonitor, WindowConfig};

let config = WindowConfig {
    top_ms: 1000.0,
    bottom_ms: 1500.0,
};

let windowed_diff = monitor_4d.compute_windowed_difference(&config);
```

## RMS Analysis

### RMS Difference

Root Mean Square of the difference measures the energy of change:

```rust
use sf_4d::TimeLapseMonitor;

let monitor_4d = TimeLapseMonitor::new(&base, &monitor);
let rms_diff = monitor_4d.compute_rms_difference();

println!("RMS Difference: {:.2}", rms_diff);
```

### RMS Ratio

Compare RMS amplitudes between surveys:

```rust
let rms_ratio = monitor_4d.compute_rms_ratio();

if rms_ratio > 1.0 {
    println!("Monitor has higher energy than base");
} else {
    println!("Base has higher energy than monitor");
}
```

## NRMS (Normalized RMS)

NRMS is a normalized measure of difference relative to the average energy:

```
NRMS = 200 × RMS(Monitor - Base) / (RMS(Monitor) + RMS(Base))
```

```rust
use sf_4d::TimeLapseMonitor;

let monitor_4d = TimeLapseMonitor::new(&base, &monitor);
let nrms = monitor_4d.compute_nrms();

println!("NRMS: {:.2}%", nrms);

// Interpretation
if nrms < 10.0 {
    println!("Excellent repeatability");
} else if nrms < 20.0 {
    println!("Good repeatability");
} else if nrms < 30.0 {
    println!("Fair - some non-repeatable noise");
} else {
    println!("Poor - significant non-repeatable effects");
}
```

## Time Shift Analysis

Time shifts detect velocity changes between surveys:

```rust
use sf_4d::{TimeLapseMonitor, TimeShiftConfig};

let config = TimeShiftConfig {
    window_length_ms: 100.0,
    max_shift_ms: 20.0,
    step_ms: 2.0,
};

let time_shifts = monitor_4d.compute_time_shifts(&config);

// time_shifts[i] = time shift at trace i (in ms)
```

### Cross-Correlation Method

Time shifts are computed using cross-correlation:

```rust
use sf_4d::CrossCorrelation;

let cc = CrossCorrelation::new(&base_trace, &monitor_trace);
let lag = cc.find_peak_lag();
let correlation_coefficient = cc.peak_correlation();

println!("Time shift: {:.2} ms", lag * sample_rate);
println!("Correlation: {:.3}", correlation_coefficient);
```

## Production Data Integration

Correlate 4D response with production data:

```rust
use sf_4d::{ProductionData, WellCorrelation};

// Load production data
let prod_data = ProductionData::from_csv("production.csv");

// Load 4D attribute at well locations
let attribute_4d = load_attribute_at_wells("nrms_map.dat", &well_locations);

// Compute correlation
let correlation = WellCorrelation::compute(&prod_data, &attribute_4d);

println!("Correlation coefficient: {:.3}", correlation.coefficient);
println!("P-value: {:.4}", correlation.p_value);

if correlation.p_value < 0.05 {
    println!("Statistically significant correlation!");
}
```

### Production Attributes

```rust
use sf_4d::ProductionData;

let prod = ProductionData::new();
prod.add_well("Well-A", vec![
    (date1, 1000.0), // bbl/day
    (date2, 1200.0),
    (date3, 800.0),
]);

// Get cumulative production
let cumulative = prod.cumulative_production("Well-A");

// Get average rate
let avg_rate = prod.average_rate("Well-A");
```

## 4D Workflows

### Basic 4D Analysis

```rust
use sf_4d::{TimeLapseMonitor, WindowConfig, TimeShiftConfig};

fn basic_4d_analysis(base: &[f64], monitor: &[f64]) {
    let monitor_4d = TimeLapseMonitor::new(base, monitor);
    
    // 1. Simple difference
    let diff = monitor_4d.compute_difference();
    
    // 2. NRMS for repeatability
    let nrms = monitor_4d.compute_nrms();
    
    // 3. Time shifts
    let ts_config = TimeShiftConfig::default();
    let time_shifts = monitor_4d.compute_time_shifts(&ts_config);
    
    println!("NRMS: {:.2}%", nrms);
    println!("Mean time shift: {:.2} ms", 
             time_shifts.iter().sum::<f64>() / time_shifts.len() as f64);
}
```

### Advanced 4D with Production

```rust
use sf_4d::{TimeLapseMonitor, ProductionData, WellCorrelation};

fn advanced_4d_with_production(base: &[f64], monitor: &[f64],
                                prod_file: &str) {
    let monitor_4d = TimeLapseMonitor::new(base, monitor);
    
    // Compute 4D attributes
    let nrms = monitor_4d.compute_nrms();
    let diff = monitor_4d.compute_difference();
    
    // Load production
    let prod_data = ProductionData::from_csv(prod_file);
    
    // Extract 4D response at wells
    let well_nrms = extract_nrms_at_wells(&nrms, &prod_data.well_locations());
    
    // Correlate
    for well in prod_data.wells() {
        let correlation = WellCorrelation::compute(
            &prod_data.production(well),
            &well_nrms[well]
        );
        println!("Well {}: r={:.3}, p={:.4}", 
                 well, correlation.coefficient, correlation.p_value);
    }
}
```

## Quality Control

### Repeatability Assessment

```rust
use sf_4d::RepeatabilityMetrics;

let metrics = RepeatabilityMetrics::compute(&base, &monitor);

println!("=== 4D Quality Metrics ===");
println!("NRMS: {:.2}%", metrics.nrms);
println!("Correlation: {:.3}", metrics.correlation);
println!("Mean time shift: {:.2} ms", metrics.mean_time_shift);
println!("RMS ratio: {:.2}", metrics.rms_ratio);

// Quality flag
let quality = metrics.quality_flag();
println!("Overall quality: {:?}", quality);
```

### Noise Estimation

```rust
use sf_4d::NoiseEstimator;

let estimator = NoiseEstimator::new(&base, &monitor);
let noise_level = estimator.estimate_outside_reservoir();

println!("Noise level (outside reservoir): {:.2}", noise_level);
```

## Interpretation Guidelines

### 4D Response Types

| Response | Interpretation |
|----------|----------------|
| Positive amplitude | Hardening (water influx, pressure increase) |
| Negative amplitude | Softening (gas expansion, pressure depletion) |
| Time push-down | Velocity decrease (gas, pressure drop) |
| Time pull-up | Velocity increase (water, pressure increase) |

### Common Artifacts

| Artifact | Cause | Mitigation |
|----------|-------|------------|
| Stripes | Acquisition footprint | Pre-stack matching |
| Edge effects | Poor illumination | Mask edges |
| Time shifts | Near-surface changes | Static corrections |
| Amplitude scaling | Source strength variation | Calibration |

## Best Practices

1. **Baseline Quality**: Ensure base and monitor surveys are properly processed
2. **Time Alignment**: Apply careful time shifts before differencing
3. **Amplitude Calibration**: Match amplitude scales between surveys
4. **Noise Assessment**: Quantify non-repeatable noise level
5. **Rock Physics**: Link 4D response to reservoir changes
6. **Production Integration**: Correlate with well data for validation

## Example: Complete 4D Study

```rust
use sf_4d::*;

fn complete_4d_study(base_path: &str, monitor_path: &str, 
                     prod_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Load data
    let base = load_seismic(base_path)?;
    let monitor = load_seismic(monitor_path)?;
    let prod = ProductionData::from_csv(prod_path);
    
    // Create monitor
    let monitor_4d = TimeLapseMonitor::new(&base, &monitor);
    
    // Quality control
    let qc = RepeatabilityMetrics::compute(&base, &monitor);
    println!("QC: NRMS={:.2}%, Corr={:.3}", qc.nrms, qc.correlation);
    
    // Compute attributes
    let diff = monitor_4d.compute_difference();
    let nrms = monitor_4d.compute_nrms();
    let time_shifts = monitor_4d.compute_time_shifts(&TimeShiftConfig::default());
    
    // Save results
    save_attribute("diff.dat", &diff)?;
    save_attribute("nrms.dat", &nrms)?;
    save_attribute("timeshift.dat", &time_shifts)?;
    
    // Production correlation
    for well in prod.wells() {
        let well_nrms = extract_at_location(&nrms, prod.well_location(well));
        let corr = WellCorrelation::compute(&prod.production(well), &well_nrms);
        println!("{}: r={:.3} (p={:.4})", well, corr.coefficient, corr.p_value);
    }
    
    Ok(())
}
```

## References

- Lumley, D. (2010). 4D seismic monitoring of CO2 sequestration.
- Landrø, M. (2001). Discrimination between pressure and fluid saturation changes.
- Johnston, D. H. (2005). Practical applications of 4D seismic.

## See Also

- [Phase 2 Features Overview](PHASE_2_FEATURES.md)
- [QI & AVO Guide](QI_GUIDE.md)
- [GPU Acceleration Guide](GPU_ACCELERATION.md)
