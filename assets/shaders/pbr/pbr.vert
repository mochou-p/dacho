// dacho/assets/shaders/pbr/pbr.vert

#version 460 core

precision highp float;

layout(binding = 0) uniform UniformBufferObject {
    mat4  view;
    mat4  projection;
    vec3  camera_position;
    float time;
} uUBO;

layout(location = 0) in  vec3  inPosition;
layout(location = 1) in  vec3  inNormal;
layout(location = 2) in  vec2  inTexCoord;

layout(location = 3) in  float inInstance;

layout(location = 0) out vec3  outWorldPosition;
layout(location = 1) out vec3  outNormal;
layout(location = 2) out vec2  outTexCoord;
layout(location = 3) out vec3  outCameraPosition;

void main() {
    mat3 rotation_x      = mat3(
         1.0, 0.0,  0.0,
         0.0, 0.0, -1.0,
         0.0, 1.0,  0.0
    );

    mat3 rotation_y      = mat3(
        -1.0, 0.0,  0.0,
         0.0, 1.0,  0.0,
         0.0, 0.0, -1.0
    );

    vec3 fixed_position  = mat3(rotation_y * rotation_x) * inPosition;
    fixed_position.y    *= -1.0;
    vec4 position        = vec4(fixed_position, 1.0);

    gl_Position          = uUBO.projection * uUBO.view * position;

    outWorldPosition     = (uUBO.view * position).xyz;
    outNormal            = inNormal;
    outTexCoord          = inTexCoord;
    outCameraPosition    = uUBO.camera_position;
}

