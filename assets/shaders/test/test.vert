// dacho/assets/shaders/test/test.vert

#version 460 core

precision highp float;

layout(binding = 0) uniform UniformBufferObject {
    mat4  view;
    mat4  projection;
    vec3  _camera_pos;
    float _time;
} ubo;

layout(location = 0) in  vec3  inPosition;
layout(location = 1) in  vec3  inNormal;
layout(location = 2) in  vec2  inTexCoord;

layout(location = 3) in  float inInstance;

layout(location = 0) out vec3  outNormal;
layout(location = 1) out vec2  outTexCoord;

void main() {
    gl_Position = ubo.projection * ubo.view * vec4((inPosition * 25.0 + vec3(0.0, inInstance * 10.0, 0.0)), 1.0);
    outNormal   = inNormal;
    outTexCoord = inTexCoord;
}

