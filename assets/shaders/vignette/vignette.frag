// dacho/assets/shaders/vignette/vignette.frag

#version 460 core

precision lowp float;

layout(location = 0) in  vec2 inUV;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(vec3(0.0), smoothstep(0.0, 1.8, length(inUV - vec2(0.0))));
}

