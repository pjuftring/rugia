#version 100

precision lowp float;

varying vec3 pos_out;
varying vec2 tex_out;

uniform sampler2D tex;

void main() {
    gl_FragColor = texture2D(tex, tex_out);
}
