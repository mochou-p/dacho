// dacho/examples/usage/assets/shaders/test/vert.glsl

#version 460

layout(location = 0) out vec3 out_color;

const vec2 positions[3] = vec2[](
    vec2( 0.0, -0.5),
    vec2( 0.5,  0.5),
    vec2(-0.5,  0.5)
);

const vec3 colors[3] = vec3[](
    vec3(0.95294, 0.54509, 0.65882),
    vec3(0.65098, 0.89019, 0.63137),
    vec3(0.53725, 0.70588, 0.98039)
);

void main() {
    gl_Position = vec4(positions[gl_VertexIndex], 0.5, 1.0);
    out_color   = colors[gl_VertexIndex];
}

