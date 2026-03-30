# Seisly API Documentation

**Auto-generated Rust API documentation for all Seisly crates.**

*Generated: 2026-03-30*

---

## 📦 Core Crates

### [seisly_core](./seisly_core/index.html)
Domain models and types for seismic interpretation.
- Well, Horizon, Fault models
- Surface and trajectory types
- CRS support

### [seisly_io](./seisly_io/index.html)
File I/O operations for seismic and well data.
- SEG-Y reader/writer
- LAS 2.0/3.0 parser
- CSV, XYZ exporters

### [seisly_compute](./seisly_compute/index.html)
Seismic computation algorithms.
- Interpolation (RBF)
- Tracking algorithms
- Volumetrics
- Well tie

### [seisly_render](./seisly_render/index.html)
GPU-accelerated rendering with wgpu.
- 3D volume rendering
- Mesh rendering
- Shader modules

### [seisly_storage](./seisly_storage/index.html)
Data persistence layer.
- SQLite database
- Blob storage
- Project management

---

## 📊 Analysis Crates

### [seisly_attributes](./seisly_attributes/index.html)
Seismic attribute computation.
- Amplitude attributes (RMS, Mean, Max)
- Frequency attributes
- GPU-accelerated variants

### [seisly_qi](./seisly_qi/index.html)
Quantitative Interpretation.
- AVO analysis
- Elastic parameter estimation
- Fluid substitution

### [seisly_4d](./seisly_4d/index.html)
4D time-lapse seismic monitoring.
- Difference computation
- Production data integration
- Timelapse attributes

### [seisly_fwi](./seisly_fwi/index.html)
Full Waveform Inversion.
- Acoustic FWI
- Elastic FWI
- Gradient computation

### [seisly_tracking](./seisly_tracking/index.html)
Horizon auto-tracking.
- Multi-horizon tracking
- Fault-guided tracking
- Quality control metrics

---

## 🤖 Machine Learning Crates

### [seisly_ml](./seisly_ml/index.html)
Machine learning for seismic interpretation.
- CNN-based auto-tracking
- Synthetic seismogram generation
- Training utilities

### [seisly_ml_deep](./seisly_ml_deep/index.html)
Deep learning models.
- U-Net architecture
- Fault detection
- Deep learning training

---

## 🛠️ Application Crates

### [seisly_app](./seisly_app/index.html)
Desktop GUI application (egui/eframe).
- Main application
- Widgets (viewport, panels)
- Interpretation state management

### [seisly_cli](./seisly_cli/index.html)
Command-line interface.
- Project management
- Import/export commands
- Batch processing

### [seisly_plugin](./seisly_plugin/index.html)
Plugin system.
- Rust plugin API
- Python plugin support (PyO3)
- Plugin manager

---

## 🗺️ Specialized Crates

### [seisly_crs](./seisly_crs/index.html)
Coordinate Reference System transformations.
- CRS registry
- Transformer (via PROJ)

### [seisly_production](./seisly_production/index.html)
Production and reservoir tools.
- Well planning
- CCUS monitoring
- Reservoir surveillance

### [seisly_attributes_gpu](./seisly_attributes_gpu/index.html)
GPU-accelerated attribute computation.
- wgpu compute shaders
- Parallel attribute calculation

---

## 📚 Additional Resources

- [Main Documentation](../index.html)
- [Quick Start Guide](../QUICKSTART.html)
- [GitHub Repository](https://github.com/ajamj/seisly)

---

**License:** MIT OR Apache-2.0
