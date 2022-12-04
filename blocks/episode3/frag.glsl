// specify we are indeed using modern opengl
#version 330 core

// output of our shader
out vec4 fragment_colour; 

// interpolated vertex position
in vec3 local_position;  

void main(void) {
	// set the output colour based on the vertex position
	fragment_colour = vec4(local_position / 2.0 + 0.5, 1.0);
}