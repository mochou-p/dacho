// dacho/assets/shaders/grass/grass.frag

#version 460

precision lowp float;

layout(location = 0) out vec4 outColor;

void main() {
    outColor = vec4(0.1, 0.4, 0.05, 1.0);
}

