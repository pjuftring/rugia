#version 130

in vec4 pos_in;
in vec2 tex_in;

out vec3 pos_out;
out vec2 tex_out;

uniform mat4 camera;
uniform mat4 light;
uniform mat4 model;

void main() {
    pos_out = pos_in.xyz;
    tex_out = vec2(tex_in.x, 1. - tex_in.y);
    gl_Position = camera * model * pos_in;
}
