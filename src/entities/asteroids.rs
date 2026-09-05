use macroquad::{
    color::WHITE,
    experimental::scene::get_node,
    prelude::ImageFormat,
    rand::{ChooseRandom, RandGenerator, RandomRange},
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
    window::{screen_height, screen_width},
};

const AST_1: &[u8] = include_bytes!("../../assets/asteroid1.png");
const AST_2: &[u8] = include_bytes!("../../assets/asteroid2.png");
const AST_3: &[u8] = include_bytes!("../../assets/asteroid3.png");
const AST: [&[u8]; 3] = [AST_1, AST_2, AST_3];

pub struct Asteroid {
    rel_pos: [f32; 2],
    velocity: [f32; 2],
    texture: Texture2D,
    rotation: f32,
    rotation_speed: f32,
}

impl Asteroid {
    pub fn new() -> Self {
        let rel_pos: [f32; 2] = [0.5, 0.0];
        let velocity: [f32; 2] = [0.0, 0.0025];

        let bytes: &[u8] = AST.choose().unwrap();
        let texture: Texture2D = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));

        let rotation = 0.0f32;
        let rotation_speed = Self::chose_rotation_speed();
        Asteroid {
            rel_pos,
            velocity,
            texture,
            rotation,
            rotation_speed,
        }
    }

    pub fn step(&mut self) {
        self.rel_pos[0] += self.velocity[0];
        self.rel_pos[1] += self.velocity[1];
        self.update_rotation();
    }

    fn chose_rotation_speed() -> f32 {
        let low = 0.0f32;
        let high = 0.01f32;
        RandomRange::gen_range(low, high)
    }

    fn update_rotation(&mut self) {
        self.rotation += self.rotation_speed;
    }

    pub fn draw(&self) {
        let texture = &self.texture;
        let x = self.rel_pos[0] * screen_width();
        let y = self.rel_pos[1] * screen_height();
        let color = WHITE;
        let params = DrawTextureParams {
            rotation: self.rotation,
            ..Default::default()
        };
        draw_texture_ex(texture, x, y, color, params);
    }
}
