---
layout: default
title: Architecture
nav_order: 4
description: "Technical overview of Seisly's crate structure and data flow."
---

# Seisly Architecture

Seisly is built with a highly modular architecture using Rust workspace members.

## Crate Overview

| Crate | Responsibility |
|:--- |:--- |
| `seisly_core` | Shared domain entities (Well, Surface, Fault) and types. |
| `seisly_io` | High-performance SEG-Y and LAS parsers. |
| `seisly_compute` | Mathematical kernels and interpretation algorithms. |
| `seisly_render` | WGPU-based 3D rendering pipeline. |
| `seisly_app` | Desktop workstation built with `egui`. |
| `seisly_plugin` | Secure Python/Rust plugin manager. |
| `seisly_py_worker` | Isolated process for Python AI execution. |

## Data Flow

Seisly prioritizes zero-copy data paths for large seismic volumes. Data is typically memory-mapped via `memmap2` in `seisly_io`, shared via raw pointers to the WGPU rendering engine in `seisly_render`, and passed to isolated Python workers via Shared Memory (SHM) segments in `seisly_plugin`.

---

[Back to Home]({% link index.md %})
