# Design Spec: Plugin-Based Python AI Architecture for Seisly

**Date:** 2026-03-30
**Status:** Draft
**Goal:** Implement a high-performance, user-extensible Deep Learning engine for automatic fault and horizon detection, enabling Seisly to compete as a lightweight, open-source alternative to Petrel.

---

## 1. Overview
This architecture replaces the standalone gRPC-based Python service with an embedded Python interpreter managed directly by the Rust-core application. It allows geoscientists to develop and share AI models as "Python Plugins" while maintaining the extreme performance and "instant" feel of the Rust engine.

## 2. Architecture & Core Concepts

### 2.1 Embedded Python Engine
- **Technology:** [PyO3](https://pyo3.rs/) for direct Rust-to-Python bindings.
- **Crate:** `seisly_plugin` (Manager) and `seisly_ml_deep` (Inference logic).
- **Lifecycle:** The Rust application initializes a single, persistent Python interpreter at startup.

### 2.2 Plugin Discovery & Registration
- **Directory:** `plugins/` (scanned on startup).
- **Structure:**
  - `manifest.yaml`: Metadata (Name, Author, Version, Plugin Type).
  - `main.py`: Core logic implementing a standard `SeislyPlugin` interface.
  - `requirements.txt`: Plugin-specific dependencies.
- **Discovery:** The `PluginManager` (Rust) parses manifests and registers available tools in the UI.

## 3. Data Flow & Performance

### 3.1 High-Speed Zero-Copy Data Sharing
- **Mechanism:** Shared memory pointers and shapes passed via PyO3.
- **Seismic Access:** Rust passes a raw pointer to the memory-mapped SEG-Y volume (or a slice).
- **Python Implementation:** Plugins wrap the raw pointer into a **NumPy array** or **PyTorch tensor** without copying data.
- **Target Latency:** <100ms for sharing a 1GB volume from Rust to NumPy.

### 3.2 Result Back-Propagation
- **Output:** Python plugins return a structured list of picks: `Vec<(f32, f32, f32, f32)>` (X, Y, Z, Confidence).
- **Persistence:** Results are stored directly in the project's SQLite database via the `seisly_storage` crate.

## 4. User Experience (UX)

### 4.1 egui Interface
- **Plugin Manager Window:** Lists installed plugins, status, and logs.
- **Dynamic Property Panels:** The Rust UI automatically generates control sliders/inputs based on a schema defined by the Python plugin.
- **AI Console:** Real-time log output from the Python interpreter (stdout/stderr).

### 4.2 Dependency Management
- **Automatic Environments:** Seisly will optionally manage a dedicated Python virtual environment for each plugin to prevent library conflicts.

## 5. Testing & Validation

### 5.1 Unit & Integration Tests
- **Rust/Python Bridge:** Verify correct pointer passing and memory safety using PyO3.
- **E2E Workflow:** Import SEG-Y → Load "Mock" Python Plugin → Run Tracking → Verify SQLite Picks.
- **Stress Test:** Load and run multiple large-volume plugins simultaneously.

### 5.2 Performance Benchmarks
- **Throughput:** Measure data sharing speed for volumes up to 10GB.
- **Memory Leak Check:** Ensure the Python interpreter correctly releases shared buffers.

---

## 6. Implementation Phases (Strategic)

1. **Phase 1: PyO3 Scaffold:** Basic interpreter initialization and `sys.path` management.
2. **Phase 2: Plugin Discovery:** `PluginManager` implementation and manifest parsing.
3. **Phase 3: High-Speed Buffer:** Shared memory implementation (Rust -> NumPy).
4. **Phase 4: Dynamic UI:** Automatic property panel generation in egui.
5. **Phase 5: Real-World Plugins:** Porting existing CNN models from `seisly_ai` to the new plugin format.
