// dacho/assets/shaders/tile/tile.vert

#version 460

precision highp float;

layout(binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 projection;
} ubo;

layout(location = 0) in  vec4 inPosition;
layout(location = 1) in  vec3 inInstancePosition;

layout(location = 0) out vec2 outUV;

void main() {
    gl_Position = ubo.projection * ubo.view * ubo.model * vec4(inPosition.xyz + inInstancePosition, 1.0);
    outUV       = vec2(inPosition.x, -inPosition.z) * inPosition.w;
}

