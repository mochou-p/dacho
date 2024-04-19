// dacho/assets/shaders/grass/grass.vert

#version 460

precision highp float;

layout(binding = 0) uniform UniformBufferObject {
    mat4 view;
    mat4 projection;
    vec3 camera_pos;
} ubo;

layout(location = 0) in vec3 inPosition;

layout(location = 1) in vec3 inInstancePosition;

void main() {
    gl_Position = ubo.projection * ubo.view * vec4(inPosition + inInstancePosition, 1.0);
}

