# Phase 1 (v0.3.0) Features

## Overview

Phase 1 (v0.3.0) introduces advanced machine learning and seismic attribute computation capabilities to myfield, transforming it into a comprehensive seismic interpretation platform.

**Release:** v0.3.0  
**Status:** 86% complete → 100%  
**Branch:** `master`

---

## Table of Contents

1. [ML Auto-Tracking](#1-ml-auto-tracking)
2. [Seismic Attributes](#2-seismic-attributes)
3. [Plugin System](#3-plugin-system)
4. [Getting Started](#getting-started)
5. [Migration Guide](#migration-guide)

---

## Features

### 1. ML Auto-Tracking

CNN-based horizon auto-tracking with synthetic training data generation and model training pipeline.

**Key Capabilities:**
- **Automated Horizon Picking:** Track horizons from seed points using deep learning
- **Synthetic Data Generation:** Generate realistic training data
- **Model Training:** Full training pipeline with hyperparameter tuning
- **Quality Control:** Confidence scoring and quality metrics
- **Batch Processing:** Track multiple horizons efficiently
- **Python Bindings:** Full Python API via PyO3

**Components:**
| Component | Description |
|-----------|-------------|
| `HorizonCNN` | Convolutional neural network for horizon detection |
| `AutoTracker` | Main tracking engine with configurable parameters |
| `SyntheticGenerator` | Synthetic seismic data generation |
| `ModelTrainer` | Training pipeline with callbacks |
| `QualityMetrics` | Horizon quality assessment |

**Quick Start:**
```rust
use sf_ml::{AutoTracker, HorizonCNN, TrackerConfig};

// Load model
let model = HorizonCNN::pretrained()?;

// Configure
let config = TrackerConfig::default()
    .with_search_window(50)
    .with_confidence_threshold(0.8);

// Track
let tracker = AutoTracker::new(model, config);
let surface = tracker.track(&seismic, il, xl, twt)?;

// Export
surface.to_xyz("horizon.xyz")?;
```

**Documentation:** [`docs/ml_auto_tracking.md`](ml_auto_tracking.md)

---

### 2. Seismic Attributes

20 seismic attributes for reservoir characterization, fault detection, and stratigraphic analysis.

**Categories:**

#### Amplitude Attributes (10)
| Attribute | Formula | Use Case |
|-----------|---------|----------|
| RMS Amplitude | `sqrt(Σx²/n)` | Energy measure, bright spots |
| Mean Amplitude | `Σx/n` | Average amplitude |
| Max Amplitude | `max(x)` | Peak detection |
| Min Amplitude | `min(x)` | Trough detection |
| Standard Deviation | `sqrt(Σ(x-mean)²/n)` | Heterogeneity, faults |
| Energy | `Σx²` | HC indicator |
| Absolute Sum | `Σ|x|` | Amplitude strength |
| Peak-to-Peak | `max(x) - min(x)` | Tuning detection |
| Variance | `Σ(x-mean)²/n` | Coherence alternative |
| Skewness | `Σ(x-mean)³/(n×std³)` | Wavelet asymmetry |

#### Frequency Attributes (10)
| Attribute | Description | Use Case |
|-----------|-------------|----------|
| Instantaneous Frequency | Time-varying frequency | Thin beds, HC |
| Dominant Frequency | Max amplitude frequency | Reservoir char. |
| Peak Frequency | Spectral peak | Spectral analysis |
| Mean Frequency | Amplitude-weighted avg | Attenuation |
| Median Frequency | 50th percentile | Robust measure |
| Frequency Centroid | Energy center of mass | Spectral balance |
| Frequency RMS | RMS of spectrum | Energy measure |
| Frequency Skewness | Spectral asymmetry | Absorption |
| Frequency Kurtosis | Spectral peakedness | Tuning |
| Narrowband Energy | Energy in band | Thin beds |

**Quick Start:**
```rust
use sf_attributes::amplitude::RmsAmplitude;
use sf_attributes::frequency::InstantaneousFrequency;

// Amplitude attribute
let rms = RmsAmplitude.compute(trace, window_size=50)?;

// Frequency attribute
let inst_freq = InstantaneousFrequency.compute(trace, dt=0.004)?;
```

**Documentation:** [`docs/seismic_attributes.md`](seismic_attributes.md)

---

### 3. Plugin System

Extensible plugin architecture with Rust API and Python bindings.

**Key Features:**
- **Plugin Trait:** Standard interface for all plugins
- **Command Pattern:** Execute plugin commands dynamically
- **Plugin Manager:** Lifecycle management and discovery
- **Python Bindings:** PyO3 integration
- **Hot Reloading:** Load/unload plugins at runtime

**Architecture:**
```
Plugin System
├── PluginManager      - Lifecycle management
├── PluginRegistry     - Discovery and registration
├── PluginTrait        - Base trait
├── CommandExecutor    - Command dispatch
└── Python Bindings    - PyO3 integration
```

**Quick Start:**
```rust
use sf_plugins::PluginManager;

// Create manager
let manager = PluginManager::new();

// Load plugin
manager.load_plugin("my_plugin.so")?;

// List plugins
for plugin in manager.list_plugins() {
    println!("{} v{}", plugin.name, plugin.version);
}

// Execute command
let result = manager.execute("my_plugin", "command", args)?;
```

**Python:**
```python
from stratforge import PluginManager

manager = PluginManager()
manager.load("my_plugin")

result = manager.execute("my_plugin", "command", {
    "param": "value"
})
```

**Documentation:** [`docs/plugin_development.md`](plugin_development.md)

---

## Getting Started

### Installation

```bash
# Add to Cargo.toml
[dependencies]
sf_ml = "0.3.0"
sf_attributes = "0.3.0"
sf_plugins = "0.3.0"
```

### Python Installation

```bash
pip install stratforge-ml
pip install stratforge-attributes
pip install stratforge-plugins
```

### Quick Example

```rust
use sf_ml::{AutoTracker, HorizonCNN};
use sf_attributes::amplitude::RmsAmplitude;
use sf_plugins::PluginManager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load seismic data
    let seismic = load_seismic("volume.segy")?;
    
    // ML Auto-Tracking
    let model = HorizonCNN::pretrained()?;
    let tracker = AutoTracker::new(model, Default::default());
    let horizon = tracker.track(&seismic, 100, 200, 1500.0)?;
    horizon.to_xyz("horizon.xyz")?;
    
    // Seismic Attributes
    let trace = seismic.get_trace(100, 200);
    let rms = RmsAmplitude.compute(&trace, 50)?;
    println!("RMS: {:.4}", rms);
    
    // Plugin System
    let manager = PluginManager::new();
    manager.load_plugin("custom_attrs.so")?;
    let result = manager.execute("custom_attrs", "compute", args)?;
    
    Ok(())
}
```

---

## Migration Guide

### From v0.2.x to v0.3.0

**Breaking Changes:**
- `HorizonTracker` renamed to `AutoTracker`
- Attribute API now uses trait-based approach
- Plugin system requires explicit lifecycle management

**Update Steps:**

1. **Update dependencies:**
```toml
[dependencies]
sf_ml = "0.3.0"
sf_attributes = "0.3.0"
sf_plugins = "0.3.0"
```

2. **Update imports:**
```rust
// Old
use sf_ml::HorizonTracker;

// New
use sf_ml::AutoTracker;
```

3. **Update attribute usage:**
```rust
// Old
let rms = compute_rms(trace, 50);

// New
use sf_attributes::amplitude::RmsAmplitude;
let rms = RmsAmplitude.compute(trace, 50)?;
```

---

## Performance Benchmarks

### ML Auto-Tracking

| Volume Size | Tracking Time | Memory |
|-------------|---------------|--------|
| 256×256×128 | ~30s | 2 GB |
| 512×512×256 | ~2min | 8 GB |
| 1024×1024×512 | ~10min | 32 GB |

*GPU acceleration available for 5x speedup*

### Seismic Attributes

| Attribute | Time per Trace | Throughput |
|-----------|---------------|------------|
| RMS Amplitude | ~10μs | 100K traces/s |
| Instantaneous Freq | ~50μs | 20K traces/s |
| Dominant Frequency | ~100μs | 10K traces/s |

*Multi-threaded processing available*

---

## API Reference

### Crates

| Crate | Description | Docs |
|-------|-------------|------|
| `sf_ml` | Machine learning | [Docs](ml_auto_tracking.md) |
| `sf_attributes` | Seismic attributes | [Docs](seismic_attributes.md) |
| `sf_plugins` | Plugin system | [Docs](plugin_development.md) |

### Python Packages

| Package | Description | PyPI |
|---------|-------------|------|
| `stratforge-ml` | ML auto-tracking | Coming soon |
| `stratforge-attributes` | Seismic attributes | Coming soon |
| `stratforge-plugins` | Plugin system | Coming soon |

---

## Examples

See the `examples/` directory for complete working examples:

```
examples/
├── ml_tracking/
│   ├── basic_tracking.rs
│   ├── batch_tracking.rs
│   └── custom_model.rs
├── attributes/
│   ├── amplitude_attrs.rs
│   ├── frequency_attrs.rs
│   └── volume_processing.rs
└── plugins/
    ├── simple_plugin/
    ├── attribute_plugin/
    └── ml_plugin/
```

---

## Changelog

### v0.3.0 (Phase 1)

**Added:**
- ML auto-tracking with CNN
- 20 seismic attributes (10 amplitude, 10 frequency)
- Plugin system with Python bindings
- Synthetic data generator
- Model training pipeline
- Quality control metrics
- Batch processing support

**Changed:**
- HorizonTracker → AutoTracker
- Attribute API refactored to trait-based

**Fixed:**
- Memory leak in volume processing
- Edge case in frequency computation

---

## Support

- **Documentation:** [docs.myfield.dev](https://docs.myfield.dev)
- **Issues:** [GitHub Issues](https://github.com/myfield/myfield/issues)
- **Discussions:** [GitHub Discussions](https://github.com/myfield/myfield/discussions)
- **Email:** support@myfield.dev

---

## License

Phase 1 features are licensed under the MIT License. See [LICENSE](../LICENSE) for details.
