struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    
    // Generate a quad using vertex index
    // 0: (-1, -1), 1: (1, -1), 2: (-1, 1), 3: (1, 1)
    let x = f32(i32(in_vertex_index & 1u) * 2 - 1);
    let y = f32(i32(in_vertex_index & 2u) - 1);
    
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.tex_coords = vec2<f32>(
        f32(in_vertex_index & 1u),
        1.0 - f32(in_vertex_index >> 1u)
    );
    
    return out;
}

// Uniforms for gain, clip, and colormap
struct SeismicUniforms {
    gain: f32,
    clip: f32,
    u_scale: f32,
    v_scale: f32,
};

@group(0) @binding(0)
var t_seismic: texture_2d<f32>;
@group(0) @binding(1)
var s_seismic: sampler;
@group(0) @binding(2)
var t_colormap: texture_1d<f32>;
@group(0) @binding(3)
var s_colormap: sampler;
@group(0) @binding(4)
var<uniform> uniforms: SeismicUniforms;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 1. Scale UVs to sample only the active part of the 2048x2048 texture
    let scaled_uv = vec2<f32>(in.tex_coords.x * uniforms.u_scale, in.tex_coords.y * uniforms.v_scale);
    
    // 2. Sample the seismic value (stored in R channel)
    let raw_value = textureSample(t_seismic, s_seismic, scaled_uv).r;
    
    // 3. Apply gain and clipping (normalized to [-1, 1])
    let normalized = clamp((raw_value * uniforms.gain) / uniforms.clip, -1.0, 1.0);
    
    // 4. Map to [0, 1] for colormap lookup
    let t = (normalized + 1.0) * 0.5;
    
    // 5. Sample colormap
    let color = textureSample(t_colormap, s_colormap, t);
    
    return color;
}

struct WiggleOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_wiggle(
    @location(0) pos: vec3<f32>, // x: trace_idx, y: sample_idx, z: amplitude
) -> WiggleOutput {
    var out: WiggleOutput;
    
    // Scale amplitude by gain
    let offset = pos.z * uniforms.gain * 0.05; // Small factor for visual scaling
    
    // Map to normalized clip space
    // pos.x is [0, xline_count], pos.y is [0, sample_count]
    let x = (pos.x / uniforms.u_scale) * 2.0 - 1.0 + offset;
    let y = 1.0 - (pos.y / uniforms.v_scale) * 2.0;
    
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);
    out.color = vec4<f32>(0.0, 0.0, 0.0, 1.0); // Black wiggles
    
    return out;
}

@fragment
fn fs_wiggle(in: WiggleOutput) -> @location(0) vec4<f32> {
    return in.color;
}

