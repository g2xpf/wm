#version 400 core

in vec2 v_uv;

out vec4 o_color;

uniform sampler2D u_glyph_texture;
uniform vec4 u_color;

const float EPS = 1e-6;

void main() {
    float gray_scale = texture(u_glyph_texture, v_uv).r;
    bool should_fill = gray_scale > EPS;

    if(should_fill){
        o_color = vec4(u_color.xyz, 1.0);
    } else {
        discard;
    }
}
