#version 100

precision lowp float;

varying vec4 color;
varying vec2 uv;

uniform sampler2D Texture;
uniform sampler2D HeightMap;
uniform sampler2D NormalMap;

void main() {
    vec3 res = texture2D(Texture, uv).rgb * color.rgb;
    gl_FragColor = vec4(res, 1.0);
}