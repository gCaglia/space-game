use std::f32::consts::PI;

use macroquad::{
    audio::{PlaySoundParams, Sound, play_sound},
    color::{WHITE, YELLOW},
    input::{
        KeyCode::{self},
        is_key_down, is_key_pressed,
    },
    math::Rect,
    shapes::draw_rectangle,
    texture::{DrawTextureParams, Texture2D, draw_texture_ex},
    window::{screen_height, screen_width},
};

use crate::{DEBUG, entities::laser::Laser};

const ACC_SPEED: f32 = 1e-5;
const GAME_BOUNDS: (f32, f32) = (0.0, 0.975);
const ACC_CEIL: f32 = 0.1;

pub struct Ship {
    texture: Texture2D,
    rel_pos: Vec<f32>,
    acc_vector: Vec<f32>,
    angle: f32,

    shots: Vec<Laser>,
    laser_sound: Sound,
}

pub trait Drawable {
    fn draw(&self);
}

pub trait Movable {
    fn update_acc(&mut self);
    fn update_pos(&mut self);
    fn update_angle(&mut self);
    fn update_fire(&mut self);
    fn get_body(&self) -> Rect;
    fn step(&mut self);
}

impl Ship {
    pub fn new(texture: Texture2D, laser_sound: Sound) -> Self {
        Ship {
            texture,
            rel_pos: vec![0.5, 0.5],
            acc_vector: vec![0.0, 0.0],
            angle: 0.0,
            shots: Vec::new(),
            laser_sound,
        }
    }

    pub fn get_shots_mut(&mut self) -> &mut Vec<Laser> {
        &mut self.shots
    }
}

impl Drawable for Ship {
    fn draw(&self) {
        let pos_x = self.rel_pos.first().unwrap() * screen_width();
        let pos_y = self.rel_pos.get(1).unwrap() * screen_height();
        draw_texture_ex(
            &self.texture,
            pos_x,
            pos_y,
            WHITE,
            DrawTextureParams {
                rotation: self.angle,
                ..Default::default()
            },
        );
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

    fn update_angle(&mut self) {
        if self.acc_vector[0] == 0.0 && self.acc_vector[1] == 0.0 {
            self.angle = 0.0
        } else {
            self.angle = self.acc_vector[0].atan2(-self.acc_vector[1])
        }
    }

    fn get_body(&self) -> Rect {
        let x = self.rel_pos[0] * screen_width();
        let y = self.rel_pos[1] * screen_height();
        let h = self.texture.height() * 0.95;
        let w = self.texture.width() * 0.95;

        // Check if we need to swap x and y for more precise colission detection
        // If the angle is between 0.25pi and 0.75pi or 1.25pi and 1.75pi,
        // the, x and y should be swapped so that the rectangle is lying on its
        // side instead of standing up.
        let swap = 0.25 * PI <= self.angle.abs() && self.angle.abs() <= 0.75 * PI;
        let body: Rect = if swap {
            Rect {
                x: x - self.texture.width() / 4.0,
                y: y + self.texture.height() / 4.0,
                w: h,
                h: w,
            }
        } else {
            Rect { x, y, w, h }
        };

        if DEBUG {
            println!("Angle: {}; Swap: {}", self.angle, swap);
            draw_rectangle(body.x, body.y, body.w, body.h, YELLOW);
        }

        body
    }

    fn update_fire(&mut self) {
        if is_key_pressed(KeyCode::Space) {
            let pos = self.get_body().center();
            let ship_velocity = [self.acc_vector[0], self.acc_vector[1]];
            self.shots.push(Laser::new(pos.into(), ship_velocity));
            let sound = &self.laser_sound;
            let params = PlaySoundParams {
                looped: false,
                volume: 1.0,
            };
            play_sound(sound, params);
        }
        self.shots.retain_mut(|s| s.step());
    }

    fn step(&mut self) {
        self.update_acc();
        self.update_angle();
        self.update_pos();
        self.update_fire();
    }
}
