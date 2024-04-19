// dacho/assets/shaders/sky/sky.frag

#version 460

precision lowp float;

layout(location = 0) in  vec3 inColor;

layout(location = 0) out vec4 outColor;

void main() {
    outColor     = vec4(inColor, 1.0);
    gl_FragDepth = 0.99999997;
}

