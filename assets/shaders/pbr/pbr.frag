// dacho/assets/shaders/pbr/pbr.frag

#version 460 core

precision lowp float;

#define PI                 3.1415926538

#define ALBEDO             0
#define NORMAL             1
#define METALLIC_ROUGHNESS 2
#define EMISSION           3
#define OCCLUSION          4
#define TEXTURE_COUNT      5

layout(binding = 1) uniform sampler2D samplers[TEXTURE_COUNT];

layout(location = 0) in  vec3 inWorldPosition;
layout(location = 1) in  vec3 inNormal;
layout(location = 2) in  vec2 inTexCoord;
layout(location = 3) in  vec3 inCameraPosition;

layout(location = 0) out vec4 outColor;

float D(float alpha, vec3 N, vec3 H) {
    float numerator   = pow(alpha, 2.0);
    float N_dot_H     = max(dot(N, H), 0.0);
    float denominator = PI * pow(pow(N_dot_H, 2.0) * (pow(alpha, 2.0) - 1.0) + 1.0, 2.0);

    denominator       = max(denominator, 0.000001);

    return numerator / denominator;
}

float G1(float alpha, vec3 N, vec3 X) {
    float numerator   = max(dot(N, X), 0.0);
    float k           = alpha / 2.0;
    float denominator = max(dot(N, X), 0.0) * (1.0 - k) + k;

    denominator       = max(denominator, 0.000001);

    return numerator / denominator;
}

float G(float alpha, vec3 N, vec3 V, vec3 L) {
    return G1(alpha, N, V) * G1(alpha, N, L);
}

vec3 F(vec3 F0, vec3 V, vec3 H) {
    return F0 + (vec3(1.0) - F0) * pow(1 - max(dot(V, H), 0.0), 5.0);
}

vec3 PBR(
    vec3  N,
    vec3  V,
    vec3  L,
    vec3  H,
    float alpha,
    float metallic,
    vec3  F0,
    vec3  albedo,
    vec3  emission,
    vec3  light_color
) {
    vec3  Ks                        = F(F0, V, H);
    vec3  Kd                        = (vec3(1.0) - Ks) * (1.0 - metallic);

    vec3  lambert                   = albedo / PI;

    vec3  cook_torrance_numerator   = D(alpha, N, H) * G(alpha, N, V, L) * F(F0, V, H);
    float cook_torrance_denominator = 4.0 * max(dot(V, N), 0.0) * max(dot(L, N), 0.0);

    cook_torrance_denominator       = max(cook_torrance_denominator, 0.000001);

    vec3  cook_torrance             = cook_torrance_numerator / cook_torrance_denominator;

    vec3  BRDF                      = Kd * lambert + cook_torrance;
    vec3  outgoing_light            = emission + BRDF * light_color * max(dot(L, N), 0.0);

    return outgoing_light;
}

void main() {
    vec3  albedo              = texture(samplers[ALBEDO],             inTexCoord).rgb;
    vec3  normal_map          = texture(samplers[NORMAL],             inTexCoord).xyz;
    vec2  metallic_roughness  = texture(samplers[METALLIC_ROUGHNESS], inTexCoord).bg;
    vec3  emission            = texture(samplers[EMISSION],           inTexCoord).rgb;
    float occlusion           = texture(samplers[OCCLUSION],          inTexCoord).r;

    albedo                   *= occlusion;

    float metallic            = metallic_roughness.x;
    float roughness           = metallic_roughness.y;
    float alpha               = roughness * roughness;

    vec3  projected_normal    = normalize(reflect(normal_map, inNormal));

    float in_normal_factor    = 0.7;
    float normal_map_factor   = 1.0 - in_normal_factor;

    vec3  normal              = normalize(inNormal * in_normal_factor + projected_normal * normal_map_factor);

    vec3  light_position      = vec3(0.0, 1.0, -0.85);
    vec3  light_color         = vec3(1.0);

    vec3  N                   = normalize(normal);
    vec3  V                   = normalize(inCameraPosition - inWorldPosition);
    vec3  L                   = normalize(light_position);
    vec3  H                   = normalize(V + L);

    vec3  F0_metallic         = vec3(0.90, 0.90, 0.80);
    vec3  F0_dielectric       = vec3(0.04, 0.04, 0.04);
    vec3  F0                  = mix(F0_dielectric, F0_metallic, metallic);

    vec3  pbr                 = PBR(N, V, L, H, alpha, metallic, F0, albedo, emission, light_color);

    outColor                  = vec4(pbr, 1.0);
}

