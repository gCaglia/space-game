use macroquad::{
    color::WHITE,
    input::{KeyCode, is_key_down},
    texture::{Texture2D, draw_texture},
    window::{screen_height, screen_width},
};

const ACC_SPEED: f32 = 1e-5;
const GAME_BOUNDS: (f32, f32) = (0.0, 0.975);
const ACC_CEIL: f32 = 0.1;

pub struct Ship {
    texture: Texture2D,
    rel_pos: Vec<f32>,
    acc_vector: Vec<f32>,
}

pub trait Drawable {
    fn draw(&self);
}

pub trait Movable {
    fn update_acc(&mut self);
    fn update_pos(&mut self);
    fn step(&mut self);
}

impl Ship {
    pub fn new(texture: Texture2D) -> Self {
        Ship {
            texture,
            rel_pos: vec![0.5, 0.5],
            acc_vector: vec![0.0, 0.0],
        }
    }
}

impl Drawable for Ship {
    fn draw(&self) {
        let pos_x = self.rel_pos.get(0).unwrap() * screen_width();
        let pos_y = self.rel_pos.get(1).unwrap() * screen_height();
        draw_texture(&self.texture, pos_x, pos_y, WHITE);
    }
}

impl Movable for Ship {
    fn update_acc(&mut self) {
        if is_key_down(KeyCode::A) {
            self.acc_vector[0] = (self.acc_vector[0] - ACC_SPEED).clamp(-ACC_CEIL, ACC_CEIL);
        }

        if is_key_down(KeyCode::W) {
            self.acc_vector[1] = (self.acc_vector[1] - ACC_SPEED).clamp(-ACC_CEIL, ACC_CEIL);
        }

        if is_key_down(KeyCode::D) {
            self.acc_vector[0] = (self.acc_vector[0] + ACC_SPEED).clamp(-ACC_CEIL, ACC_CEIL);
        }
        if is_key_down(KeyCode::S) {
            self.acc_vector[1] = (self.acc_vector[1] + ACC_SPEED).clamp(-ACC_CEIL, ACC_CEIL);
        }
    }

    fn update_pos(&mut self) {
        let (lower, upper) = GAME_BOUNDS;
        self.rel_pos[0] = (self.rel_pos[0] + self.acc_vector[0]).clamp(lower, upper);
        self.rel_pos[1] = (self.rel_pos[1] + self.acc_vector[1]).clamp(lower, upper);

        if self.rel_pos[0] == lower || self.rel_pos[0] == upper {
            self.acc_vector[0] = 0.0
        }
        if self.rel_pos[1] == lower || self.rel_pos[1] == upper {
            self.acc_vector[1] = 0.0
        }
    }

    fn step(&mut self) {
        self.update_acc();
        self.update_pos();
    }
}
