# wgpu

`wgpu` is the high-performance 3D rendering engine used in StrataForge. It provides a modern, portable graphics API based on the WebGPU standard.

## Overview

In StrataForge, we use `wgpu` to render large geophysical datasets (surfaces, well trajectories, and seismic volumes) with high efficiency. Its cross-platform nature allows us to run the same rendering code across different OS platforms and hardware (Vulkan, Metal, DirectX 12).

### Why wgpu?
- **Modern API:** Better performance and control compared to legacy APIs like OpenGL.
- **Portability:** Write once, run anywhere.
- **Safety:** Built-in safety checks for memory and GPU state.
- **WGPU Integration:** Seamless integration with `egui` via `egui_wgpu`.

## Usage in Project

The `sf_render` crate contains our low-level `wgpu` rendering primitives.

### Buffer Management
Located in `crates/sf_render/src/mesh.rs`:

```rust
use wgpu::{Device, Buffer, BufferUsages};
use wgpu::util::DeviceExt;

pub struct MeshRenderer {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl MeshRenderer {
    pub fn new(device: &Device, vertices: &[f32], indices: &[u32]) -> Self {
        // Create vertex buffer using DeviceExt for easy initialization
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Mesh Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
        });
        
        // ... (similarly for index buffer)
    }
}
```

### Rendering Pipeline
We utilize `wgpu` render passes to draw our 3D objects. In `crates/sf_app/src/widgets/viewport.rs`, we integrate with `egui`'s paint callback mechanism to perform custom `wgpu` drawing.

## Key Concepts

- **Instance:** The root object of the `wgpu` API, used to find and select a GPU adapter.
- **Adapter:** A representation of a physical GPU (e.g., NVIDIA GeForce RTX 3070).
- **Device:** A logical connection to the GPU, used for creating resources (buffers, textures, shaders).
- **Queue:** Used for submitting command buffers to the GPU for execution.
- **RenderPass:** A set of commands to be executed in a single pass of the GPU.
- **Shader:** Small programs (written in WGSL) that run on the GPU to process vertices and pixels.

## Resources

- [Official wgpu Website](https://wgpu.rs/)
- [wgpu Documentation (docs.rs)](https://docs.rs/wgpu)
- [Learn wgpu Tutorial](https://sotrh.github.io/learn-wgpu/)
- [WebGPU Specification](https://gpuweb.github.io/gpuweb/)
