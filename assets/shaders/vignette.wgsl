// dacho/assets/shaders/vignette.wgsl

struct VertexInput {
    @location(0) pos:      vec2<f32>,
    @location(1) instance: f32
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,

    @location(0) uv: vec2<f32>
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.position = vec4<f32>(in.pos, 0.0, 1.0);
    out.uv       = in.pos;

    return out;
}

struct FragmentInput {
    @location(0) uv: vec2<f32>
};

struct FragmentOutput {
    @location(0) color: vec4<f32>
};

@fragment
fn fragment(in: FragmentInput) -> FragmentOutput {
    var out: FragmentOutput;

    out.color = vec4<f32>(
        vec3<f32>(0),
        smoothstep(
            0.90,
            1.0,
            max(length(in.uv.x), length(in.uv.y))
        ) * 0.4
    );

    return out;
}

