#version 400 core

uniform vec2 u_resolution;
uniform vec4 u_color;
uniform float u_round_radius;
uniform vec2 u_center_position;
uniform vec2 u_button_size;
uniform float u_scale_factor;
uniform bool u_pressed;

in vec2 v_position;

out vec4 o_color;

void main() {
    float round_radius = u_round_radius * u_scale_factor;
    vec2 center_position = vec2(u_center_position.x * u_scale_factor, u_resolution.y - u_center_position.y * u_scale_factor);
    vec2 button_size = u_button_size * u_scale_factor;

    vec2 p = abs(gl_FragCoord.xy - center_position);
    vec2 q = min(button_size * .5 - vec2(round_radius), p);
    float l = round_radius - length(p - q);
    if(l > 0) {
        vec4 color = u_pressed ? u_color * 0.7 : u_color;
        o_color = color;
    } else {
        discard;
    }
}

