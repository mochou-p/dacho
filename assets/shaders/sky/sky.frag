// dacho/assets/shaders/sky/sky.frag

#version 460 core

precision lowp float;

layout(location = 0) out vec4 outColor;

void main() {
    gl_FragDepth = 0.99999997;
    outColor     = vec4(0.3, 0.7, 1.0, 1.0);
}

