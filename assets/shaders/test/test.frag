// dacho/assets/shaders/test/test.frag

#version 460

layout(location = 0) in  vec3 inColor;
layout(location = 1) in  vec3 inNormal;

layout(location = 0) out vec4 outColor;

const vec3 directionalLight = vec3(-0.17, -0.83, 0.44);

void main() {
    float minIntensity = 0.2;

    float diffuse = (dot(inNormal, -directionalLight) + 1.0) * 0.5 * (1.0 - minIntensity) + minIntensity;

    outColor = vec4(inColor * diffuse, 1.0);
}

