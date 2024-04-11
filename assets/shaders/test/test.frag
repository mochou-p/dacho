// dacho/assets/shaders/test/test.frag

#version 460

precision lowp float;

layout(location = 0) in  vec2 inUV;

layout(location = 0) out vec4 outColor;

void main() {
    vec2 xz = abs(inUV);

    outColor = vec4(vec3(0.02) + vec3(step(0.99, max(xz.x, xz.y)) * 0.3), 1.0);
}

