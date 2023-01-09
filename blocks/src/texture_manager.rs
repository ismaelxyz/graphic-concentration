use glow::{HasContext, PixelUnpackData};
use image::open;
use std::{path::PathBuf, sync::Arc};

#[rustfmt::skip]
const VERTEX_POSITIONS: [f32; 72] = [
     0.5,  0.5,  0.5,  0.5, -0.5,  0.5,  0.5, -0.5, -0.5,  0.5,  0.5, -0.5,
    -0.5,  0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5,  0.5, -0.5,  0.5,  0.5,
    -0.5,  0.5,  0.5, -0.5,  0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,  0.5,
    -0.5, -0.5,  0.5, -0.5, -0.5, -0.5,  0.5, -0.5, -0.5,  0.5, -0.5,  0.5,
    -0.5,  0.5,  0.5, -0.5, -0.5,  0.5,  0.5, -0.5,  0.5,  0.5,  0.5,  0.5,
     0.5,  0.5, -0.5,  0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5,  0.5, -0.5,
];

#[rustfmt::skip]
const TEX_COORDS: [f32; 72] = [
    0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
    0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
    0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
    0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
    0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
    0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0, 0.0,
];

#[allow(dead_code)]
#[rustfmt::skip]
const SHADING: [f32; 24] = [
    0.80, 0.80, 0.80, 0.80,
    0.80, 0.80, 0.80, 0.80,
    1.00, 1.00, 1.00, 1.00,
    0.49, 0.49, 0.49, 0.49,
    0.92, 0.92, 0.92, 0.92,
    0.92, 0.92, 0.92, 0.92,

];

#[rustfmt::skip]
const INDICES: [u32; 36] = [
     0,  1,  2,  0,  2,  3, // right
     4,  5,  6,  4,  6,  7, // left
     8,  9, 10,  8, 10, 11, // top
    12, 13, 14, 12, 14, 15, // bottom
    16, 17, 18, 16, 18, 19, // front
    20, 21, 22, 20, 22, 23, // back
];

pub struct TextureManager {
    gl: Arc<glow::Context>,
    /// Common width for all textures
    width: u32,
    /// Common height for all textures
    height: u32,
    /// an array to keep track of the textures we've already added
    textures: Vec<String>,
    texture_array: Option<glow::NativeTexture>,
    max_textures: usize,
}

impl TextureManager {
    pub fn new(gl: &Arc<glow::Context>, width: u32, height: u32, max_textures: usize) -> Self {
        TextureManager {
            gl: gl.clone(),
            width,
            height,
            textures: Vec::new(),
            max_textures,
            texture_array: unsafe {
                // create our texture array
                let texture_array = Some(gl.create_texture().unwrap());
                gl.bind_texture(glow::TEXTURE_2D_ARRAY, texture_array);

                // disable texture filtering for magnification (return the texel
                // that's nearest to the fragment's texture coordinate)
                gl.tex_parameter_i32(
                    glow::TEXTURE_2D_ARRAY,
                    glow::TEXTURE_MAG_FILTER,
                    glow::NEAREST as i32,
                );

                // set the dimensions of our texture array
                gl.tex_image_3d(
                    glow::TEXTURE_2D_ARRAY,
                    0,
                    glow::RGBA as i32,
                    width as i32,
                    height as i32,
                    max_textures as i32,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    None,
                );

                texture_array
            },
        }
    }

