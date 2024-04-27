// dacho/assets/shaders/sky/sky.frag

#version 460 core

precision lowp float;

#define SKYBOX_DEPTH 0.99999997

layout(location = 0) out vec4 outColor;

void main() {
    gl_FragDepth = SKYBOX_DEPTH;
    outColor     = vec4(vec3(0.5), 1.0);
}

