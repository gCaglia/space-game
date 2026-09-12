// Overaly while not playing

use macroquad::{
    color::Color,
    shapes::draw_rectangle,
    window::{screen_height, screen_width},
};

pub fn draw_overlay() {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.6),
    );
}
