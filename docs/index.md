---
# StrataForge Documentation Site
# See: https://ajamj.github.io/StrataForge

title: StrataForge
description: Open-source subsurface interpretation and modeling platform
theme: jekyll-theme-cayman

# Navigation
navigation:
  - title: Home
    url: /
  - title: Quick Start
    url: /QUICKSTART.html
  - title: Blueprint
    url: /docs/blueprint.html
  - title: Phase 0
    url: /docs/superpowers/specs/2026-03-29-phase-0-well-seismic-design.html
  - title: Phase 1
    url: /docs/PHASE_1_FEATURES.html
  - title: Phase 2
    url: /docs/PHASE_2_FEATURES.html
  - title: Phase 3
    url: /docs/superpowers/plans/2026-03-29-auto-tracking-engine.html
  - title: API Docs
    url: /docs/api/
  - title: GitHub
    url: https://github.com/ajamj/StrataForge

# Features
features:
  - title: 🚀 Fast Performance
    description: Rust-powered for lightning-fast seismic interpretation
  - title: 🎯 ML-Powered
    description: CNN-based auto-tracking and fault detection
  - title: 🔌 Extensible
    description: Plugin system with Python bindings
  - title: 📊 Advanced Analytics
    description: QI, AVO, FWI, and 4D monitoring

# Quick links
quick_links:
  - title: Installation Guide
    url: /README.html#installation
  - title: Synthetic Data
    url: /docs/synthetic_data.html
  - title: Well-Seismic Tie
    url: /docs/well_seismic_tie.html
  - title: Seismic Attributes
    url: /docs/seismic_attributes.html

# Social
social:
  github: https://github.com/ajamj/StrataForge
  license: MIT OR Apache-2.0

---

# Welcome to StrataForge

**Rust-powered Seismic Studio – Lightning fast, fully open, no license hell.**

[![CI/CD](https://github.com/ajamj/StrataForge/actions/workflows/ci-cd.yml/badge.svg)](https://github.com/ajamj/StrataForge/actions/workflows/ci-cd.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

## What is StrataForge?

StrataForge is a modern, reproducible platform for geoscientists and engineers to analyze subsurface data from exploration to production.

## Key Features

### 🎨 Seismic Interpretation
- Interactive horizon picking
- Fault modeling with RBF surfaces
- Auto-tracking with ML
- Multi-volume visualization

### 📈 Quantitative Interpretation
- AVO analysis
- Full Waveform Inversion (FWI)
- Elastic parameter estimation
- DHI scoring

### 🌊 4D Monitoring
- Time-lapse seismic analysis
- Production data integration
- CCUS monitoring
- Reservoir surveillance

### 🤖 Machine Learning
- U-Net based auto-tracking
- Fault detection with CNN
- Synthetic data generation
- Transfer learning

## Quick Start

### Installation

```bash
# Clone repository
git clone https://github.com/ajamj/StrataForge.git
cd StrataForge

# Build release
cargo build --release

# Run application
cargo run --release --bin sf-app
```

### Generate Synthetic Data

1. Launch StrataForge
2. Go to **Tools → Generate Synthetic Data**
3. Configure parameters:
   - Inlines: 100-500
   - Crosslines: 100-500
   - Samples: 512
   - Frequency: 35 Hz
4. Click **Generate**

### Import Your Data

```bash
# Create project
sf init --name "MyField" --crs 32648

# Import seismic
sf import --project MyField.sf segy volume.segy

# Import wells
sf import --project MyField.sf las --well "Well-1" well1.las
```

## Documentation

### User Guides
- [Quick Start Guide](QUICKSTART.md)
- [Well-Seismic Tie](docs/well_seismic_tie.md)
- [Seismic Attributes](docs/seismic_attributes.md)
- [ML Auto-Tracking](docs/ml_auto_tracking.md)
- [4D Monitoring](docs/4d_monitoring.md)

### Developer Guides
- [Architecture Overview](docs/architecture.md)
- [Plugin Development](docs/plugin_development.md)
- [GPU Acceleration](docs/GPU_ACCELERATION.md)
- [API Reference](docs/api/)

### Phase Documentation
- [Phase 0: Well-Seismic Workflow](docs/superpowers/specs/2026-03-29-phase-0-well-seismic-design.md)
- [Phase 1: Advanced Features](docs/PHASE_1_FEATURES.md)
- [Phase 2: QI & 4D](docs/PHASE_2_FEATURES.md)
- [Phase 3: Enhanced Tracking](docs/superpowers/plans/2026-03-29-auto-tracking-engine.md)

## Version History

| Version | Date | Status | Key Features |
|---------|------|--------|--------------|
| 0.5.0 | 2026-03-29 | 🎉 Latest | Deep Learning, FWI, Production Tools |
| 0.4.0 | 2026-03-29 | ✅ Stable | QI, 4D, GPU Acceleration |
| 0.3.0 | 2026-03-29 | ✅ Stable | ML, Attributes, Plugins |
| 0.2.0 | 2026-03-28 | ✅ Stable | Well-Seismic Workflow |
| 0.1.1 | 2026-03-20 | ✅ Stable | Core Interpretation |

## Community

- **GitHub Issues**: Report bugs and feature requests
- **Discussions**: Ask questions and share ideas
- **Discord**: Real-time chat (coming soon)

## License

MIT OR Apache-2.0

---

**Built with ❤️ using Rust**
