// dacho/assets/shaders/test/test.vert

#version 460

layout(binding = 0) uniform UniformBufferObject {
    mat4 model;
    mat4 view;
    mat4 projection;
} ubo;

layout(location = 0) in  vec3 inPosition;
layout(location = 1) in  vec3 inColor;
layout(location = 2) in  uint inNormalIndex;

layout(location = 3) in  vec3 inInstancePosition;

layout(location = 0) out vec3 outColor;
layout(location = 1) out vec3 outNormal;

const vec3[6] normals = vec3[6](
    vec3( 0.0,  1.0,  0.0),
    vec3( 0.0, -1.0,  0.0),
    vec3(-1.0,  0.0,  0.0),
    vec3( 1.0,  0.0,  0.0),
    vec3( 0.0,  0.0,  1.0),
    vec3( 0.0,  0.0, -1.0)
);

void main() {
    gl_Position = ubo.projection * ubo.view * ubo.model * vec4(inPosition + inInstancePosition, 1.0);
    outColor    = mod(inInstancePosition * 0.02, 1.0);
    outNormal   = normals[inNormalIndex];
}

