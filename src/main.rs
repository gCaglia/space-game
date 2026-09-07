use std::time::UNIX_EPOCH;

use macroquad::audio::{Sound, load_sound_from_bytes};
use macroquad::prelude::*;
use macroquad::rand::srand;

use crate::entities::ship::Ship;
use crate::entities::ship::{Drawable, Movable};
use crate::entities::spawner;

const LASER_SOUND: &[u8] = include_bytes!("../assets/sounds/laserSmall_004.ogg");

mod entities;

#[macroquad::main("SpaceGame")]
async fn main() {
    srand(
        std::time::SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
    );

    let background_bytes = include_bytes!("../assets/graphics/space.png");
    let ship_bytes = include_bytes!("../assets/graphics/ship.png");
    let background: Texture2D =
        Texture2D::from_file_with_format(background_bytes, Some(ImageFormat::Png));
    let ship_texture: Texture2D =
        Texture2D::from_file_with_format(ship_bytes, Some(ImageFormat::Png));
    let laser_sound: Sound = load_sound_from_bytes(LASER_SOUND).await.unwrap();
    let mut ship = Ship::new(ship_texture, laser_sound);
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
        spawner.step(ship.get_shots_mut());
        next_frame().await
    }
}
