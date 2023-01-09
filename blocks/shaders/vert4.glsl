#version 330 core

layout(location = 0) in vec3 vertex_position;

out vec3 local_position;
// create matrix uniform variable
uniform mat4 matrix;

void main(void) {
	local_position = vertex_position;
	// multiply matrix by vertex_position vector
	gl_Position = matrix * vec4(vertex_position, 1.0); 
}