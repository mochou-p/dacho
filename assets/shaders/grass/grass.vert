// dacho/assets/shaders/grass/grass.vert

#version 460 core

precision highp float;

layout(binding = 0) uniform UniformBufferObject {
    mat4  view;
    mat4  projection;
    vec3  camera_pos;
    float time;
} ubo;

layout(location = 0) in  vec3 inPosition;

layout(location = 1) in  vec3 inInstancePosition;

layout(location = 0) out vec3 outColor;

float noise(vec2 xy) {
    return fract(sin(dot(xy, vec2(12.9898, 4.1414))) * 43758.5453);
}

void main() {
    vec3 position  = (inPosition + inInstancePosition);
    float posHash  = noise(inInstancePosition.xz * 0.3);
    float offset   = sin(ubo.time + posHash);
    position.xz   += pow(inPosition.y * 0.2, 2.0) * offset;

    gl_Position    = ubo.projection * ubo.view * vec4(position, 1.0);
    outColor       = vec3(0.2, 0.8 + posHash, noise(inInstancePosition.xz + vec2(0.2)) * 0.2);
}

