# GitHub Release Notes for v0.3.0

## Copy-paste ini ke GitHub Release:

---

## 🎉 StrataForge v0.3.0 - Phase 1 Complete!

### Major Features

#### 🤖 Machine Learning & Auto-Tracking
- CNN-based horizon auto-tracking
- Synthetic training data generation  
- Training pipeline with early stopping
- Model checkpoint saving

#### 📊 Seismic Attributes (20 Total)

**Amplitude (10):**
- RMS, Mean, Max, Min, Std Dev
- Energy, Average Energy
- Absolute, Max Absolute, Skewness

**Frequency (10):**
- Instantaneous, Dominant, Peak, Mean
- Bandwidth, Spectral Blue/Red
- Thin Bed, Absorption, Wavelet Phase, Instantaneous Phase

#### 🔌 Plugin System
- Rust plugin API
- Python bindings (PyO3)
- Command-based execution
- Plugin manager

#### 🐍 Python Integration
- Full Python package (`stratforge`)
- maturin build system
- Example scripts included

### Statistics
- **30+ new files**
- **3,000+ lines of code**
- **100+ tests**
- **2,400+ lines of documentation**

### Installation

#### Rust (Cargo)
```bash
cargo install --path crates/sf_cli
```

#### Python (pip - development)
```bash
pip install maturin
cd python
maturin develop
```

### Documentation
- [Phase 1 Features](docs/PHASE_1_FEATURES.md)
- [ML Auto-Tracking Guide](docs/ml_auto_tracking.md)
- [Seismic Attributes Reference](docs/seismic_attributes.md)
- [Plugin Development](docs/plugin_development.md)

### What's Changed
- Branch strategy: `master` is now default
- Parallel development workflow
- Improved error handling

### Contributors
- @ajamj

**Full Changelog:** https://github.com/ajamj/StrataForge/compare/v0.2.0...v0.3.0

---

## Release URL setelah dibuat:
https://github.com/ajamj/StrataForge/releases/tag/v0.3.0
