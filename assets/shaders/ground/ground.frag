// dacho/assets/shaders/ground/ground.frag

#version 460 core

precision lowp float;

layout(location = 0) in  float inHeight;

layout(location = 0) out vec4  outColor;

void main() {
    outColor = vec4(vec3(0.0, 1.0, 0.0) - vec3(-inHeight), 1.0);
}

