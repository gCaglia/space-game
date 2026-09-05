use macroquad::{color::WHITE, prelude::ImageFormat, rand::ChooseRandom, texture::{Texture2D, draw_texture}, window::{screen_height, screen_width}};


const AST_1: &[u8] = include_bytes!("../../assets/asteroid1.png");
const AST_2: &[u8] = include_bytes!("../../assets/asteroid2.png");
const AST_3: &[u8] = include_bytes!("../../assets/asteroid3.png");
const AST: [&[u8]; 3] = [AST_1, AST_2, AST_3];

pub struct Asteroid {
    rel_pos: Vec<f32>,
    velocity: Vec<f32>,
    texture: Texture2D,
    rotation_speed: f32,
}

impl Asteroid {
    pub fn new() -> Self {
        let bytes: &[u8] = AST.choose().unwrap();
        let texture: Texture2D = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
        Asteroid { rel_pos: vec![0.5, 0.0], velocity: vec![0.0, 0.0025], texture: texture }
    }

    pub fn step(&mut self) {
        self.rel_pos[0] += self.velocity[0];
        self.rel_pos[1] += self.velocity[1];
    }

    pub fn draw(&self) {
        let x = self.rel_pos[0] * screen_width();
        let y = self.rel_pos[1] * screen_height();
        let color = WHITE;
        draw_texture(
            &self.texture, x, y, color);
    }
}