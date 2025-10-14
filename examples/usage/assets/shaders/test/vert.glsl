// dacho/examples/usage/assets/shaders/test/vert.glsl

#version 460

#extension GL_EXT_buffer_reference                       : require
#extension GL_EXT_scalar_block_layout                    : require
#extension GL_EXT_shader_explicit_arithmetic_types_int32 : require
#extension GL_EXT_shader_explicit_arithmetic_types_int64 : require


struct Vertex {
    vec2 position;
};

struct Instance {
    vec2 position;
};

layout(buffer_reference, scalar) buffer VertexBuffer {
    Vertex data[];
};

layout(buffer_reference, scalar) buffer InstanceBuffer {
    Instance data[];
};

layout(buffer_reference, scalar) buffer IndexBuffer {
    uint32_t data[];
};

layout(push_constant) uniform push {
    uint64_t  vertices_pointer;
    uint64_t instances_pointer;
    uint64_t   indices_pointer;
};

layout(location = 0) out vec3 out_color;


const vec3 colors[3] = vec3[](
    vec3(0.0, 1.0, 1.0),
    vec3(1.0, 0.0, 1.0),
    vec3(1.0, 1.0, 0.0)
);

void main() {
    IndexBuffer       index_buffer =    IndexBuffer(  indices_pointer);
    InstanceBuffer instance_buffer = InstanceBuffer(instances_pointer);
    VertexBuffer     vertex_buffer =   VertexBuffer( vertices_pointer);

    uint32_t vertex_index =    index_buffer.data[  gl_VertexIndex];
    Vertex   vertex       =   vertex_buffer.data[    vertex_index];
    Instance instance     = instance_buffer.data[gl_InstanceIndex];

    vec2 position = vertex.position + instance.position;

    gl_Position = vec4(position, 0.0, 1.0);
    out_color   = colors[gl_VertexIndex % 3];
}

