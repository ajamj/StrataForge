---
layout: home
title: Seisly - Rust-Powered Seismic Studio
nav_order: 1
description: The fastest, most accessible seismic interpretation platform
---

# Welcome to Seisly Documentation

**Seisly** (pronounced: /ˈsaɪzli/) is a modern, open-source seismic interpretation platform built in Rust.

> **🎯 Vision:** The fastest, most accessible seismic studio - from exploration to production.

---

## ⚡ Quick Stats

| Metric | Seisly | Petrel | OpendTect |
|--------|--------|--------|-----------|
| **Startup** | <2s | 30-60s | 10-20s |
| **Size** | <100MB | ~5GB | ~2GB |
| **License** | MIT (Free) | $10k+/yr | Open-core |
| **Engine** | Rust + GPU | .NET | C++ |

---

## 📚 Documentation Sections

### Getting Started

- [**Quick Start Guide**]({{ site.baseurl }}/QUICKSTART.md) - Get started in 5 minutes
- [**Installation**]({{ site.baseurl }}/#installation) - Install Seisly on your platform
- [**Project Format**]({{ site.baseurl }}/docs/project_format.md) - Understand the project structure

### Core Features

- [**Well-Seismic Tie**]({{ site.baseurl }}/docs/well_seismic_tie.md) - Integrate well logs with seismic
- [**Horizon Interpretation**]({{ site.baseurl }}/docs/blueprint.md#horizon-management) - Manual and auto-picking
- [**Fault Modeling**]({{ site.baseurl }}/docs/blueprint.md#fault-modeling) - Sketch and model faults
- [**Velocity Modeling**]({{ site.baseurl }}/docs/blueprint.md#velocity-modeling) - Time-depth conversion

### Advanced Topics

- [**Seismic Attributes**]({{ site.baseurl }}/docs/seismic_attributes.md) - GPU-accelerated attributes
- [**Quantitative Interpretation**]({{ site.baseurl }}/docs/QI_GUIDE.md) - AVO, FWI, elastic parameters
- [**4D Monitoring**]({{ site.baseurl }}/docs/4D_MONITORING.md) - Time-lapse seismic analysis
- [**ML Auto-Tracking**]({{ site.baseurl }}/docs/ml_auto_tracking.md) - CNN-based horizon tracking

### Developer Resources

- [**Architecture**]({{ site.baseurl }}/docs/architecture.md) - System architecture overview
- [**Plugin Development**]({{ site.baseurl }}/docs/plugin_development.md) - Build custom plugins
- [**GPU Acceleration**]({{ site.baseurl }}/docs/GPU_ACCELERATION.md) - wgpu compute shaders
- [**API Reference**]({{ site.baseurl }}/docs/api/README.md) - Rust API documentation

---

## 🚀 Quick Start

### Installation

```bash
# Clone repository
git clone https://github.com/ajamj/seisly.git
cd seisly

# Build release
cargo build --release

# Run application
cargo run --release --bin seisly-app
```

### Generate Synthetic Data

1. Launch Seisly GUI
2. Go to **Tools → Generate Synthetic Data**
3. Configure parameters:
   - Inlines: 100-500
   - Crosslines: 100-500
   - Samples: 512
   - Frequency: 35 Hz
4. Click **Generate**

---

## 🎯 Roadmap

### Phase 0 (v0.2.0) - Well-Seismic Workflow ✅
- Basic seismic visualization
- Well-seismic tie
- Horizon picking

### Phase 1 (v0.3.0) - ML + Attributes ✅
- Machine learning auto-tracking
- Seismic attributes (GPU)
- Plugin system

### Phase 2 (v0.4.0) - QI + 4D ✅
- Quantitative interpretation
- 4D time-lapse monitoring
- FWI (Full Waveform Inversion)

### Phase 3 (v0.5.0) - Production Tools ✅
- Enhanced tracking
- Well planning
- CCUS monitoring

### Phase 4 (v1.0.0) - Production Release 🚧
- Performance optimization
- Cloud integration
- Enterprise features

---

## 📦 Installation

### Windows

```powershell
# Using winget (recommended)
winget install seisly

# Or download installer
# https://github.com/ajamj/seisly/releases/latest
```

### Linux

```bash
# Ubuntu/Debian
sudo apt install seisly

# Arch Linux
yay -S seisly

# Or build from source
cargo build --release
```

### macOS

```bash
# Using Homebrew
brew install seisly

# Or build from source
cargo build --release
```

---

## 🤝 Community

- **GitHub:** https://github.com/ajamj/seisly
- **Discord:** [Join our server](https://discord.gg/seisly)
- **Twitter:** [@SeislyIO](https://twitter.com/SeislyIO)

---

## 📄 License

MIT OR Apache-2.0 - See [LICENSE](https://github.com/ajamj/seisly/blob/master/LICENSE) for details.
