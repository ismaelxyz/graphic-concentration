use sdl2::{
    pixels::Color,
    rect::{Point, Rect},
    render::{Texture, WindowCanvas},
};

pub(crate) fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    world: &crate::World,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let (mut pos, mut sprite) = (&world.player.position, &world.player.frame);
    let mut current = None;

    loop {
        let current_frame = sprite.region;

        // Treat the center of the screen as the (0, 0) coordinate
        let screen_position = pos.0 + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(
            screen_position,
            current_frame.width(),
            current_frame.height(),
        );
        canvas.copy(&textures[sprite.spritesheet], current_frame, screen_rect)?;

        if let Some(i) = current.or(Some(0)) {
            if i == world.enemies.len() {
                break;
            }

            pos = &world.enemies[i].position;
            sprite = &world.enemies[i].frame;

            current = Some(i + 1);
        }
    }

    canvas.present();

    Ok(())
}
