use macroquad::prelude::*;

use crate::entities::ship::Ship;
use crate::entities::ship::{Drawable, Movable};

mod entities;

#[macroquad::main("SpaceGame")]
async fn main() {
    let background_bytes = include_bytes!("../assets/space.png");
    let ship_bytes = include_bytes!("../assets/ship.png");
    let background: Texture2D =
        Texture2D::from_file_with_format(background_bytes, Some(ImageFormat::Png));
    let ship_texture: Texture2D =
        Texture2D::from_file_with_format(ship_bytes, Some(ImageFormat::Png));
    let mut ship = Ship::new(ship_texture);

    loop {
        clear_background(WHITE);
        draw_texture_ex(
            &background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
        ship.step();
        ship.draw();
        next_frame().await
    }
}
