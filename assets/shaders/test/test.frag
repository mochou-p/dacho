// dacho/assets/shaders/test/test.frag

#version 460 core

precision lowp float;

layout(location = 0) in  vec3 inNormal;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(vec3(max(dot(inNormal, vec3(0.0, 1.0, -1.0)), 0.0)) * 0.95 + vec3(0.05), 1.0);
}

