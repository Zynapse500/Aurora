#version 330

out vec4 color;

uniform sampler2D tex0;

in FragData {
    vec2 position;
    vec4 color;
    vec2 tex_coord;
} frag;

void main() {
	color = frag.color * texture(tex0, frag.tex_coord);
}
