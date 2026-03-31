# Seisly

**The Professional open-source seismic interpretation platform, powered by Rust.**

[![CI/CD](https://github.com/ajamj/seisly/actions/workflows/ci-cd.yml/badge.svg)](https://github.com/ajamj/seisly/actions/workflows/ci-cd.yml)
[![Latest Release](https://img.shields.io/github/v/release/ajamj/seisly)](https://github.com/ajamj/seisly/releases/latest)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.75+-orange.svg)](https://www.rust-lang.org)

**Seisly** (pronounced: /ˈsaɪzli/) is a modern subsurface interpretation workstation built for extreme performance, security, and extensibility.

---

## 🚀 Version 1.0.0 is Here!

Seisly has reached its production-grade milestone. 

### Key Highlights:
- 🏢 **Professional Workstation UI**: Multi-tab, dockable panel architecture with persistent layouts.
- 🛡️ **Secure AI Plugins**: Process-isolated Python worker model for safe, extensible deep learning.
- ⚡ **Out-of-Core Performance**: LRU brick caching allows handling massive seismic volumes beyond your RAM capacity.
- 🎨 **Modern Design System**: High-fidelity dark/light themes with a consistent 8px grid.
- 📈 **Full Interpretation Suite**: Horizon auto-tracking, RBF fault modeling, and well-seismic tie.

---

## 📦 Installation

### Download Installers
Pre-built binaries for all major platforms are available on the **[Releases Page](https://github.com/ajamj/seisly/releases/latest)**.

- **Windows:** Download the `.msi` installer.
- **macOS:** Download the `.pkg` installer.
- **Linux:** Download the `.deb` or `.tar.gz` package.

### Build from Source
If you prefer building from source, ensure you have the [Rust stable toolchain](https://rustup.rs) installed.

```bash
# Clone and build
git clone https://github.com/ajamj/seisly.git
cd seisly
cargo build --release --features python

# Launch the desktop workstation
cargo run --release --bin seisly-app --features python
```

---

## ✨ Features

- **Seismic Visualization**: High-performance 3D volume rendering and multi-slice viewing.
- **Horizon Interpretation**: Manual picking, BFS-based auto-tracking, and seed point expansion.
- **Fault Modeling**: Interactive sketch mode with real-time RBF surface generation.
- **Well Integration**: Full support for LAS 2.0/3.0, formation tops, and time-depth conversion.
- **Advanced Attributes**: 20+ CPU and GPU-accelerated seismic attributes (RMS, Frequency, Phase, etc.).
- **Python Extensibility**: Develop your own deep learning models using PyTorch/TensorFlow and run them as isolated Seisly plugins.

---

## 🏗️ Architecture

Seisly is organized into specialized crates for maximum modularity:

| Crate | Purpose |
|-------|---------|
| `seisly_app` | Main Desktop Workstation (egui + wgpu) |
| `seisly_core` | Core domain models and shared types |
| `seisly_io` | High-performance SEG-Y, LAS, and CSV I/O |
| `seisly_compute` | Interpretation algorithms and math kernels |
| `seisly_render` | Low-level WGPU 3D rendering engine |
| `seisly_plugin` | Secure Python/Rust plugin manager |
| `seisly_py_worker` | Isolated process for Python AI execution |

---

## 📚 Documentation

- **[📖 User Manual](docs/user-manual/SUMMARY.md)** - Comprehensive guide for geoscientists.
- **[📊 API Reference](https://ajamj.github.io/seisly/docs/api/)** - Rust and Python developer docs.
- **[🚀 Quick Start](docs/getting-started/quickstart.md)** - Get productive in 5 minutes.

---

## 🧪 Testing & Quality

Seisly maintains high standards for production readiness:

```bash
# Run all tests (requires Python environment for plugin tests)
cargo test --workspace --features python

# Run benchmarks
cargo bench --workspace
```

---

## 🤝 Contributing

We welcome contributions from geoscientists and engineers alike. Please see our **[Contributing Guidelines](CONTRIBUTING.md)** to get started.

## 📄 License

Seisly is licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

---
**Built with ❤️ for the Geoscience Community.**
