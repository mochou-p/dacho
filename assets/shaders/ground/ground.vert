// dacho/assets/shaders/ground/ground.vert

#version 460 core

precision highp float;

layout(location = 0) in  vec2  inVertex;

layout(location = 1) in  float _inInstance;

void main() {
    gl_Position = vec4(inVertex.x, 0.0, inVertex.y, 1.0);
}

