#version 130

precision lowp float;

in vec3 pos_out;
in vec2 tex_out;

uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, tex_out);
}
