mod matrix;
mod shader;
mod texture_manager;

pub use matrix::Matrix;
pub use shader::Shader;
pub use texture_manager::{Block, TextureManager};

/// 3d coordinates for each vertex
#[rustfmt::skip]
pub const VERTEX_POSITIONS: [f64; 12] = [
    // X    Y    Z
    -0.5,  0.5, 1.0,
    -0.5, -0.5, 1.0,
     0.5, -0.5, 1.0, 
     0.5,  0.5, 1.0,
];

/// Indices for the triangles
#[rustfmt::skip]
pub const INDICES: [u32; 6] = [
    // first triangle
    0, 1, 2,
    // second triangle
    0, 2, 3,
];
