// dacho/assets/shaders/skybox/skybox.frag

#version 460 core

precision lowp float;

#define CUBE_FACES   6
#define SKYBOX_DEPTH 0.99999997

layout(binding = 1) uniform sampler2D uSamplers[CUBE_FACES];

layout(location = 0) in  vec3 inPosition;

layout(location = 0) out vec4 outColor;

void main() {
    float ax = abs(inPosition.x);
    float ay = abs(inPosition.y);
    float az = abs(inPosition.z);

    int  face;
    vec2 tex_coord;

    if (ax >= ay && ax >= az) {
        face      = int(ceil(inPosition.x * 0.1));
        tex_coord = vec2(
            1.0 - abs((inPosition.x + 1.0) * 0.5 - (inPosition.z + 1.0) * 0.5),
            1.0 - (inPosition.y + 1.0) * 0.5
        );
    } else if (ay >= ax && ay >= az) {
        face      = int(ceil(inPosition.y * 0.1)) + 2;
        tex_coord = abs(
            vec2(1.0) * abs(max(inPosition.y, 0.0))
            - vec2(
                abs((inPosition.y + 1.0) * 0.5 - (inPosition.x + 1.0) * 0.5),
                (inPosition.z + 1.0) * 0.5
            )
        );
    } else {
        face      = int(ceil(inPosition.z * 0.1)) + 4;
        tex_coord = vec2(
            abs((inPosition.z + 1.0) * 0.5 - (inPosition.x + 1.0) * 0.5),
            1.0 - (inPosition.y + 1.0) * 0.5
        );
    }

    gl_FragDepth = SKYBOX_DEPTH;
    outColor = texture(uSamplers[face], tex_coord);
}

