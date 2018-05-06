#version 330

in vec2 position;
in vec4 color;

out FragData {
    vec2 position;
    vec4 color;
} frag;

void main() {
	gl_Position = vec4(position, 0.0, 1.0);

	frag.position = position;
	frag.color = color;
}
