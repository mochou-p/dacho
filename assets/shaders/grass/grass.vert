// dacho/assets/shaders/grass/grass.vert

#version 460 core

precision highp float;

layout(binding = 0) uniform UniformBufferObject {
    mat4  view;
    mat4  projection;
    vec3  _camera_pos;
    float time;
} ubo;

layout(location = 0) in  vec2 inVertex;

layout(location = 1) in  vec2 inInstance;

layout(location = 0) out vec4 outColor;

mat4 rotate_y(float angle) {
    vec3  a = normalize(vec3(0.0, 1.0, 0.0));
    float s = sin(angle);
    float c = cos(angle);
    float o = 1.0 - c;
    
    return mat4(
        o * a.x * a.x + c,       o * a.x * a.y - a.z * s, o * a.z * a.x + a.y * s, 0.0,
        o * a.x * a.y + a.z * s, o * a.y * a.y + c,       o * a.y * a.z - a.x * s, 0.0,
        o * a.z * a.x - a.y * s, o * a.y * a.z + a.x * s, o * a.z * a.z + c,       0.0,
        0.0,                     0.0,                     0.0,                     1.0
    );
}

vec4 permute(vec4 xyzw) {
    return mod(((xyzw * 34.0) + 1.0) * xyzw, 289.0);
}

vec2 fade(vec2 xy) {
    return xy * xy * xy * (xy * (xy * 6.0 - 15.0) + 10.0);
}

float noise(vec2 xy) {
    vec4 Pi = floor(xy.xyxy) + vec4(0.0, 0.0, 1.0, 1.0);
    vec4 Pf = fract(xy.xyxy) - vec4(0.0, 0.0, 1.0, 1.0);

    Pi = mod(Pi, 289.0);

    vec4 ix = Pi.xzxz;
    vec4 iy = Pi.yyww;
    vec4 fx = Pf.xzxz;
    vec4 fy = Pf.yyww;
    vec4 i  = permute(permute(ix) + iy);
    vec4 gx = 2.0 * fract(i * 0.0243902439) - 1.0;
    vec4 gy = abs(gx) - 0.5;
    vec4 tx = floor(gx + 0.5);

    gx = gx - tx;

    vec2 g00 = vec2(gx.x, gy.x);
    vec2 g10 = vec2(gx.y, gy.y);
    vec2 g01 = vec2(gx.z, gy.z);
    vec2 g11 = vec2(gx.w, gy.w);

    vec4 norm = 1.79284291400159
        - 0.85373472095314 * vec4(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));

    g00 *= norm.x;
    g01 *= norm.y;
    g10 *= norm.z;
    g11 *= norm.w;

    float n00 = dot(g00, vec2(fx.x, fy.x));
    float n10 = dot(g10, vec2(fx.y, fy.y));
    float n01 = dot(g01, vec2(fx.z, fy.z));
    float n11 = dot(g11, vec2(fx.w, fy.w));

    vec2 fade_xy = fade(Pf.xy);

    vec2  n_x  = mix(vec2(n00, n01), vec2(n10, n11), fade_xy.x);
    float n_xy = mix(n_x.x, n_x.y, fade_xy.y);

    return 2.3 * n_xy;
}

void main() {
    float angle      = noise(inInstance) * 3.14159;
    vec3 rotated     = (rotate_y(angle) * vec4(inVertex, 0.0, 1.0)).xyz;
    vec3 instancePos = vec3(inInstance.x, noise(inInstance * 0.01) * 20.0, inInstance.y);
    vec3 position    = rotated + instancePos;
    float posHash    = noise(position.xz);
    float offset     = sin((ubo.time * 2.71828) + posHash);
    position.xz     += pow(inVertex.y * 0.2, 2.0) * offset * 1.5;

    gl_Position      = ubo.projection * ubo.view * vec4(position, 1.0);
    outColor         = vec4(vec3(0.05, 0.8, 0.1) - (posHash * 0.5) - vec3(min(-position.y * 0.025, 0.0)) * 0.5, 1.0);
}

