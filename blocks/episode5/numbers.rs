
pub const VERTEX_POSITIONS: [f32; 72] = [
	 0.5,  0.5,  0.5,  0.5, -0.5,  0.5,  0.5, -0.5, -0.5,  0.5,  0.5, -0.5,
	-0.5,  0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5,  0.5, -0.5,  0.5,  0.5,
	-0.5,  0.5,  0.5, -0.5,  0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,  0.5,
	-0.5, -0.5,  0.5, -0.5, -0.5, -0.5,  0.5, -0.5, -0.5,  0.5, -0.5,  0.5,
	-0.5,  0.5,  0.5, -0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,  0.5,  0.5,
	 0.5,  0.5, -0.5,  0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5,  0.5, -0.5,
];

pub const TEX_COORDS: [f32; 72] = [
	0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
	0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
	0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
	0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
	0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
	0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
];

pub const SHADING: [f32; 24] = [
	0.80, 0.80, 0.80, 0.80,
	0.80, 0.80, 0.80, 0.80,
	1.00, 1.00, 1.00, 1.00,
	0.49, 0.49, 0.49, 0.49,
	0.92, 0.92, 0.92, 0.92,
	0.92, 0.92, 0.92, 0.92,
];

pub const INDICES: [f32; 36] = [
	 0.0,  1.0,  2.0,  0.0,  2.0,  3.0, // right
	 4.0,  5.0,  6.0,  4.0,  6.0,  7.0, // left
	 8.0,  9.0, 10.0,  8.0, 10.0, 11.0, // top
	12.0, 13.0, 14.0, 12.0, 14.0, 15.0, // bottom
	16.0, 17.0, 18.0, 16.0, 18.0, 19.0, // front
	20.0, 21.0, 22.0, 20.0, 22.0, 23.0, // back
];