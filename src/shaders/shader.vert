#version 330

attribute vec2 position;
attribute vec4 color;
attribute vec2 tex_coord;

out FragData {
    vec2 position;
    vec4 color;
    vec2 tex_coord;
} frag;

void main() {
	gl_Position = vec4(position, 0.0, 1.0);

	frag.position = position;
	frag.color = color;
	frag.tex_coord = tex_coord;
}
