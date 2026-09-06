use macroquad::{
    color::YELLOW,
    shapes::{DrawRectangleParams, draw_rectangle_ex},
    window::{screen_height, screen_width},
};

pub struct Laser {
    pos: [f32; 2],
    velocity: [f32; 2],
}

impl Laser {
    pub fn new(pos: [f32; 2], ship_velocity: [f32; 2]) -> Self {
        let speed: f32 = 0.01;
        let ship_vec_magnitude = (ship_velocity[0].powi(2) + ship_velocity[1].powi(2)).sqrt();
        let velocity = [
            ship_velocity[0] / ship_vec_magnitude * speed,
            ship_velocity[1] / ship_vec_magnitude * speed,
        ];

        Laser { pos, velocity }
    }

    pub fn step(&mut self) -> bool {
        self.update();
        self.draw();

        self.in_bounds()
    }

    fn update(&mut self) {
        self.pos[0] += self.velocity[0] * screen_width();
        self.pos[1] += self.velocity[1] * screen_height();
    }
    fn draw(&self) {
        let x = self.pos[0];
        let y = self.pos[1];

        let w = 0.005 * screen_width();
        let h = 0.02 * screen_height();

        
        let rotation: f32 = if self.velocity[0] == 0.0 && self.velocity[1] == 0.0 {
            0.0
        } else {
            self.velocity[0].atan2(-self.velocity[1])
        };

        let params = DrawRectangleParams {
            rotation,
            color: YELLOW,
            ..Default::default()
        };

        draw_rectangle_ex(x, y, w, h, params);
    }

    fn in_bounds(&self) -> bool {
        self.pos.iter().all(|v| -0.1 < *v || *v < 1.1)
    }
}
