// dacho/assets/shaders/vignette/vignette.vert

#version 460 core

precision highp float;

layout(location = 0) in  vec2  inVertex;

layout(location = 1) in  float _inInstance;

layout(location = 0) out vec2  outUV;

void main() {
    gl_Position = vec4(inVertex, 0.0, 1.0);
    outUV       = inVertex;
}

