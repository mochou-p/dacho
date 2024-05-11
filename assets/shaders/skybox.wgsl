// dacho/assets/shaders/skybox.wgsl

const far_depth = 0.99999997;
const tau       = 6.2831853076;

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
    @builtin(position) position: vec4<f32>,

    @location(0) pos: vec3<f32>
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.position = ubo.proj * ubo.view * vec4<f32>(in.pos + ubo.camera_pos.xyz, 1.0);
    out.pos      = in.pos;

    return out;
}

@group(0) @binding(1) var smp: sampler;
@group(0) @binding(2) var tex: texture_2d<f32>;

struct FragmentInput {
    @location(0) pos: vec3<f32>
}

struct FragmentOutput {
    @builtin(frag_depth) frag_depth: f32,

    @location(0) color: vec4<f32>
}

@fragment
fn fragment(in: FragmentInput) -> FragmentOutput {
    var out: FragmentOutput;

    out.frag_depth = far_depth;
    out.color      = textureSample(tex, smp, sphere_uv(normalize(in.pos)));

    return out;
}

fn sphere_uv(view_dir: vec3<f32>) -> vec2<f32> {
    return vec2<f32>(
        atan2(view_dir.z, view_dir.x) / tau + 0.5,
        view_dir.y * 0.5 + 0.5
    );
}

