# ML Auto-Tracking User Guide

## Introduction

The ML Auto-Tracking system provides automated horizon picking using Convolutional Neural Networks (CNNs). This guide covers everything from basic usage to advanced customization.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Core Concepts](#core-concepts)
3. [Basic Usage](#basic-usage)
4. [Advanced Features](#advanced-features)
5. [Model Training](#model-training)
6. [Best Practices](#best-practices)
7. [Troubleshooting](#troubleshooting)

## Getting Started

### Prerequisites

```bash
# Required dependencies
cargo add seisly_ml
cargo add ndarray          # Multi-dimensional arrays
cargo add ndarray-numpy    # NumPy interoperability
```

### Quick Example

```rust
use seisly_ml::{AutoTracker, HorizonCNN, TrackerConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load pre-trained model
    let model = HorizonCNN::load("models/horizon_cnn_v1.bin")?;
    
    // Configure tracker
    let config = TrackerConfig::default()
        .with_search_window(50)
        .with_confidence_threshold(0.8);
    
    // Create tracker
    let tracker = AutoTracker::new(model, config);
    
    // Track horizon from seed point
    let surface = tracker.track(&seismic_volume, 100, 200, 1500.0)?;
    
    println!("Tracked {} points", surface.len());
    Ok(())
}
```

## Core Concepts

### Horizon CNN

The HorizonCNN is a convolutional neural network specifically designed for seismic horizon detection. It analyzes local seismic patterns to identify horizon continuations.

**Architecture:**
```
Input: 64x64 seismic patch
  ↓
Conv2D (32 filters, 3x3) + ReLU
  ↓
Conv2D (64 filters, 3x3) + ReLU
  ↓
MaxPool (2x2)
  ↓
Conv2D (128 filters, 3x3) + ReLU
  ↓
Global Average Pooling
  ↓
Dense (128) + ReLU
  ↓
Output: Horizon presence probability
```

### AutoTracker

The AutoTracker is the main engine that orchestrates horizon tracking:

1. **Seed Point:** Starting point (IL, XL, TWT) provided by user
2. **Search Window:** Area around current horizon for next point search
3. **Propagation:** Iterative tracking in specified directions
4. **Confidence Scoring:** Quality measure for each tracked point

### Synthetic Data Generator

Generates realistic synthetic seismic data for model training:

```rust
use seisly_ml::data::SyntheticGenerator;

let generator = SyntheticGenerator::new();
let (seismic, horizons) = generator.generate(
    size=(512, 512, 256),
    num_horizons=5,
    noise_level=0.3
)?;
```

## Basic Usage

### Loading a Model

```rust
use seisly_ml::HorizonCNN;

// Load from file
let model = HorizonCNN::load("path/to/model.bin")?;

// Load from bytes
let model = HorizonCNN::from_bytes(model_bytes)?;

// Use pre-trained model
let model = HorizonCNN::pretrained()?;
```

### Tracking a Horizon

```rust
use seisly_ml::{AutoTracker, TrackerConfig};

let config = TrackerConfig::default()
    .with_search_window(50)        // Search ±50 samples
    .with_confidence_threshold(0.8) // Minimum confidence
    .with_max_gap(10)              // Max gap to interpolate
    .with_propagation_direction("bidirectional");

let tracker = AutoTracker::new(model, config);

// Track from single seed
let surface = tracker.track(&seismic, il, xl, twt)?;

// Track from multiple seeds
let seeds = vec![(il1, xl1, twt1), (il2, xl2, twt2)];
let surface = tracker.track_multi(&seismic, &seeds)?;
```

### Working with Surfaces

```rust
use seisly_ml::HorizonSurface;

let surface: HorizonSurface = tracker.track(&seismic, il, xl, twt)?;

// Access tracked points
for point in surface.iter() {
    println!("IL: {}, XL: {}, TWT: {:.2}, Confidence: {:.2}",
             point.il, point.xl, point.twt, point.confidence);
}

// Export to XYZ
surface.to_xyz("output.xyz")?;

// Export to ZMAP
surface.to_zmap("output.zmap")?;

// Get statistics
let stats = surface.statistics();
println!("Min TWT: {:.2}, Max TWT: {:.2}", stats.min_twt, stats.max_twt);
```

## Advanced Features

### Custom Search Strategies

```rust
use seisly_ml::search::{SearchStrategy, AdaptiveSearch};

// Adaptive search window based on confidence
let strategy = AdaptiveSearch::new()
    .with_min_window(20)
    .with_max_window(100)
    .with_expansion_rate(1.5);

tracker.set_search_strategy(strategy);
```

### Quality Control

```rust
use seisly_ml::qc::QualityMetrics;

let metrics = tracker.compute_quality(&surface, &seismic);

// Check overall quality
if metrics.overall_score < 0.7 {
    eprintln!("Warning: Low quality horizon (score: {:.2})", metrics.overall_score);
}

// Identify problematic areas
let low_confidence = surface.filter_by_confidence(0.5);
println!("{} points with low confidence", low_confidence.len());
```

### Batch Processing

```rust
use seisly_ml::batch::BatchTracker;

let batch = BatchTracker::new(tracker);

let horizons = vec![
    ("Horizon_A", seed_a),
    ("Horizon_B", seed_b),
    ("Horizon_C", seed_c),
];

let results = batch.process(&seismic, &horizons)?;

for (name, surface) in results {
    surface.to_xyz(&format!("{}.xyz", name))?;
}
```

### Parallel Tracking

```rust
use seisly_ml::parallel::ParallelTracker;

let parallel = ParallelTracker::new(tracker, num_threads=4);

let surfaces = parallel.track_all(&seismic, &seeds)?;
```

## Model Training

### Preparing Training Data

```rust
use seisly_ml::data::{TrainingDataset, DataAugmentation};

// Create dataset
let mut dataset = TrainingDataset::new();

// Add real data
dataset.add_seismic_volume("volume1.segy", "horizons1.json")?;
dataset.add_seismic_volume("volume2.segy", "horizons2.json")?;

// Add synthetic data
let generator = SyntheticGenerator::new();
for i in 0..100 {
    let (seismic, horizons) = generator.generate_random()?;
    dataset.add_synthetic(seismic, horizons)?;
}

// Apply augmentation
let augment = DataAugmentation::default()
    .with_rotation(true)
    .with_flip(true)
    .with_noise(0.1)
    .with_time_shift(5);

dataset.augment(augment)?;

// Save dataset
dataset.save("training_data.bin")?;
```

### Training Pipeline

```rust
use seisly_ml::training::{ModelTrainer, TrainingConfig, Optimizer};

let config = TrainingConfig::default()
    .with_epochs(100)
    .with_batch_size(32)
    .with_learning_rate(0.001)
    .with_optimizer(Optimizer::Adam)
    .with_early_stopping(true)
    .with_validation_split(0.2);

let mut trainer = ModelTrainer::new(config);

// Load dataset
let dataset = TrainingDataset::load("training_data.bin")?;

// Train model
let model = trainer.train(&dataset)?;

// Save trained model
model.save("models/my_horizon_cnn.bin")?;
```

### Monitoring Training

```rust
use seisly_ml::training::TrainingCallback;

struct MyCallback;

impl TrainingCallback for MyCallback {
    fn on_epoch_end(&mut self, epoch: usize, metrics: TrainingMetrics) {
        println!("Epoch {}: loss={:.4}, val_loss={:.4}, acc={:.2}%",
                 epoch, metrics.loss, metrics.val_loss, metrics.accuracy * 100.0);
    }
}

trainer.add_callback(MyCallback);
trainer.train(&dataset)?;
```

### Hyperparameter Tuning

```rust
use seisly_ml::tuning::HyperparameterSearch;

let search = HyperparameterSearch::new()
    .with_learning_rates(&[0.0001, 0.001, 0.01])
    .with_batch_sizes(&[16, 32, 64])
    .with_architectures(&["small", "medium", "large"]);

let best_config = search.run(&dataset, cross_validation=5)?;
println!("Best config: {:?}", best_config);
```

## Best Practices

### Seed Point Selection

1. **High Confidence Areas:** Choose seed points in areas with clear, continuous reflections
2. **Avoid Faults:** Keep seeds away from fault zones initially
3. **Multiple Seeds:** Use multiple seeds for complex structures
4. **Quality Control:** Manually verify seed point accuracy

### Parameter Tuning

```rust
// Conservative tracking (high accuracy)
let config = TrackerConfig::default()
    .with_search_window(30)
    .with_confidence_threshold(0.9)
    .with_max_gap(5);

// Aggressive tracking (more coverage)
let config = TrackerConfig::default()
    .with_search_window(80)
    .with_confidence_threshold(0.6)
    .with_max_gap(20);
```

### Performance Optimization

```rust
// Use GPU acceleration (if available)
let tracker = AutoTracker::new(model, config)
    .with_device(Device::GPU);

// Enable multi-threading
tracker.set_num_threads(4);

// Process in chunks for large volumes
let chunks = seismic.chunk(256, 256, 128);
for chunk in chunks {
    tracker.track(&chunk, ...)?;
}
```

## Troubleshooting

### Low Tracking Quality

**Problem:** Horizon has many low-confidence points

**Solutions:**
1. Reduce search window size
2. Increase confidence threshold
3. Add more seed points
4. Check seismic data quality

### Tracking Jumps to Wrong Horizon

**Problem:** Tracker follows incorrect reflection

**Solutions:**
1. Reduce search window
2. Increase confidence threshold
3. Use adaptive search strategy
4. Add intermediate seed points

### Slow Performance

**Problem:** Tracking takes too long

**Solutions:**
1. Enable GPU acceleration
2. Reduce search window
3. Use parallel tracking
4. Process in smaller chunks

### Model Not Converging

**Problem:** Training loss doesn't decrease

**Solutions:**
1. Check learning rate (try 0.0001, 0.001, 0.01)
2. Increase dataset size
3. Add data augmentation
4. Verify data normalization

## Python API

```python
from seisly.ml import AutoTracker, HorizonCNN, TrackerConfig

# Load model
model = HorizonCNN.load("model.bin")

# Configure
config = TrackerConfig(
    search_window=50,
    confidence_threshold=0.8,
    max_gap=10
)

# Create tracker
tracker = AutoTracker(model, config)

# Track
surface = tracker.track(seismic, il=100, xl=200, twt=1500)

# Export
surface.to_xyz("horizon.xyz")
surface.to_zmap("horizon.zmap")

# Batch tracking
seeds = [
    ("H1", 100, 200, 1500),
    ("H2", 100, 200, 1800),
]
results = tracker.track_batch(seismic, seeds)
```

## API Reference

### AutoTracker

| Method | Description |
|--------|-------------|
| `new(model, config)` | Create new tracker |
| `track(seismic, il, xl, twt)` | Track from single seed |
| `track_multi(seismic, seeds)` | Track from multiple seeds |
| `set_search_strategy(strategy)` | Set custom search strategy |
| `compute_quality(surface, seismic)` | Compute quality metrics |

### TrackerConfig

| Method | Description | Default |
|--------|-------------|---------|
| `with_search_window(n)` | Search window size | 50 |
| `with_confidence_threshold(t)` | Min confidence | 0.8 |
| `with_max_gap(g)` | Max interpolation gap | 10 |
| `with_propagation_direction(d)` | Direction | "bidirectional" |

### HorizonSurface

| Method | Description |
|--------|-------------|
| `iter()` | Iterate over points |
| `len()` | Number of points |
| `to_xyz(path)` | Export to XYZ |
| `to_zmap(path)` | Export to ZMAP |
| `statistics()` | Get statistics |
| `filter_by_confidence(min_conf)` | Filter by confidence |
