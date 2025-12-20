// dacho/examples/usage/assets/shaders/test/comp.glsl

#version 460

#extension GL_EXT_buffer_reference                       : require
#extension GL_EXT_scalar_block_layout                    : require
#extension GL_EXT_shader_explicit_arithmetic_types_int32 : require
#extension GL_EXT_shader_explicit_arithmetic_types_int64 : require


layout(local_size_x = 128, local_size_y = 1, local_size_z = 1) in;

struct Instance {
    vec2 position;
};

layout(buffer_reference, scalar) buffer InstanceBuffer {
    Instance data[];
};

layout(push_constant) uniform PushConstant {
    uint64_t  vertices_pointer;
    uint64_t   indices_pointer;
    uint64_t instances_pointer;
    uint32_t      index_offset;
} pc;


void main() {
    InstanceBuffer instance_buffer = InstanceBuffer(pc.instances_pointer);

    instance_buffer.data[gl_GlobalInvocationID.x].position += vec2(0.0005);
}

