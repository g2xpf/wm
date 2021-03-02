#version 400 core

in vec2 v_uv;

out vec4 o_color;

uniform sampler2D u_glyph_texture;
uniform vec4 u_color;

const float EPS = 1. / 256.;

void main() {
    float gray_scale = texture(u_glyph_texture, v_uv).r;
    o_color = vec4(u_color.rgb, gray_scale);
}
