use std::time::UNIX_EPOCH;

use macroquad::audio::{Sound, load_sound_from_bytes};
use macroquad::prelude::*;
use macroquad::rand::srand;

use crate::entities::ship::Ship;
use crate::entities::ship::{Drawable, Movable};
use crate::entities::spawner;
use crate::screens::credits::render_credits;
use crate::screens::menu::render_menu;
use crate::utils::overlay::draw_overlay;

const LASER_SOUND: &[u8] = include_bytes!("../assets/sounds/laserSmall_004.ogg");
const DEBUG: bool = false;
const GODMODE: bool = false;

mod entities;
mod screens;
mod utils;

#[derive(PartialEq)]
enum GameState {
    Menu,
    Playing,
    Credits,
    GameOver,
    Exit,
}

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
    let mut spawner = spawner::Spawner::new().load_sounds().await;
    let mut game_state = GameState::Menu;

    loop {
        // Background Texture
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

        // Opening Menu
        if game_state == GameState::Menu {
            draw_overlay();
            game_state = render_menu();
        } else if game_state == GameState::Playing || game_state == GameState::GameOver {
            // Playing
            if game_state != GameState::GameOver {
                ship.step();
            }
            ship.draw();

            let ship_body = ship.get_body();
            let mut_shots = ship.get_shots_mut();
            let mut game_over = game_state == GameState::GameOver;
            let life_lost = spawner.step(mut_shots, ship_body, &game_over);

            if life_lost {
                game_over = ship.take_damage();

                if game_over {
                    game_state = GameState::GameOver;
                }
            }

            if game_state == GameState::GameOver {
                draw_text(
                    "GAME\nOVER!",
                    0.5 * screen_width() - 100.0,
                    0.5 * screen_height(),
                    50.0,
                    ORANGE,
                );
            }
            if is_key_pressed(KeyCode::Escape) {
                game_state = GameState::Menu;
            }
        } else if game_state == GameState::Credits {
            draw_overlay();
            game_state = render_credits();
        } else if game_state == GameState::Exit {
            break;
        }

        // Next Frame
        next_frame().await
    }
}
