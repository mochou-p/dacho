// dacho/assets/shaders/sky/sky.frag

#version 460 core

precision lowp float;

layout(location = 0) in  vec3  inPosition;

layout(location = 0) out vec4  outColor;

float from_to(float v, float a, float b, float c, float d) {
    float scale   = (d - c) / (b - a);
    float clamped = clamp(v, a, b);

    return c + (clamped - a) * scale;
}

void main() {
    gl_FragDepth   = 0.99999997;

    vec3  viewDir  = normalize(inPosition);
    vec3  sunDir   = vec3(0.0, 1.0, -1.0);

    vec3  sunColor = vec3(1.0, 0.9, 0.8);
    vec3  skyColor = vec3(0.3, 0.7, 1.0);

    float sunInt   = from_to(pow(max(dot(viewDir, sunDir) * 0.61, 0.0), 50.0) * 100.0, 0.0, 0.1, 0.0, 1.0);

    vec3  sun      = sunInt * sunColor;
    vec3  sky      = skyColor;

    vec3  color    = sun + sky;

    outColor       = vec4(color, 1.0);
}

