// dacho/assets/shaders/test/test.vert

#version 460 core

precision highp float;

layout(binding = 0) uniform UniformBufferObject {
    mat4  view;
    mat4  projection;
    vec3  _camera_pos;
    float _time;
} ubo;

layout(location = 0) in  vec3  inVertex;

layout(location = 1) in  float inInstance;

layout(location = 0) out vec3  outColor;

void main() {
    gl_Position = ubo.projection * ubo.view * vec4((inVertex + vec3(0.0, inInstance, 0.0)) * 15.0, 1.0);
    outColor    = inVertex;
}

