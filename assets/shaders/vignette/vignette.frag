// dacho/assets/shaders/vignette/vignette.frag

#version 460 core

precision lowp float;

layout(location = 0) in  vec2 inPosition;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(vec3(0.0), smoothstep(0.0, 1.75, length(inPosition - vec2(0.0))));
}

