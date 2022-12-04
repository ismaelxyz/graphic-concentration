use crate::BlockBuild;
use std::path::PathBuf;

pub(crate) struct TextureManager {
    /// Common width for all textures
    width: u32,
    /// Common height for all textures
    height: u32,
    /// an array to keep track of the textures we've already added
    textures: Vec<String>,
    pub texture_array: u32,
    max_textures: usize,
}

use image::{open, DynamicImage};

impl TextureManager {
    pub(crate) fn new(width: u32, height: u32, max_textures: usize) -> Self {
        TextureManager {
            width,
            height,
            textures: Vec::new(),
            max_textures,
            texture_array: unsafe {
                // create our texture array
                let mut texture_array = 0;
                gl::GenTextures(1, &mut texture_array);
                gl::BindTexture(gl::TEXTURE_2D_ARRAY, texture_array);

                // disable texture filtering for magnification (return the texel that's nearest to the fragment's texture coordinate)
                gl::TexParameteri(
                    gl::TEXTURE_2D_ARRAY,
                    gl::TEXTURE_MAG_FILTER,
                    gl::NEAREST as i32,
                );

                // set the dimensions of our texture array
                gl::TexImage3D(
                    gl::TEXTURE_2D_ARRAY,
                    0,
                    gl::RGBA as i32,
                    width as i32,
                    height as i32,
                    max_textures as i32,
                    0,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    std::ptr::null(),
                );

                texture_array
            },
        }
    }

    pub fn build_block<'t, N: Into<Option<&'t str>>>(&'t mut self, name: N) -> BlockBuild<'t> {
        BlockBuild {
            name: &name.into().unwrap_or("unknown"),
            tex_coords: crate::numbers::TEX_COORDS.clone(),
            texture_manager: self,
        }
    }

    pub(crate) fn index(&self, name: &str) -> usize {
        self.textures.iter().position(|n| n == name).unwrap()
    }

    pub(crate) fn generate_mipmaps(&self) {
        unsafe {
            // make sure our texture is bound
            gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.texture_array);
            // generate mipmaps for our texture
            gl::GenerateMipmap(gl::TEXTURE_2D_ARRAY);
        }
    }

    /// Add a new Texture and return it index
    pub(crate) fn push(&mut self, texture: &str) -> usize {
        match self.textures.iter().position(|t| t == texture) {
            Some(pos) => pos,
            None => unsafe {
                // add it to our textures list if not
                self.textures.push(texture.into());

                let mut path = PathBuf::from("episode5");
                path.push("textures");
                path.push(texture);
                path.set_extension("png");
                
                // load and get the image data of the texture we want
                let texture_image = open(path).unwrap().into_rgba8();
                let data = texture_image.into_vec();
                // $ let texture_image = pyglet.image.load(f"textures/{texture}.png").get_image_data()
                // make sure our texture array is bound
                gl::BindTexture(gl::TEXTURE_2D_ARRAY, self.texture_array);

                // paste our texture's image data in the appropriate spot in our texture array
                gl::TexSubImage3D(
                    gl::TEXTURE_2D_ARRAY,
                    0,
                    0,
                    0,
                    self.index(texture) as i32,
                    self.width as i32,
                    self.height as i32,
                    1,
                    gl::RGBA,
                    gl::UNSIGNED_BYTE,
                    data.as_ptr() as *const _,
                    // $ texture_image.get_data("RGBA", texture_image.width * 4)
                    //std::ptr::null(),
                );

                self.textures.len() - 1
            },
        }
    }
}
