// dacho/assets/shaders/tile/tile.frag

#version 460 core

precision lowp float;

layout(location = 0) in  vec4 inColor;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = inColor;
}