    pub fn build_block<'a>(&'a mut self, name: &str) -> BlockBuild {
        BlockBuild {
            name: name.into(),
            tex_coords: TEX_COORDS,
            texture_manager: self,
        }
    }

    pub fn finish(&self) {
        unsafe {
            // make sure our texture is bound
            self.gl
                .bind_texture(glow::TEXTURE_2D_ARRAY, self.texture_array);
            // generate mipmaps for our texture
            self.gl.generate_mipmap(glow::TEXTURE_2D_ARRAY);
        }
    }

    pub fn texture_array(&self) -> Option<glow::NativeTexture> {
        self.texture_array
    }

    pub fn max_textures(&self) -> usize {
        self.max_textures
    }

    /// Add a new Texture and return it index
    fn get_or_insert(&mut self, texture: &str) -> usize {
        match self.textures.iter().position(|t| t == texture) {
            Some(pos) => pos,
            None => unsafe {
                // add it to our textures list if not
                self.textures.push(texture.into());

                let mut path = PathBuf::from("textures");
                path.push(texture);
                path.set_extension("png");

                // load and get the image data of the texture we want
                // TODO: Pendding. pitch is updated and not used `.flipv()`
                let texture_image = open(path).unwrap().flipv().into_rgba8();
                let data = texture_image.into_vec();
                
                // make sure our texture array is bound
                self.gl
                    .bind_texture(glow::TEXTURE_2D_ARRAY, self.texture_array);

                // paste our texture's image data in the appropriate spot in our texture array
                self.gl.tex_sub_image_3d(
                    glow::TEXTURE_2D_ARRAY,
                    0,
                    0,
                    0,
                    self.textures.len() as i32 - 1,
                    self.width as i32,
                    self.height as i32,
                    1,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    PixelUnpackData::Slice(&data),
                );

                self.textures.len() - 1
            },
        }
    }
}

pub struct BlockBuild<'a> {
    name: String,
    texture_manager: &'a mut TextureManager,
    tex_coords: [f32; 72],
}

impl<'a> BlockBuild<'a> {
    /// set a specific face of the block to a certain texture
    fn set_block_face(&mut self, face: usize, texture: &str) {
        // find that texture's index (texture's Z component in our texture
        // array) so that we can modify the texture coordinates of each face appropriately
        let texture = self.texture_manager.get_or_insert(texture);

        for vertex in 0..4 {
            self.tex_coords[face * 12 + vertex * 3 + 2] = texture as f32;
        }
    }

    /// set the texture for all faces if "all" is specified
    pub fn all(mut self, texture: &str) -> Self {
        (0..6).for_each(|face| self.set_block_face(face, texture));

        self
    }

    /// set the texture for only the sides if "sides" is specified
    pub fn sides(mut self, texture: &str) -> Self {
        self.set_block_face(0, texture);
        self.set_block_face(1, texture);
        self.set_block_face(4, texture);
        self.set_block_face(5, texture);

        self
    }

    /// set the texture for only right side
    pub fn right(mut self, texture: &str) -> Self {
        self.set_block_face(0, texture);

        self
    }

    /// set the texture for only left side
    pub fn left(mut self, texture: &str) -> Self {
        self.set_block_face(1, texture);

        self
    }

    /// set the texture for only top side
    pub fn top(mut self, texture: &str) -> Self {
        self.set_block_face(2, texture);

        self
    }

    /// set the texture for only bottom side
    pub fn bottom(mut self, texture: &str) -> Self {
        self.set_block_face(3, texture);

        self
    }

    /// set the texture for only front side
    pub fn front(mut self, texture: &str) -> Self {
        self.set_block_face(4, texture);

        self
    }

    /// set the texture for only back side
    pub fn back(mut self, texture: &str) -> Self {
        self.set_block_face(5, texture);

        self
    }

    pub fn build(self) -> Block {
        let BlockBuild {
            name, tex_coords, ..
        } = self;
        Block {
            name,
            tex_coords,
            vertex_positions: VERTEX_POSITIONS,
            indices: INDICES,
        }
    }
}

pub struct Block {
    name: String,
    vertex_positions: [f32; 72],
    tex_coords: [f32; 72],
    indices: [u32; 36],
}

impl Block {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn vertex_positions(&self) -> &[f32; 72] {
        &self.vertex_positions
    }

    pub fn tex_coords(&self) -> &[f32; 72] {
        &self.tex_coords
    }

    pub fn indices(&self) -> &[u32; 36] {
        &self.indices
    }
}
