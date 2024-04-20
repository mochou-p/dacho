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

mat4 rotate_y(float angle) {
    vec3  a = normalize(vec3(0.0, 1.0, 0.0));
    float s = sin(angle);
    float c = cos(angle);
    float o = 1.0 - c;
    
    return mat4(
        o * a.x * a.x + c,       o * a.x * a.y - a.z * s, o * a.z * a.x + a.y * s, 0.0,
        o * a.x * a.y + a.z * s, o * a.y * a.y + c,       o * a.y * a.z - a.x * s, 0.0,
        o * a.z * a.x - a.y * s, o * a.y * a.z + a.x * s, o * a.z * a.z + c,       0.0,
        0.0,                     0.0,                     0.0,                     1.0
    );
}

void main() {
    float angle    = noise(inInstancePosition.xz) * 3.14159;
    vec3 rotated   = (rotate_y(angle) * vec4(inPosition, 1.0)).xyz;
    vec3 position  = rotated + inInstancePosition;
    float posHash  = noise(position.xz);
    float offset   = sin((ubo.time * 2.71828) + posHash);
    position.xz   += pow(inPosition.y * 0.2, 2.0) * offset * 1.5;

    gl_Position    = ubo.projection * ubo.view * vec4(position, 1.0);
    outColor       = vec3(0.05, 0.8, 0.1) - (posHash * 0.5);
}

