struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.tex_coords = model.tex_coords;
    return out;
}

// Uniforms for gain, clip, and colormap
struct SeismicUniforms {
    gain: f32,
    clip: f32,
    colormap_index: u32,
    _padding: u32,
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
    // 1. Sample the raw seismic value (float)
    let raw_value = textureSample(t_seismic, s_seismic, in.tex_coords).r;
    
    // 2. Apply gain and clipping
    let normalized = clamp((raw_value * uniforms.gain) / uniforms.clip, -1.0, 1.0);
    
    // 3. Map to [0, 1] for colormap lookup
    let t = (normalized + 1.0) * 0.5;
    
    // 4. Sample colormap
    let color = textureSample(t_colormap, s_colormap, t);
    
    return color;
}
