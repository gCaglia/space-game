use std::time::SystemTime;

use crate::entities::asteroids::Asteroid;

pub struct Spawner {
    asteroids: Vec<Asteroid>,
    start_time: SystemTime,
}

impl Spawner {
    pub fn new() -> Self {
        let asteroids: Vec<Asteroid> = Vec::new();
        let start_time: SystemTime = SystemTime::now();
        Spawner {
            asteroids,
            start_time,
        }
    }

    pub fn step(&mut self) {
        // Update the position of all asteroids and
        // despawn all OOB
        self.update_asteroids();

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

    fn update_asteroids(&mut self) {
        self.asteroids.retain_mut(|a| a.step());
    }
}
