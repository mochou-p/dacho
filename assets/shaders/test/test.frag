#version 460

layout(location = 0) in  vec3 vertexColor;
layout(location = 0) out vec4 outColor;

const vec3 gammaCorrection = vec3(0.45454545454);

void main() {
    vec3 fixedColor = pow(vertexColor, gammaCorrection);

    outColor = vec4(fixedColor, 1.0);
}

