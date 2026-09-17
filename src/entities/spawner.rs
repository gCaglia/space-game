use std::time::SystemTime;

use macroquad::{
    audio::{PlaySoundParams, Sound, load_sound_from_bytes, play_sound},
    color::YELLOW,
    math::Rect,
    rand::ChooseRandom,
    text::draw_text,
    window::{screen_height, screen_width},
};

use crate::{
    GODMODE,
    entities::{asteroids::Asteroid, laser::Laser},
    utils::constants::EXPLOSIONS,
};

pub struct Spawner {
    asteroids: Vec<Asteroid>,
    start_time: SystemTime,
    sounds: Vec<Sound>,
    hits: u64,
}

impl Spawner {
    pub fn new() -> Self {
        let asteroids: Vec<Asteroid> = Vec::new();
        let start_time: SystemTime = SystemTime::now();
        let hits = 0;
        Spawner {
            asteroids,
            start_time,
            sounds: Vec::new(),
            hits,
        }
    }

    pub async fn load_sounds(mut self) -> Spawner {
        let futures = EXPLOSIONS.iter().map(|data| load_sound_from_bytes(data));
        let sounds: Vec<Sound> = futures::future::join_all(futures)
            .await
            .into_iter()
            .map(|r| r.unwrap())
            .collect();
        self.sounds = sounds;
        self
    }

    pub fn step(&mut self, shots: &mut Vec<Laser>, ship_body: Rect, game_over: &bool) -> bool {
        // Update the position of all asteroids and
        // despawn all OOB
        if !game_over {
            self.update_asteroids();
        }

        // Despawn by hit
        self.check_laser_collission(shots);

        // Get current number of asteroids and derive goal number
        let current: u16 = self.asteroids.len().try_into().unwrap();
        let goal: u16 = self.get_goal();
        let diff = goal - current;

        for _ in 0..diff {
            let asteroid = Asteroid::new();
            self.asteroids.push(asteroid);
        }

        // Draw all
        self.asteroids.iter().for_each(|a| a.draw());
        self.draw_num_hits();

        // Check collision with ship
        if GODMODE {
            false
        } else {
            self.asteroids
                .iter()
                .any(|a| a.get_body().intersect(ship_body).is_some())
        }
    }

    fn get_goal(&self) -> u16 {
        let time_since_start = SystemTime::now()
            .duration_since(self.start_time)
            .unwrap()
            .as_secs();

        if time_since_start < 60 {
            5
        } else if time_since_start <= 120 {
            10
        } else if time_since_start <= 180 {
            20
        } else {
            50
        }
    }

    fn draw_num_hits(&self) {
        let x = 0.9 * screen_width();
        let mut y = 0.1 * screen_height();
        let font_size = 20.0;
        let color = YELLOW;
        draw_text("Score:", x, y, font_size, color);
        y = 0.15 * screen_height();
        draw_text(self.hits.to_string(), x, y, font_size, color);
    }

    fn check_laser_collission(&mut self, shots: &mut Vec<Laser>) {
        self.asteroids.retain_mut(|a| {
            let shots_orig = shots.len();
            shots.retain_mut(|s| !a.get_body().intersect(s.get_body()).is_some());
            let shots_after = shots.len();
            if shots_orig == shots_after {
                true
            } else {
                Self::play_explosion_sound(&self.sounds);
                self.hits += 1;
                false
            }
        });
    }

    fn play_explosion_sound(sounds: &[Sound]) {
        let sound = sounds.choose().unwrap();
        let params = PlaySoundParams {
            looped: false,
            volume: 1.0,
        };
        play_sound(sound, params);
    }

    fn update_asteroids(&mut self) {
        self.asteroids.retain_mut(|a| a.step());
    }
}
