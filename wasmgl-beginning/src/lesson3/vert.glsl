#version 300 es // specify we are indeed using modern opengl

// vertex position attribute
layout(location = 0) in vec3 vertex_position; 

// interpolated vertex position
out vec3 local_position; 

void main(void) {
	local_position = vertex_position;
	// set vertex position
	gl_Position = vec4(vertex_position, 1.0); 
}
