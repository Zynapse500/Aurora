#version 330

attribute vec2 position;
attribute vec4 color;
attribute vec2 tex_coord;


uniform float left, right, top, bottom;


out FragData {
    vec2 position;
    vec4 color;
    vec2 tex_coord;
} frag;

void main() {
    vec2 transformed = vec2(
        (position.x - left) / (right - left) * 2.0 - 1.0,
        (position.y - bottom) / (top - bottom) * 2.0 - 1.0
    );

	gl_Position = vec4(transformed, 0.0, 1.0);

	frag.position = position;
	frag.color = color;
	frag.tex_coord = tex_coord;
}
