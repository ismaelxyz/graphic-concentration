use sdl2::{
    image::LoadSurface, pixels::Color, render::Texture, render::TextureCreator as TCreator,
    surface::Surface, video::WindowContext as WContext,
};

/// Loads image at specified path
pub fn load_texture(path: &str, creator: &TCreator<WContext>, color: (u8, u8, u8)) -> Texture {
    let mut surf = Surface::from_file(path).expect("Could not load surface from file!");

    // Color key image
    surf.set_color_key(true, Color::from(color))
        .expect("Can't set color key");

    // Create texture from surface pixels
    creator.create_texture_from_surface(&surf).unwrap()

}
