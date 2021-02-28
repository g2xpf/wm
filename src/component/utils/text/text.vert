#version 400 core

uniform vec2 u_resolution;
uniform vec2 u_position;
uniform float u_scale_factor;

in vec2 a_uv;
in ivec2 a_position;

out vec2 v_uv;

void main() {
    vec2 physical_position = vec2(u_position.x * u_scale_factor, u_resolution.y - u_position.y * u_scale_factor);
    vec2 scaled_position = 2.0 * ((vec2(a_position.x, -a_position.y) + physical_position) / u_resolution - vec2(0.5));
    v_uv = a_uv;
    gl_Position = vec4(scaled_position, 0.0, 1.0);
}
