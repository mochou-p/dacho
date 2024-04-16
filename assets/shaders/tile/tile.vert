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
layout(location = 1) out vec3 outColor;

float hash(vec2 position) {
    return fract(sin(dot(position, vec2(12.9898, 78.233))) * 43758.5453);
}

void main() {
    gl_Position = ubo.projection * ubo.view * ubo.model * vec4(inPosition.xyz + inInstancePosition, 1.0);

    outUV       = vec2(inPosition.x, -inPosition.z) * inPosition.w;
    outColor    = vec3(hash(inPosition.xz), hash(inInstancePosition.xz), hash(vec2(inPosition.y, inInstancePosition.y)));
}

