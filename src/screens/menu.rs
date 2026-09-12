// Menu Screen

use macroquad::{
    color::{DARKGRAY, GRAY, WHITE},
    input::{is_mouse_button_pressed, mouse_position},
    math::{Rect, Vec2},
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{draw_text, measure_text},
    window::{screen_height, screen_width},
};

use crate::GameState;

struct Button {
    rect: Rect,
    label: String,
}

impl Button {
    fn draw(&self, hovered: bool) {
        self.draw_box(hovered);
        self.draw_text();
    }

    fn draw_box(&self, hovered: bool) {
        let x = self.rect.x;
        let y = self.rect.y;
        let w = self.rect.w;
        let h = self.rect.h;
        let color = if hovered { DARKGRAY } else { GRAY };

        let thickness = 2.0;

        draw_rectangle(x, y, w, h, color);
        draw_rectangle_lines(x, y, w, h, thickness, color);
    }

    fn draw_text(&self) {
        let text = &self.label;
        let font = None;
        let font_size = 30;
        let font_scale = 1.0;
        let text_size = measure_text(text, font, font_size, font_scale);

        let x = self.rect.x + (self.rect.w - text_size.width) / 2.0;
        let y = self.rect.y + (self.rect.h - text_size.height);
        let color = WHITE;

        draw_text(text, x, y, font_size as f32, color);
    }

    fn is_hovered(&self, mouse: Vec2) -> bool {
        self.rect.contains(mouse)
    }

    fn is_clicked(&self, mouse: Vec2) -> bool {
        self.is_hovered(mouse) && is_mouse_button_pressed(macroquad::input::MouseButton::Left)
    }
}

pub fn render_menu() -> GameState {
    let x = 0.4 * screen_width();
    let w = 0.2 * screen_width();
    let h = 0.05 * screen_width();

    let play_button = Button {
        rect: Rect {
            x,
            y: 0.30 * screen_height(),
            w,
            h,
        },
        label: "Play".to_string(),
    };
    let credits_button = Button {
        rect: Rect {
            x,
            y: 0.45 * screen_height(),
            w,
            h,
        },
        label: "Credits".to_string(),
    };
    let exit_button = Button {
        rect: Rect {
            x,
            y: 0.6 * screen_height(),
            w,
            h,
        },
        label: "Exit".to_string(),
    };

    let mouse = Vec2::from(mouse_position());
    play_button.draw(play_button.is_hovered(mouse));
    credits_button.draw(credits_button.is_hovered(mouse));
    exit_button.draw(exit_button.is_hovered(mouse));
    if play_button.is_clicked(mouse) {
        GameState::Playing
    } else if credits_button.is_clicked(mouse) {
        GameState::Credits
    } else if exit_button.is_clicked(mouse) {
        GameState::Exit
    } else {
        GameState::Menu
    }
}
