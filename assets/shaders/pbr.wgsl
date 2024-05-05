// dacho/assets/shaders/pbr.wgsl

struct UniformBufferObject {
    view:       mat4x4<f32>,
    proj:       mat4x4<f32>,
    camera_pos: vec3<f32>,
    time:       f32
};

@group(0) @binding(0) var<uniform> ubo: UniformBufferObject;

struct VertexInput {

    @location(0) pos:      vec3<f32>,
    @location(1) normal:   vec3<f32>,
    @location(2) uv:       vec2<f32>,
    @location(3) instance: f32
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,

    @location(0) world_pos:  vec3<f32>,
    @location(1) normal:     vec3<f32>,
    @location(2) uv:         vec2<f32>,
    @location(3) camera_pos: vec3<f32>
};

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    let rotation_x = mat3x3<f32>(
         1.0, 0.0,  0.0,
         0.0, 0.0, -1.0,
         0.0, 1.0,  0.0
    );

    let rotation_y = mat3x3<f32>(
        -1.0, 0.0,  0.0,
         0.0, 1.0,  0.0,
         0.0, 0.0, -1.0
    );

    let fixed_position = mat3x3<f32>(rotation_y * rotation_x) * in.pos;
    let pos            = vec4<f32>(fixed_position.x, -fixed_position.y, fixed_position.z, 1.0);

    var out: VertexOutput;

    out.position   = ubo.proj * ubo.view * pos;
    out.world_pos  = (ubo.view * pos).xyz;
    out.normal     = in.normal;
    out.uv         = in.uv;
    out.camera_pos = ubo.camera_pos;

    return out;
}

@group(0) @binding(1) var smp:  sampler;
@group(0) @binding(3) var texs: array<texture_2d<f32>, 5>;

struct FragmentInput {
    @location(0) world_pos:  vec3<f32>,
    @location(1) normal:     vec3<f32>,
    @location(2) uv:         vec2<f32>,
    @location(3) camera_pos: vec3<f32>
};

struct FragmentOutput {
    @location(0) color: vec4<f32>
};

@fragment
fn fragment(in: FragmentInput) -> FragmentOutput {
    let normal_map = textureSample(texs[1], smp, in.uv).xyz;
    let metrou     = textureSample(texs[2], smp, in.uv).zy;
    let emission   = textureSample(texs[3], smp, in.uv).xyz;
    let occlusion  = textureSample(texs[4], smp, in.uv).x;
    let albedo     = textureSample(texs[0], smp, in.uv).xyz * occlusion;

    let met   = metrou.x;
    let rou   = metrou.y;
    let alpha = rou * rou;

    let projected_normal = normalize(reflect(normal_map, in.normal));
    let in_normal_f      = 0.7;
    let normal_map_f     = 1.0 - in_normal_f;
    let normal           = normalize(in.normal * in_normal_f + projected_normal * normal_map_f);

    let light_position = vec3<f32>(0.0, 1.0, -0.85);
    let light_color    = vec3<f32>(1.0);

    let N = normalize(normal);
    let V = normalize(in.camera_pos - in.world_pos);
    let L = normalize(light_position);
    let H = normalize(V + L);

    let F0_met = vec3<f32>(0.90, 0.90, 0.80);
    let F0_die = vec3<f32>(0.04, 0.04, 0.04);
    let F0     = mix(F0_die, F0_met, met);

    var out: FragmentOutput;

    out.color = vec4<f32>(PBR(N, V, L, H, alpha, met, F0, albedo, emission, light_color), 1.0);

    return out;
}

fn D(alpha: f32, N: vec3<f32>, H: vec3<f32>) -> f32 {
    let numerator   = pow(alpha, 2.0);
    let N_dot_H     = max(dot(N, H), 0.0);
    var denominator = 3.1415926538 * pow(pow(N_dot_H, 2.0) * (pow(alpha, 2.0) - 1.0) + 1.0, 2.0);

    denominator = max(denominator, 0.000001);

    return numerator / denominator;
}

fn G1(alpha: f32, N: vec3<f32>, X: vec3<f32>) -> f32 {
    let numerator   = max(dot(N, X), 0.0);
    let k           = alpha / 2.0;
    var denominator = max(dot(N, X), 0.0) * (1.0 - k) + k;

    denominator = max(denominator, 0.000001);

    return numerator / denominator;
}

fn G(alpha: f32, N: vec3<f32>, V: vec3<f32>, L: vec3<f32>) -> f32 {
    return G1(alpha, N, V) * G1(alpha, N, L);
}

fn F(F0: vec3<f32>, V: vec3<f32>, H: vec3<f32>) -> vec3<f32> {
    return F0 + (vec3<f32>(1.0) - F0) * pow(1 - max(dot(V, H), 0.0), 5.0);
}

fn PBR(
    N:           vec3<f32>,
    V:           vec3<f32>,
    L:           vec3<f32>,
    H:           vec3<f32>,
    alpha:       f32,
    met:         f32,
    F0:          vec3<f32>,
    albedo:      vec3<f32>,
    emission:    vec3<f32>,
    light_color: vec3<f32>
) -> vec3<f32> {
    let Ks = F(F0, V, H);
    let Kd = (vec3<f32>(1.0) - Ks) * (1.0 - met);

    let lambert = albedo / 3.1415926538;

    let cook_torrance_numerator   = D(alpha, N, H) * G(alpha, N, V, L) * F(F0, V, H);
    var cook_torrance_denominator = 4.0 * max(dot(V, N), 0.0) * max(dot(L, N), 0.0);

    cook_torrance_denominator = max(cook_torrance_denominator, 0.000001);

    let cook_torrance = cook_torrance_numerator / cook_torrance_denominator;

    let BRDF = Kd * lambert + cook_torrance;

    return emission + BRDF * light_color * max(dot(L, N), 0.0);
}

