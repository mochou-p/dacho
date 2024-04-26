// dacho/assets/shaders/test/test.frag

#version 460 core

precision lowp float;

layout(binding = 1) uniform sampler2D samplers;

layout(location = 0) in  vec3 inNormal;
layout(location = 1) in  vec2 inTexCoord;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = texture(samplers, inTexCoord);
}

