// dacho/examples/usage/assets/shaders/test/vert.glsl

#version 460

#extension GL_EXT_buffer_reference                       : require
#extension GL_EXT_scalar_block_layout                    : require
#extension GL_EXT_shader_explicit_arithmetic_types_int64 : require


struct Vertex {
    vec4 position;
};

layout(buffer_reference, scalar) buffer VertexBuffer {
    Vertex verts[];
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
    VertexBuffer vertex_buffer = VertexBuffer(vertices_pointer);
    Vertex       vertex        = vertex_buffer.verts[gl_VertexIndex];

    gl_Position = vertex.position;
    out_color   = colors[gl_VertexIndex % 3];
}

