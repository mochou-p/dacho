// dacho/assets/shaders/skybox/skybox.frag

#version 460 core

precision lowp float;

#define TAU          6.2831853076

#define SKYBOX_DEPTH 0.99999997

layout(binding = 1) uniform sampler2D uSkyboxSampler;

layout(location = 0) in  vec3 inPosition;

layout(location = 0) out vec4 outColor;

vec2 sphere_uv(vec3 view_dir) {
    return vec2(
        atan(view_dir.z, view_dir.x) / TAU + 0.5,
        1.0 - (view_dir.y * 0.5 + 0.5)
    );
}

void main() {
    vec3 view_dir  = normalize(inPosition);
    vec2 tex_coord = sphere_uv(view_dir);

    gl_FragDepth   = SKYBOX_DEPTH;
    outColor       = texture(uSkyboxSampler, tex_coord);
}

