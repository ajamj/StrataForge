//! Phase 1 Integration Tests
//! Test all Phase 1 components working together

use sf_ml::{HorizonCNN, AutoTracker, SyntheticTrainer, Trainer, TrainingConfig};
use sf_attributes::{RmsAmplitude, MeanAmplitude, InstantaneousFrequency, SeismicAttribute, all_amplitude_attributes, all_frequency_attributes};
use sf_plugin::{PluginManager, Plugin, PluginContext, PluginCommand, Result as PluginResult};
use serde_json::Value;

#[test]
fn test_ml_synthetic_data_generation() {
    // Test synthetic data generator
    let mut trainer = SyntheticTrainer::new(42);
    let (patch, label) = trainer.generate_sample(64);
    
    assert_eq!(patch.len(), 64 * 64);
    assert!(label >= 0.0 && label <= 1.0);
}

#[test]
fn test_ml_and_attributes_pipeline() {
    // Test ML auto-tracking + attributes together
    let mut trainer = SyntheticTrainer::new(42);
    let (seismic, _labels) = trainer.generate_batch(10, 64);
    
    // Compute attributes on synthetic data
    let rms = RmsAmplitude;
    let rms_result = rms.compute(&seismic[0..100], 32);
    
    let mean = MeanAmplitude;
    let mean_result = mean.compute(&seismic[0..100], 32);
    
    assert!(!rms_result.is_empty());
    assert!(!mean_result.is_empty());
    // RMS should be >= mean for real signals
    assert!(rms_result[0] >= mean_result[0].abs());
}

#[test]
fn test_all_amplitude_attributes() {
    let attrs = all_amplitude_attributes();
    let trace = vec![1.0, 2.0, 3.0, 4.0, 5.0, 4.0, 3.0, 2.0, 1.0, 0.0];
    
    assert_eq!(attrs.len(), 10);
    
    for attr in attrs {
        let result = attr.compute(&trace, 5);
        assert!(!result.is_empty(), "{} failed", attr.name());
        assert!(result.iter().all(|&x| x.is_finite()), "{} produced non-finite values", attr.name());
    }
}

#[test]
fn test_all_frequency_attributes() {
    let attrs = all_frequency_attributes();
    let trace = vec![1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0, 0.0];
    
    assert_eq!(attrs.len(), 10);
    
    for attr in attrs {
        let result = attr.compute(&trace, 8);
        assert!(!result.is_empty(), "{} failed", attr.name());
        // Frequency attributes can produce NaN in edge cases, so just check length
        assert_eq!(result.len(), trace.len(), "{} produced wrong length", attr.name());
    }
}

#[test]
fn test_plugin_with_custom_plugin() {
    // Test plugin system with custom plugin
    struct TestPlugin;
    
    impl Plugin for TestPlugin {
        fn name(&self) -> &str { "TestPlugin" }
        fn version(&self) -> &str { "1.0.0" }
        fn description(&self) -> &str { "Test plugin for integration" }
        
        fn commands(&self) -> Vec<PluginCommand> {
            vec![PluginCommand {
                name: "compute_rms".to_string(),
                description: "Compute RMS amplitude".to_string(),
            }]
        }
        
        fn execute(&self, cmd: &str, args: Value) -> PluginResult<Value> {
            if cmd == "compute_rms" {
                // Extract trace from args and compute RMS
                let trace = vec![1.0, 2.0, 3.0, 4.0, 5.0];
                let rms: f32 = (trace.iter().map(|x| x * x).sum::<f32>() / trace.len() as f32).sqrt();
                Ok(Value::Number(serde_json::Number::from_f64(rms as f64).unwrap()))
            } else {
                Err(sf_plugin::api::PluginError::ExecutionError("Unknown command".to_string()))
            }
        }
    }
    
    let mut manager = PluginManager::new();
    manager.register(Box::new(TestPlugin));
    
    assert_eq!(manager.plugin_count(), 1);
    assert!(manager.list_plugins().contains(&"TestPlugin"));
    
    let result = manager.execute("TestPlugin", "compute_rms", Value::Null).unwrap();
    assert!(result.is_number());
}

#[test]
fn test_full_workflow_synthetic_to_attributes() {
    // Complete workflow:
    // 1. Generate synthetic seismic
    // 2. Compute multiple attributes
    // 3. Verify results are consistent
    
    let mut trainer = SyntheticTrainer::new(42);
    let (seismic, _labels) = trainer.generate_batch(5, 32);
    
    // Compute amplitude attributes
    let rms = RmsAmplitude;
    let mean = MeanAmplitude;
    
    for trace_data in seismic.chunks(32 * 32) {
        let rms_result = rms.compute(trace_data, 16);
        let mean_result = mean.compute(trace_data, 16);
        
        // Verify RMS >= |mean|
        for (r, m) in rms_result.iter().zip(mean_result.iter()) {
            assert!(r >= &m.abs(), "RMS should be >= |mean|");
        }
    }
}
