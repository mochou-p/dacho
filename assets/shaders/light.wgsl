// dacho/assets/shaders/light.wgsl

struct UniformBufferObject {
    view:       mat4x4<f32>,
    proj:       mat4x4<f32>,
    camera_pos: vec4<f32>,
    light_pos:  vec4<f32>,
    time:       f32
}

@group(0) @binding(0) var<uniform> ubo: UniformBufferObject;

struct VertexInput {
    @location(0) pos: vec3<f32>,

    @location(1) instance: f32
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    let pos = vec4<f32>(in.pos * vec3<f32>(0.1) + ubo.light_pos.xyz, 1.0);

    var out: VertexOutput;

    out.position = ubo.proj * ubo.view * pos;

    return out;
}

struct FragmentOutput {
    @location(0) color: vec4<f32>
}

@fragment
fn fragment() -> FragmentOutput {
    var out: FragmentOutput;

    out.color = vec4<f32>(1.0);

    return out;
}

