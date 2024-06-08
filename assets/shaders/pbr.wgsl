// dacho/assets/shaders/pbr.wgsl

const pi      = 3.14159265359;
const sun_pos = vec3<f32>(1.5, -1.0, 1.0);

struct UniformBufferObject {
    view:       mat4x4<f32>,
    proj:       mat4x4<f32>,
    camera_pos: vec4<f32>,
    time:       f32
}

@group(0) @binding(0) var<uniform> ubo: UniformBufferObject;

struct VertexInput {
    @location(0) pos:    vec3<f32>,
    @location(1) normal: vec3<f32>,

    @location(2) color:  vec3<f32>,
    @location(3) metrou: vec2<f32>
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,

    @location(0) world_pos:  vec3<f32>,
    @location(1) normal:     vec3<f32>,
    @location(2) camera_pos: vec3<f32>,
    @location(3) base_color: vec3<f32>,
    @location(4) metalness:  f32,
    @location(5) roughness:  f32
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    let pos   = vec4<f32>(in.pos, 1.0);

    var out: VertexOutput;

    out.position   = ubo.proj * ubo.view * pos;
    out.world_pos  = pos.xyz;
    out.normal     = in.normal;
    out.camera_pos = ubo.camera_pos.xyz;
    out.base_color = in.color;
    out.metalness  = in.metrou.x;
    out.roughness  = in.metrou.y;

    return out;
}

struct FragmentInput {
    @location(0) world_pos:  vec3<f32>,
    @location(1) normal:     vec3<f32>,
    @location(2) camera_pos: vec3<f32>,
    @location(3) base_color: vec3<f32>,
    @location(4) metalness:  f32,
    @location(5) roughness:  f32
}

struct FragmentOutput {
    @location(0) color: vec4<f32>
}

@fragment
fn fragment(in: FragmentInput) -> FragmentOutput {
    let N = normalize(in.normal);
    let V = normalize(in.camera_pos - in.world_pos);

    var radiance = vec3<f32>(0.0);

    /*
    for (var i = 0; i < 2; i++) {
        let L = // branchless if
            vec3<f32>(abs(i-1)) * normalize(sun_pos)
            + vec3<f32>(i)      * normalize(light_pos - in.world_pos);

        let H = normalize(L + V);

        let V_dot_H = max(0.0, dot(V, H));

        let Ks      = fresnel(vec3<f32>((1.0 - in.metalness) * 0.8), in.metalness, V_dot_H);
        let Kd      = (vec3<f32>(1.0) - Ks) * vec3<f32>(1.0 - in.metalness) * in.base_color;
        let diffuse = Kd * vec3<f32>(max(dot(L, N), 0.0));

        let cosTheta = max(dot(N, L), 0.0);
        
        let specular = clamp_0_1(cook_torrance(N, V, H, L, in.base_color, in.metalness, in.roughness));

        radiance += (diffuse + specular) * vec3<f32>(cosTheta);
    }
    */

    let ambient  = in.base_color * 0.035 * (1.0 - in.metalness);
    radiance    += ambient;

    var out: FragmentOutput;

    out.color = vec4<f32>(radiance, 1.0);

    return out;
}

fn clamp_0_1(value: vec3<f32>) -> vec3<f32> {
    return clamp(value, vec3<f32>(0.0), vec3<f32>(1.0));
}

fn fresnel(r: vec3<f32>, metalness: f32, a: f32) -> vec3<f32> {
    return r + (vec3<f32>(1.0) - r) * pow(1.0 - max(a, 0.0), 5.0);
}

fn ggx(N_dot_H: f32, roughness: f32) -> f32 {
    let a  = roughness * roughness;
    let a2 = a * a;
    let d  = N_dot_H * N_dot_H * (a2 - 1.0) + 1.0;

    return a2 / (pi * d * d);
}

fn schlick(N_dot_L: f32, N_dot_V: f32, a: f32) -> f32 {
    let a1 = a + 1.0;
    let k  = a1 * a1 * 0.125;
    let G1 = N_dot_L / (N_dot_L * (1.0 - k) + k);
    let G2 = N_dot_V / (N_dot_V * (1.0 - k) + k);

    return G1 * G2;
}

fn f(F0: vec3<f32>, V: vec3<f32>, H: vec3<f32>) -> vec3<f32> {
    return F0 + (vec3<f32>(1.0) - F0) * pow(1 - max(dot(V, H), 0.0), 5.0);
}

fn lambert(L_dot_N: f32) -> vec3<f32> {
    return vec3<f32>(L_dot_N);
}

fn cook_torrance(
    N: vec3<f32>, V: vec3<f32>, H: vec3<f32>, L: vec3<f32>, base_color: vec3<f32>, metalness: f32, roughness: f32
) -> vec3<f32> {
    let N_dot_H = max(0.0,  dot(N, H));
    let N_dot_V = max(1e-7, dot(N, V));
    let N_dot_L = max(1e-7, dot(N, L));
    let V_dot_H = max(0.0,  dot(V, H));

    let D = ggx(N_dot_H, roughness);
    let G = schlick(N_dot_L, N_dot_V, roughness);
    let F = fresnel(base_color, metalness, V_dot_H);

    return (F / vec3<f32>(pi)) * vec3<f32>(D * G) / vec3<f32>(4.0 * N_dot_L * N_dot_V);
}

