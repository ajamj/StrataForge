//! Phase 2 4D Integration Tests

use sf_4d::{TimeLapseAnalysis, DifferenceMap, AnomalyDetector, ProductionTimeline, ProductionData};

#[test]
fn test_full_4d_workflow() {
    // Create synthetic base and monitor surveys
    let base = vec![1.0; 10000];
    let mut monitor = vec![1.0; 10000];
    
    // Add 4D signal (production effect)
    for i in 4000..5000 {
        monitor[i] = 1.5; // Brightening
    }
    
    let analysis = TimeLapseAnalysis::new(base, monitor, 0.004);
    
    // Compute differences
    let diff = analysis.difference();
    let rms = analysis.rms_difference();
    let nrms = analysis.nrms();
    
    assert!(rms > 0.0, "Should detect 4D signal");
    assert!(nrms > 5.0, "NRMS should be significant");
    
    // Detect anomalies
    let map = DifferenceMap::from_timelapse(&analysis);
    let detector = AnomalyDetector::new(0.3);
    let anomalies = detector.detect(&map);
    
    assert!(!anomalies.is_empty(), "Should detect anomalies");
}

#[test]
fn test_4d_no_change() {
    // Test with identical surveys (no 4D signal)
    let base = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let monitor = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    
    let analysis = TimeLapseAnalysis::new(base, monitor, 0.004);
    
    assert_eq!(analysis.rms_difference(), 0.0);
    assert_eq!(analysis.nrms(), 0.0);
}

#[test]
fn test_production_integration() {
    let mut timeline = ProductionTimeline::new("Well-1".to_string());
    timeline.add(ProductionData {
        well_name: "Well-1".to_string(),
        date: "2024-01".to_string(),
        oil_rate: 1000.0,
        gas_rate: 500.0,
        water_rate: 100.0,
        pressure: 3000.0,
    });
    timeline.add(ProductionData {
        well_name: "Well-1".to_string(),
        date: "2024-02".to_string(),
        oil_rate: 900.0,
        gas_rate: 600.0,
        water_rate: 150.0,
        pressure: 2800.0,
    });
    
    let (oil, gas, water) = timeline.cumulative();
    assert!((oil - 1900.0).abs() < 0.01);
    assert!((gas - 1100.0).abs() < 0.01);
    
    // Test water cut
    let wc = timeline.data[0].water_cut();
    assert!((wc - 0.09).abs() < 0.01); // ~9% water cut
}

#[test]
fn test_4d_multi_survey_comparison() {
    // Simulate multiple monitor surveys (time series)
    let base = vec![1.0; 1000];
    let monitors = vec![
        vec![1.0; 1000],           // Baseline
        vec![1.1; 1000],           // +10% change
        vec![1.2; 1000],           // +20% change
    ];
    
    let mut nrms_values = Vec::new();
    
    for monitor in monitors {
        let analysis = TimeLapseAnalysis::new(base.clone(), monitor, 0.004);
        nrms_values.push(analysis.nrms());
    }
    
    // NRMS should increase over time
    assert!(nrms_values[1] > nrms_values[0]);
    assert!(nrms_values[2] > nrms_values[1]);
}

#[test]
fn test_anomaly_classification() {
    let detector = AnomalyDetector::new(0.3);
    
    // Test positive anomaly (hardening)
    let pos_class = detector.classify_anomaly(0.5);
    assert!(pos_class.contains("Hardening"));
    
    // Test negative anomaly (softening)
    let neg_class = detector.classify_anomaly(-0.5);
    assert!(neg_class.contains("Softening"));
}
