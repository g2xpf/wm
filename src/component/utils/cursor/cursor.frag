#version 400 core

uniform int u_cursor_type;
uniform vec4 u_color;

in float v_position_x;

out vec4 o_color;

void main() {
    if(u_cursor_type == 0) {
        // line
        if(v_position_x <= 2.0) {
            o_color = u_color;
        } else {
            discard;
        }
    } else {
        // box
        o_color = vec4(u_color.rgb, 0.3);
    }
}

