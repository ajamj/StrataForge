# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Seisly** is a Rust-based seismic interpretation platform for geoscientists. It's organized as a Cargo workspace with multiple specialized crates.

## Build Commands

```bash
# Build entire workspace
cargo build --workspace

# Build release version
cargo build --release --workspace

# Build specific crate
cargo build -p seisly_cli

# Run desktop application
cargo run --release --bin seisly-app

# Run CLI tool
cargo run --release --bin sf -- <args>
```

## Test Commands

```bash
# Run all tests
cargo test --workspace

# Run tests with output
cargo test --workspace --verbose --no-fail-fast

# Run specific crate tests
cargo test -p seisly_core

# Run benchmarks
cargo bench

# Code coverage (requires cargo-tarpaulin)
cargo tarpaulin --workspace --out Html
```

## Lint Commands

```bash
# Check formatting
cargo fmt --all -- --check

# Apply formatting
cargo fmt --all

# Run Clippy
cargo clippy --workspace --all-targets -- -W clippy::all

# Fix auto-fixable Clippy warnings
cargo clippy --workspace --all-targets --fix
```

## Crate Architecture

The workspace follows a layered architecture:

| Crate | Purpose |
|-------|---------|
| `seisly_core` | Domain types, entities (Well, Log, Surface), CRS definitions |
| `seisly_crs` | PROJ library wrappers for coordinate transforms |
| `seisly_storage` | SQLite schema, content-addressed blob store (BLAKE3) |
| `seisly_io` | File I/O: SEG-Y, LAS 2.0/3.0, CSV, XYZ |
| `seisly_compute` | Algorithms: RBF interpolation, triangulation, gridding |
| `seisly_render` | wgpu-based 3D rendering primitives |
| `seisly_app` | Desktop GUI application (egui/eframe) |
| `seisly_cli` | Command-line interface (`sf` binary) |
| `seisly_attributes` | Seismic attribute computation (CPU) |
| `seisly_attributes_gpu` | GPU-accelerated attributes (wgpu) |
| `seisly_ml` | Machine learning: auto-tracking, fault detection |
| `seisly_qi` | Quantitative Interpretation (AVO, elastic params) |
| `seisly_4d` | Time-lapse seismic, CCUS monitoring |
| `seisly_tracking` | Horizon tracking algorithms |
| `seisly_fwi` | Full Waveform Inversion |
| `seisly_plugin` | Plugin system (Rust + Python) |
| `seisly_production` | Well planning, reservoir surveillance |

## Key Binaries

- `sf` - CLI tool for project management, data import/export
- `seisly-app` - Desktop GUI application

## Project Format

Seisly projects are folders with `.sf` extension:

```
MyField.sf/
  project.yaml          # Manifest with name, CRS, version
  metadata.sqlite       # Structured metadata
  blobs/                # Content-addressed storage (BLAKE3 hashes)
  cache/                # Derived data cache
  workflows/            # Workflow execution records
  logs/                 # Application logs
```

## Common Development Tasks

### Create and test a project via CLI

```bash
cargo run --bin sf -- init --name "TestField" --crs 32648
cargo run --bin sf -- list --project TestField.sf
```

### Run a specific test

```bash
cargo test --package seisly_io --test integration_test
```

### Build with specific features

```bash
cargo build --package seisly_app --features wgpu
```

## System Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get install -y libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev \
  libxkbcommon-dev libssl-dev pkg-config libgtk-3-dev libfontconfig1-dev \
  protobuf-compiler libproj-dev proj-bin
```

**Windows:**
```powershell
choco install protoc sqlite
```

**macOS:**
```bash
brew install openssl pkg-config proj
```

## Key Technologies

- **UI:** egui/eframe (immediate mode GUI)
- **GPU:** wgpu (cross-platform rendering)
- **Geospatial:** PROJ (via `proj` crate)
- **Database:** SQLite (via `rusqlite` with bundled feature)
- **Serialization:** serde, serde_json, serde_yaml
- **Async:** tokio
- **ML:** candle (Rust-native ML)
- **Testing:** Built-in `cargo test`, benchmarks in `benches/`

## Code Patterns

- All entities use UUID v4 for IDs
- CRS handling uses EPSG codes (e.g., "EPSG:32648")
- Content-addressed storage uses BLAKE3 hashes
- Large binary objects stored in `blobs/` by hash
- Triangulation uses `spade` crate (Delaunay)
- Error handling uses `thiserror` and `anyhow`

## CI/CD Pipeline

The `.github/workflows/ci-cd.yml` runs:
1. Format checking (`cargo fmt`)
2. Clippy linting
3. Build on Linux and Windows
4. Test on both platforms
5. Coverage reporting (tarpaulin)
6. Artifact upload

## Documentation

- Architecture: `architecture.md`
- Blueprint/Roadmap: `blueprint.md`
- Quickstart: `QUICKSTART.md`
- API docs: Generated at `api/` (Jekyll site)
