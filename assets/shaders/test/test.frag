// dacho/assets/shaders/test/test.frag

#version 460 core

precision lowp float;

layout(binding = 1) uniform sampler2D samplers[2];

layout(location = 0) in  vec3 inNormal;
layout(location = 1) in  vec2 inTexCoord;

layout(location = 0) out vec4 outColor;

#define ALBEDO 0
#define NORMAL 1

void main() {
    vec3 normal = normalize(inNormal * 0.7 + texture(samplers[NORMAL], inTexCoord).xyz * 0.3);

    float light = max(dot(normal, vec3(0.0, 1.0, -1.0)), 0.0);
    light *= 0.98;
    light += 0.02;

    outColor    = vec4(texture(samplers[ALBEDO], inTexCoord).xyz * light, 1.0);
}

