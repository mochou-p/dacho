// dacho/assets/shaders/tile/tile.vert

#version 460 core

precision highp float;

layout(binding = 0) uniform UniformBufferObject {
    mat4  view;
    mat4  projection;
    vec3  camera_pos;
    float time;
} ubo;

layout(location = 0) in  vec4 inPosition;

layout(location = 1) in  vec3 inInstancePosition;

layout(location = 0) out vec4 outColor;

void main() {
    gl_Position = ubo.projection * ubo.view * vec4(inPosition.xyz + inInstancePosition, 1.0);
    outColor    = vec4(vec3(mod((inInstancePosition.x + inInstancePosition.z) * inPosition.w, 2.0)) * 0.006 + 0.01, 1.0);
}

