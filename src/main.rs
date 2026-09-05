use std::time::UNIX_EPOCH;

use macroquad::prelude::*;
use macroquad::rand::srand;

use crate::entities::ship::Ship;
use crate::entities::ship::{Drawable, Movable};
use crate::entities::spawner;

mod entities;

#[macroquad::main("SpaceGame")]
async fn main() {
    srand(
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    let background_bytes = include_bytes!("../assets/space.png");
    let ship_bytes = include_bytes!("../assets/ship.png");
    let background: Texture2D =
        Texture2D::from_file_with_format(background_bytes, Some(ImageFormat::Png));
    let ship_texture: Texture2D =
        Texture2D::from_file_with_format(ship_bytes, Some(ImageFormat::Png));
    let mut ship = Ship::new(ship_texture);
    let mut spawner = spawner::Spawner::new();

    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
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
        spawner.step();
        next_frame().await
    }
}
