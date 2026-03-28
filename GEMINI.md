# StrataForge Instructional Context

This file provides instructional context for Gemini CLI and other AI agents working on the StrataForge project.

## Project Overview
StrataForge is an open-source subsurface interpretation and modeling platform built in Rust. It follows a modular architecture to handle geophysical data (wells, surfaces, seismic) with a high-performance rendering engine and AI-integrated interpretation tools.

### Tech Stack
- **Languages:** Rust (Core & UI), Python (AI/Deep Learning).
- **UI Framework:** `egui` and `eframe`.
- **Rendering:** `wgpu` (WebGPU abstraction).
- **Data Storage:** SQLite (metadata), BLAKE3-hashed blob store (heavy data).
- **Seismic I/O:** Custom SEG-Y parser (scaffolded).
- **Deep Learning:** PyTorch (via a gRPC microservice bridge).
- **Communication:** gRPC (via `tonic` in Rust and `grpcio` in Python).

## Architecture
The workspace is organized into several crates:
- `sf_core`: Shared types, domain models (Well, Surface, Trajectory), and Protobuf definitions.
- `sf_crs`: Coordinate Reference System transformations using PROJ.
- `sf_storage`: Project manifest management, SQLite persistence, and blob store.
- `sf_io`: File parsers (LAS, CSV, XYZ, SEG-Y).
- `sf_compute`: Geometric algorithms (Triangulation, Resampling, Seismic Slicing).
- `sf_render`: Low-level GPU rendering logic using `wgpu`.
- `sf_cli`: Command-line interface for data management.
- `sf_app`: Desktop interpretation workstation (`egui` + `wgpu`).
- `sf_ai`: (Folder) Python-based AI service for fault detection.

## Building and Running

### Environment Prerequisites (Windows/GNU)
The project currently requires the **GNU toolchain** (`x86_64-pc-windows-gnu`) due to specific dependency linking requirements.
- **GCC:** Required for C/C++ dependencies (`proj-sys`, `libsqlite3-sys`).
- **Protoc:** Required for gRPC code generation.
- **Build Tools:** `make`, `cmake`, `ninja`.

### Key Commands
- **Check Workspace:** `cargo check --workspace`
- **Run CLI:** `cargo run --bin sf -- <args>`
- **Run Desktop App:** `cargo run --bin sf-app`
- **Test All:** `cargo test --workspace`
- **Build All:** `cargo build --workspace`

### Environment Setup (PowerShell)
To ensure the correct tools are found during build:
```powershell
$env:PATH = "C:\Users\Thinkpad\scoop\shims;C:\Users\Thinkpad\scoop\apps\gcc\current\bin;" + $env:PATH
```

## Development Conventions

### Code Style
- **Rust:** Adhere to `cargo fmt` and `cargo clippy` (no warnings allowed).
- **Safety:** Avoid `unsafe` unless strictly necessary for performance or FFI.
- **Errors:** Use `thiserror` for library crates and `anyhow` for applications (CLI/App).

### Testing Practices
- **Unit Tests:** Located in the same file as the implementation (`mod tests`).
- **Integration Tests:** Use `tempfile` for file system operations.
- **TDD:** Prefer writing a failing test case before implementing fixes.

### AI/Python Integration
- The AI service is a standalone Python process.
- Communication is defined in `crates/sf_core/proto/analysis.proto`.
- Always update both sides of the bridge when changing the gRPC contract.

## Known Configuration Fixes
- **Linker:** The project uses a custom `.cargo/config.toml` to point to the `gcc` linker and MinGW libraries.
- **Linker Hack:** If `lgcc_eh` is missing, a hard link from `libgcc.a` to `libgcc_eh.a` in the GCC library directory is used as a workaround.

## Recent Milestones
- **v0.1:** Core domain and CLI tools completed.
- **v0.2:** Desktop UI workstation and AI gRPC bridge completed.
- **v0.3:** (Next) SEG-Y header parsing, memory-mapped access, and interactive picking tools.
