// dacho/assets/shaders/ground/ground.tese

#version 460 core

precision highp float;

layout(quads, equal_spacing, ccw) in;

layout(binding = 0) uniform UniformBufferObject {
    mat4  view;
    mat4  projection;
    vec3  _camera_pos;
    float _time;
} ubo;

layout(location = 0) out float outHeight;

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

void main()
{
    float u = gl_TessCoord.x;
    float v = gl_TessCoord.y;

    vec4 p00 = gl_in[0].gl_Position;
    vec4 p01 = gl_in[1].gl_Position;
    vec4 p10 = gl_in[2].gl_Position;
    vec4 p11 = gl_in[3].gl_Position;

    vec4 p    = mix(mix(p00, p01, u), mix(p10, p11, u), v);
    outHeight = noise(p.xz * 0.01);
    p.y       = outHeight * 20.0;

    gl_Position = ubo.projection * ubo.view * p;
}

