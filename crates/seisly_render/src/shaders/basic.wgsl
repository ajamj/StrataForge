// Basic shader for 3D rendering

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
};

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    // For now, identity transform as basic pipeline lacks uniforms
    out.position = vec4<f32>(in.position, 1.0);
    out.normal = in.normal;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Simple directional light for basic shading
    let light_dir = normalize(vec3<f32>(0.5, 0.8, 1.0));
    let normal = normalize(in.normal);
    let diff = max(dot(normal, light_dir), 0.3);
    
    let base_color = vec3<f32>(0.5, 0.7, 1.0); // Default Seisly blue
    return vec4<f32>(base_color * diff, 1.0);
}
