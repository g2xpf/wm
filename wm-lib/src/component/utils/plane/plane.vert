#version 400 core

uniform vec2 u_resolution;
uniform float u_scale_factor;
uniform vec2 u_position;

in vec2 a_position;

out vec2 v_position;

void main() {
    vec2 physical_position = vec2((a_position.x + u_position.x) * u_scale_factor, u_resolution.y - (a_position.y + u_position.y) * u_scale_factor);
    vec2 scaled_position = (physical_position / u_resolution) * 2.0 - vec2(1.0);

    v_position = physical_position;

    gl_Position = vec4(scaled_position, 0.0, 1.0);
}
