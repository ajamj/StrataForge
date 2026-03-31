# Changelog

All notable changes to Seisly will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-03-31

### 🎉 Production Release & Rebranding

#### Added

**Professional UI/UX:**
- Implemented industry-standard dockable panel architecture using `egui_dock`.
- New multi-tab workspace with persistent layouts (remembers your setup between restarts).
- Unified Seisly design system with brand-aligned colors and improved 8px grid spacing.
- Global shortcut manager (Cmd+Z/Y for Undo/Redo, Cmd+S for Save, D for Depth Toggle).
- Real-time theme switcher (Light/Dark mode) with high-quality icons.

**Secure Plugin Architecture:**
- Transitioned Python plugins to a **process-isolated worker model**.
- Plugins now run in a separate OS process, making the main application crash-proof against faulty scripts.
- Robust JSON-RPC communication bridge between Rust and Python.
- Automatic worker recovery and state restoration.

**Performance & Robustness:**
- **Out-of-Core Handling:** Implemented LRU brick caching for seismic volumes using `moka`, allowing Seisly to handle datasets larger than available RAM.
- **Production Observability:** Integrated Sentry SDK for real-time crash reporting and performance monitoring.
- **Enhanced Rendering:** Proper vertex normal computation for smooth 3D shaded surfaces.
- Automated cross-platform installers (MSI for Windows, PKG for macOS) via `cargo-dist`.

#### Changed
- **REBRANDING:** Completely rebranded from *StrataForge* to **Seisly**.
- Updated all crate names, module paths, and metadata to `seisly_*`.
- Modernized entire workspace to `egui` 0.29 and `wgpu` 22.1.

---

## [0.3.1] - 2026-03-29

### GPU Acceleration for Seismic Attributes

#### Added
- New `seisly_attributes_gpu` crate for GPU compute pipelines.
- wgpu-based compute shaders for RMS, Mean, and Energy attributes.
- 10x performance improvement for large volumes.

---

## [0.3.0] - 2026-03-29

### Phase 1: Advanced Features

#### Added
- CNN-based horizon auto-tracking (`seisly_ml`).
- 20+ seismic attributes (Amplitude & Frequency).
- Initial Python plugin system with zero-copy data sharing.

---

## [0.2.0] - 2026-03-28

### Phase 0: Well-Seismic Workflow Foundation

#### Added
- SEG-Y I/O with memory-mapped access.
- LAS 3.0 parser and Formation Top management.
- Well-seismic tie with V0+kZ velocity modeling.

---

## Version History

| Version | Date | Status | Key Features |
|---------|------|--------|--------------|
| **1.0.0** | **2026-03-31** | **🚀 Released** | **Production Release, UI/UX, Security** |
| 0.3.1 | 2026-03-29 | ✅ Released | GPU Acceleration |
| 0.3.0 | 2026-03-29 | ✅ Released | ML + Attributes |
| 0.2.0 | 2026-03-28 | ✅ Released | Well-Seismic Workflow |
| 0.1.1 | 2026-03-20 | ✅ Released | Core interpretation |

---

## License

MIT OR Apache-2.0

---

**[1.0.0]:** https://github.com/ajamj/Seisly/compare/v0.3.1...v1.0.0
**[0.3.1]:** https://github.com/ajamj/Seisly/compare/v0.3.0...v0.3.1
**[0.3.0]:** https://github.com/ajamj/Seisly/compare/v0.2.0...v0.3.0
**[0.2.0]:** https://github.com/ajamj/Seisly/compare/v0.1.1...v0.2.0
**[0.1.1]:** https://github.com/ajamj/Seisly/releases/tag/v0.1.1
