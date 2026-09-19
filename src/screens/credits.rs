use macroquad::{
    color::YELLOW,
    input::{KeyCode, is_key_pressed},
    text::{draw_text, measure_text},
    window::{screen_height, screen_width},
};

use crate::GameState;

fn render_line(text: String, y: f32) {
    let font = None;
    let font_size = 15;
    let font_scale = 1.0;
    let text_dim = measure_text(&text, font, font_size, font_scale);

    let x = 0.5 * screen_width() - text_dim.width / 2.0;
    let color = YELLOW;

    draw_text(&text, x, y * screen_height(), font_size as f32, color);
}

pub fn render_credits() -> GameState {
    render_line(
        "Background image asset provided by ESA with credit:".to_string(),
        0.3,
    );
    render_line(
        "ESA/Hubble & NASA, A. Riess and the SH0ES team; Acknowledgment: Mahdi Zamani".to_string(),
        0.35,
    );

    render_line("Space ship asset provided by:".to_string(), 0.45);
    render_line("Monogames on Pixabay".to_string(), 0.50);

    render_line("Other assets and game by:".to_string(), 0.6);
    render_line("Giulia Caglia".to_string(), 0.65);
    if is_key_pressed(KeyCode::Escape) {
        GameState::Menu
    } else {
        GameState::Credits
    }
}
