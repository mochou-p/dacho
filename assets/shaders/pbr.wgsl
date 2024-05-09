// dacho/assets/shaders/pbr.wgsl

struct UniformBufferObject {
    view:       mat4x4<f32>,
    proj:       mat4x4<f32>,
    camera_pos: vec4<f32>,
    light_pos:  vec4<f32>,
    time:       f32
}

@group(0) @binding(0) var<uniform> ubo: UniformBufferObject;

struct VertexInput {
    @location(0) pos:    vec3<f32>,
    @location(1) normal: vec3<f32>,

    @location(2) instance: vec2<f32>
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,

    @location(0) world_pos:  vec3<f32>,
    @location(1) normal:     vec3<f32>,
    @location(2) camera_pos: vec3<f32>,
    @location(3) light_pos:  vec3<f32>,
    @location(4) metalness:  f32,
    @location(5) roughness:  f32
}

@vertex
fn vertex(in: VertexInput) -> VertexOutput {
    let steps = 10.0;
    let pos   = vec4<f32>(in.pos * 0.4 + vec3<f32>(in.instance, 0.0), 1.0);
    let met   = (in.instance.x + 0.5 * (steps - 1.0)) / (steps - 1.0);
    let rou   = (in.instance.y + 0.5 * (steps - 1.0)) / (steps - 1.0);

    var out: VertexOutput;

    out.position   = ubo.proj * ubo.view * pos;
    out.world_pos  = (ubo.view * pos).xyz;
    out.normal     = in.normal;
    out.camera_pos = ubo.camera_pos.xyz;
    out.light_pos  = ubo.light_pos.xyz;
    out.metalness  = met * 0.92 + 0.04;
    out.roughness  = rou * 0.92 + 0.04;

    return out;
}

struct FragmentInput {
    @location(0) world_pos:  vec3<f32>,
    @location(1) normal:     vec3<f32>,
    @location(2) camera_pos: vec3<f32>,
    @location(3) light_pos:  vec3<f32>,
    @location(4) metalness:  f32,
    @location(5) roughness:  f32
}

struct FragmentOutput {
    @location(0) color: vec4<f32>
}

@fragment
fn fragment(in: FragmentInput) -> FragmentOutput {
    let BaseColor = vec3<f32>(0.0, 0.01, 1.0);

    let N = normalize(in.normal);
    let L = normalize(in.light_pos);
    let V = normalize(in.camera_pos - in.world_pos);
    let H = normalize(L + V);

    let VdotH = max(0.0,  dot(V, H));
    let Ks    = Fresnel(BaseColor, in.metalness, VdotH);
    let Kd    = (vec3<f32>(1.0) - Ks) * vec3<f32>(1.0 - in.metalness);

    let Li       = vec3<f32>(7.0);
    let cosTheta = vec3<f32>(max(dot(N, L), 0.0));
    let diffuse  = Kd * Lambert(BaseColor);
    let specular = clamp(CookTorrance(N, V, H, L, BaseColor, in.metalness, in.roughness), vec3<f32>(0.0), vec3<f32>(1.0));
    let radiance = (diffuse + specular) * cosTheta * Li;

    var out: FragmentOutput;

    out.color = vec4<f32>(radiance, 1.0);

    return out;
}

fn toColor(v: vec3<f32>) -> vec3<f32> {
    return v * vec3<f32>(0.5) + vec3<f32>(0.5);
}

fn Fresnel(BaseColor: vec3<f32>, Metalness: f32, b: f32) -> vec3<f32> {
    let F0 = mix(vec3<f32>(0.04), BaseColor, Metalness);
    return F0 + (vec3<f32>(1.0) - F0) * pow(vec3<f32>(1.0) - b, vec3<f32>(5.0));
}

fn GGX(NdotH: f32, Roughness: f32) -> f32 {
    let a  = Roughness * Roughness;
    let a2 = a * a;
    let d  = NdotH * NdotH * (a2 - 1.0) + 1.0;
    return a2 / (3.14159265359 * d * d);
}

fn Beckmann(NdotH: f32, Roughness: f32) -> f32 {
    let a  = Roughness * Roughness;
    let a2 = a * a;
    let r1 = 1.0 / (4.0 * a2 * pow(NdotH, 4.0));
    let r2 = (NdotH * NdotH - 1.0) / (a2 * NdotH * NdotH);
    return r1 * exp(r2);
}

fn Schlick(NdotL: f32, NdotV: f32, a: f32) -> f32 {
    let a1 = a + 1.0;
    let k  = a1 * a1 * 0.125;
    let G1 = NdotL / (NdotL * (1.0 - k) + k);
    let G2 = NdotV / (NdotV * (1.0 - k) + k);
    return G1 * G2;
}

fn GCT(NdotL: f32, NdotV: f32, NdotH: f32, VdotH: f32) -> f32 {
    let G1 = (2.0 * NdotH * NdotV) / VdotH;
    let G2 = (2.0 * NdotH * NdotL) / VdotH;
    return min(1.0, min(G1, G2));
}

fn Keleman(NdotL: f32, NdotV: f32, VdotH: f32) -> f32 {
    return (NdotL * NdotV) / (VdotH * VdotH);
}

fn F(F0: vec3<f32>, V: vec3<f32>, H: vec3<f32>) -> vec3<f32> {
    return F0 + (vec3<f32>(1.0) - F0) * pow(1 - max(dot(V, H), 0.0), 5.0);
}

fn Lambert(DiffuseReflectance: vec3<f32>) -> vec3<f32> {
    return DiffuseReflectance / vec3<f32>(3.14159265359);
}

fn CookTorrance(
    N: vec3<f32>, V: vec3<f32>, H: vec3<f32>, L: vec3<f32>, BaseColor: vec3<f32>, Metalness: f32, Roughness: f32
) -> vec3<f32> {
    let NdotH = max(0.0,  dot(N, H));
    let NdotV = max(1e-7, dot(N, V));
    let NdotL = max(1e-7, dot(N, L));
    let VdotH = max(0.0,  dot(V, H));

    let D = GGX(NdotH, Roughness);
//    let D = Beckmann(NdotH, Roughness);

    let G = Schlick(NdotL, NdotV, Roughness);
//    let G = GCT(NdotL, NdotV, NdotH, VdotH);
//    let G = Keleman(NdotL, NdotV, VdotH);

    let F = Fresnel(BaseColor, Metalness, VdotH);

    return (F / vec3<f32>(3.14159265359)) * vec3<f32>(D * G) / vec3<f32>(4.0 * NdotL * NdotV);
}

