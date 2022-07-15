use std::time::Duration;

use crate::{
    frame::{Drawable, Frame},
    invaders::Invaders,
    shot::Shot,
    NUM_COLS, NUM_ROWS,
};

pub const MAX_SHOTS: usize = 3;
pub const MAX_AMMO: u32 = 100;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
    pub count_shot: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: vec![],
            count_shot: 0,
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < MAX_SHOTS && self.count_shot < MAX_AMMO {
            self.shots.push(Shot::new(self.x, self.y - 1));
            self.count_shot += 1;
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta)
        }
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_smt = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding && invaders.kill_invader_at(shot.x, shot.y) {
                hit_smt = true;
                shot.explode();
            }
        }

        hit_smt
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.y][self.x] = "A";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}
