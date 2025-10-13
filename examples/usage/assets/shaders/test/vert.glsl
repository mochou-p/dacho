// dacho/examples/usage/assets/shaders/test/vert.glsl

#version 460

#extension GL_EXT_shader_explicit_arithmetic_types_int64 : require
#extension GL_EXT_buffer_reference                       : require


#define VERTEX_SIZE (3*(32/8))
layout(buffer_reference) buffer Vertex {
    vec3 position;
};

layout(push_constant) uniform push {
    uint64_t vertices_pointer;
};

layout(location = 0) out vec3 out_color;


const vec3 colors[3] = vec3[](
    vec3(1.0, 0.0, 0.0),
    vec3(0.0, 1.0, 0.0),
    vec3(0.0, 0.0, 1.0)
);

void main() {
    uint     vertex_offset  = gl_VertexIndex * uint(VERTEX_SIZE);
    uint64_t vertex_pointer = vertices_pointer + vertex_offset;
    Vertex   vertex         = Vertex(vertex_pointer);

    gl_Position = vec4(vertex.position, 1.0);
    out_color   = colors[gl_VertexIndex];
}

