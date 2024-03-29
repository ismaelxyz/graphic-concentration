#version 330

layout(location = 0) in vec3 vertex_position;
// texture coordinates attribute
layout(location = 1) in vec3 tex_coords; 

out vec3 local_position;
// interpolated texture coordinates
out vec3 interpolated_tex_coords;

uniform mat4 matrix;

void main(void) {
	local_position = vertex_position;
	interpolated_tex_coords = tex_coords;
	gl_Position = matrix * vec4(vertex_position, 1.0);
}
