use crate::{numbers, texture_manager::TextureManager};

macro_rules! face_method {
    ($name:ident, $face:expr) => {
        // Set the texture for only one of the sides
        pub fn $name(mut self, texture: &str) -> Self {
            let texture_index = self.texture_manager.push(texture);
            self.set_block_face($face, texture_index);
            self
        }
    };
}

pub struct BlockBuild<'t> {
    pub name: &'t str,
    pub(crate) tex_coords: [f32; 72], // VERTEX_POSITIONS
    pub(crate) texture_manager: &'t mut TextureManager,
}

impl<'t> BlockBuild<'t> {
    /// Set a specific face of the block to a certain texture
    pub fn set_block_face(&mut self, face: usize, texture_index: usize) {
        for vertex in 0..4 {
            self.tex_coords[face * 12 + vertex * 3 + 2] = texture_index as f32;
        }
    }

    /// Set the texture for all faces
    pub fn all(mut self, texture: &str) -> Self {
        // find that texture's index (texture's Z component in our texture
        // array) so that we can modify the texture coordinates of each face
        // appropriately
        let texture_index = self.texture_manager.push(texture);
        for face in 0..=5 {
            self.set_block_face(face, texture_index);
        }

        self
    }

    /// Set the texture for only the sides
    pub fn sides(mut self, texture: &str) -> Self {
        let texture_index = self.texture_manager.push(texture);
        self.set_block_face(0, texture_index);
        self.set_block_face(1, texture_index);
        self.set_block_face(4, texture_index);
        self.set_block_face(5, texture_index);

        self
    }

    face_method!(right, 0);
    face_method!(left, 1);
    face_method!(top, 2);
    face_method!(bottom, 3);
    face_method!(front, 4);
    face_method!(back, 5);

    pub fn sides_build(self, texture: &str) -> Block {
        self.sides(texture).build()
    }

    pub fn all_build(self) -> Block {
        let name = self.name;
        self.all(name).build()
    }

    pub fn build(self) -> Block {
        let BlockBuild {
            name, tex_coords, ..
        } = self;
        let name = name.to_string();
        Block {
            name,
            tex_coords,
            vertex_positions: numbers::VERTEX_POSITIONS,
            indices: numbers::INDICES,
        }
    }
}

pub struct Block {
    name: String,
    vertex_positions: [f32; 72],
    pub(crate) tex_coords: [f32; 72],
    indices: [f32; 36],
}

impl Block {}
