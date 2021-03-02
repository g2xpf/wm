#version 400 core

in vec2 v_uv;

out vec4 o_color;

uniform sampler2D u_glyph_texture;
uniform vec4 u_color;

void main() {
    float gray_scale = texture(u_glyph_texture, v_uv).r;
    o_color = vec4(u_color.rgb, min(1.0, gray_scale >= 1.0 ? 1.0 : 1.0 - pow(2.0, -10.0 * gray_scale)));
}
