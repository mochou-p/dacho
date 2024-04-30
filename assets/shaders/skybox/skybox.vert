// dacho/assets/shaders/skybox/skybox.vert

#version 460 core

precision highp float;

layout(binding = 0) uniform UniformBufferObject {
    mat4  view;
    mat4  projection;
    vec3  camera_position;
    float time;
} uUBO;

layout(location = 0) in  vec3  inPosition;

layout(location = 1) in  float inInstance;

layout(location = 0) out vec3  outPosition;

void main() { 
    gl_Position = uUBO.projection * uUBO.view * vec4(inPosition + uUBO.camera_position, 1.0);
    outPosition = inPosition;
}

