// dacho/assets/shaders/sky/sky.vert

#version 460 core

precision highp float;

layout(binding = 0) uniform UniformBufferObject {
    mat4  view;
    mat4  projection;
    vec3  camera_pos;
    float time;
} ubo;

layout(location = 0) in  vec3  inPosition;

layout(location = 1) in  vec3  _;

layout(location = 0) out vec3  outPosition;

void main() { 
    gl_Position = ubo.projection * ubo.view * vec4(inPosition + ubo.camera_pos, 1.0);
    outPosition = inPosition;
}
