#version 330

out vec4 color;

in FragData {
    vec2 position;
    vec4 color;
} frag;

void main() {
	color = frag.color;
}
