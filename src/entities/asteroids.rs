use macroquad::{
    color::WHITE,
    prelude::ImageFormat,
    rand::{ChooseRandom, RandomRange},
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
    window::{screen_height, screen_width},
};

const AST_1: &[u8] = include_bytes!("../../assets/asteroid1.png");
const AST_2: &[u8] = include_bytes!("../../assets/asteroid2.png");
const AST_3: &[u8] = include_bytes!("../../assets/asteroid3.png");
const AST: [&[u8]; 3] = [AST_1, AST_2, AST_3];

const BOX_BOUND: f32 = 1.3;

pub struct Asteroid {
    rel_pos: [f32; 2],
    velocity: [f32; 2],
    texture: Texture2D,
    rotation: f32,
    rotation_speed: f32,
}

impl Asteroid {
    pub fn new() -> Self {
        let rel_pos: [f32; 2] = Self::chose_origin();
        let velocity: [f32; 2] = Self::choose_trajectory(rel_pos);

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

    pub fn step(&mut self) -> bool {
        // Advance the position and return if still in bounds
        self.rel_pos[0] += self.velocity[0];
        self.rel_pos[1] += self.velocity[1];
        self.update_rotation();

        self.in_bounds()
    }

    fn in_bounds(&self) -> bool {
        self.rel_pos.iter().all(|i| i.abs() <= BOX_BOUND)
    }

    fn chose_rotation_speed() -> f32 {
        let low = 0.0f32;
        let high = 0.01f32;
        RandomRange::gen_range(low, high)
    }

    fn chose_origin() -> [f32; 2] {
        let sides = [0u8, 1, 2, 3];
        let range: f32 = RandomRange::gen_range(-0.1, 1.1); // Including margin

        let side = *sides.choose().unwrap();

        if side == 0u8 {
            return [range, -0.1];
        } else if side == 1u8 {
            return [1.1, range];
        } else if side == 2u8 {
            return [range, 1.1];
        } else if side == 3u8 {
            return [-0.1, range];
        }
        [-0.1, -0.1]
    }

    fn choose_trajectory(origin: [f32; 2]) -> [f32; 2] {
        // Choose a point on a cross in the middle of the game box
        // and draw a trajectory through that point from origin with randomized
        // magnitude
        let sides = [0, 1u8];
        let range = RandomRange::gen_range(0.0, 1.0);
        let speed: f32 = RandomRange::gen_range(0.001, 0.005);

        let side = *sides.choose().unwrap();
        let direction: [f32; 2];

        if side == 0u8 {
            // vertical axis
            direction = [0.5 - origin[0], range - origin[1]];
        } else {
            // horizontal axis
            direction = [range - origin[0], 0.5 - origin[1]];
        }

        let magnitude: f32 = (direction[0].powi(2) + direction[1].powi(2)).sqrt();
        [
            direction[0] / magnitude * speed,
            direction[1] / magnitude * speed,
        ]
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
