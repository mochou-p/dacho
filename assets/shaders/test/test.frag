// dacho/assets/shaders/test/test.frag

#version 460

layout(location = 0) in  vec3 inColor;
layout(location = 1) in  vec3 inNormal;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(mix(inColor, inNormal, 0.5), 1.0);
}

