#version 400 core

uniform float u_scale_factor;
uniform vec2 u_resolution;
uniform vec2 u_position;

in vec2 a_position;

out float v_position_x;

void main() {
    vec2 physical_position = vec2(u_position.x, u_resolution.y - u_position.y) + vec2(a_position.x, -a_position.y) * u_scale_factor;
    vec2 scaled_position = 2. * physical_position / u_resolution - vec2(1.);

    v_position_x = a_position.x * u_scale_factor;

    gl_Position = vec4(scaled_position, 0.0, 1.0);
}
