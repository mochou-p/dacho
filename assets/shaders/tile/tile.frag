// dacho/assets/shaders/tile/tile.frag

#version 460

precision lowp float;

layout(location = 0) in  vec2 inUV;

layout(location = 0) out vec4 outColor;

void main() {
    vec2 xz = abs(inUV);

    outColor = vec4(vec3(smoothstep(0.989, 0.999, max(xz.x, xz.y))), 1.0);
}

