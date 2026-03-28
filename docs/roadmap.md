# StrataForge Roadmap

## v0.1 - Offline Wells + Surfaces (Completed)

**Status:** Completed

### Completed
- [x] Workspace skeleton with 8 crates
- [x] Core domain model (Well, Trajectory, Log, Surface)
- [x] CRS support with PROJ
- [x] SQLite schema and blob store
- [x] LAS parser
- [x] Trajectory CSV parser
- [x] XYZ surface parser
- [x] Delaunay triangulation
- [x] Trajectory resampling
- [x] CLI commands (init, import, list)
- [x] Desktop app shell (egui + wgpu)

---

## v0.2 - Interactive Seismic & AI Integration (Completed)

**Status:** Completed

### Completed Features
- [x] Desktop App UI
  - [x] Three-panel layout (Explorer, 3D Viewport, Analysis)
  - [x] wgpu integration via PaintCallback
- [x] Seismic I/O & Compute
  - [x] SEG-Y parser scaffold
  - [x] 3D Volume Slicer (Inline/Crossline)
- [x] AI Microservice
  - [x] gRPC Service Contract (ProtoBuf)
  - [x] Python AI Service (PyTorch + gRPC Server)
  - [x] Rust gRPC Client (Tonic)

---

## v0.3 - SEG-Y + Slice + Picks (Completed)

**Status:** Completed

### Completed Features
- [x] Real SEG-Y header parsing (EBCDIC/Binary)
- [x] High-performance Memory-mapped volume access (`memmap2`)
- [x] Interactive Horizon Picking
  - [x] 1D Snap-to-extrema
  - [x] 2D Seed & Auto-Track algorithm
- [x] Interpretation Workflow
  - [x] Interpretation Explorer (Horizon management)
  - [x] Picking Toolbar (Mode selection)
- [x] Smooth Surface Generation (RBF Interpolation)
- [x] Viewport Visualization (Picks & Smooth Meshes)

---

## v0.4 - Advanced Features (Next)

**Status:** Planned

### Planned Features
- [ ] Multi-volume support
- [ ] Fault stick picking and modeling
- [ ] Depth conversion (Simple velocity models)
- [ ] Surface-to-surface intersection
- [ ] Export interpreted horizons (XYZ/SEG-Y)

**Target:** Q4 2026
