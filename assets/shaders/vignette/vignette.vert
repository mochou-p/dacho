// dacho/assets/shaders/vignette/vignette.vert

#version 460 core

precision highp float;

layout(binding = 0) uniform UniformBufferObject {
    mat4  view;
    mat4  projection;
    vec3  camera_pos;
    float time;
} ubo;

layout(location = 0) in  vec2 inPosition;

layout(location = 1) in  vec3 _;

layout(location = 0) out vec2 outPosition;

void main() {
    gl_Position = vec4(inPosition, 0.0, 1.0);
    outPosition = inPosition;
}

