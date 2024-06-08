use std::{f32::consts::PI, time::SystemTime};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::ErrorAlias;

#[derive(Serialize, Deserialize, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
const ARENA_DIMENSIONS: Vec2 = Vec2 {
    x: 800.0,
    y: 600.0,
};

const PADDLE_DIMENSIONS: Vec2 = Vec2 {
    x: 40.0,
    y: 175.0,
};

const PADDLE_PADDING: f32 = 40.0;

const BALL_DIMENSIONS: Vec2 = Vec2 {
    x: 40.0,
    y: 40.0
};

const INIT_BALL_POS: Vec2 = Vec2 {
   x: ARENA_DIMENSIONS.x / 2.0 - BALL_DIMENSIONS.x / 2.0,
   y: ARENA_DIMENSIONS.y / 2.0 - BALL_DIMENSIONS.y / 2.0

};

struct Game {
    game_state: GameState,
    last_tick: SystemTime,
    paused: bool,
}
impl Game {
    pub fn new() -> Self {
        let p1_pos = Vec2 {
            x: 0.0 + PADDLE_PADDING,
            y: ARENA_DIMENSIONS.y + (ARENA_DIMENSIONS.y / 2.0) - (PADDLE_DIMENSIONS.y / 2.0)
        };
        let p2_pos = Vec2 {
            x: ARENA_DIMENSIONS.x - PADDLE_PADDING - PADDLE_DIMENSIONS.x,
            y: ARENA_DIMENSIONS.y + (ARENA_DIMENSIONS.y / 2.0) - (PADDLE_DIMENSIONS.y / 2.0)
        };

        let ball_pos = INIT_BALL_POS; 
        let ball_velocity = Game::init_ball_velocity();

        let game_state = GameState {
            p1_pos,
            p2_pos,
            ball_pos,
            current_score: (0, 0),
            ball_velocity,
            current_magnitude: 1.0,
            timer: 0.0,
        };

        return Self {
            game_state,
            last_tick: SystemTime::now(),
            paused: true
        }
    }

    pub fn advance(&mut self) -> Result<(), ErrorAlias>{
        if self.paused {

        }
        let time_since_last_tick = self.last_tick.elapsed()?;
        let current_time = SystemTime::now();

        let mut new_ball_pos = Vec2 {
            x: self.game_state.ball_pos.x + (self.game_state.ball_velocity.x * time_since_last_tick.as_secs_f32()),
            y: self.game_state.ball_pos.y + (self.game_state.ball_velocity.y * time_since_last_tick.as_secs_f32()),
        };

        if Game::check_ball_scored(&new_ball_pos) {
            if new_ball_pos.x < 0.0 {
                self.game_state.current_score.1 += 1;
            }
            else {
                self.game_state.current_score.0 += 1;
            }

            new_ball_pos = INIT_BALL_POS;
            self.paused = true;
            self.game_state.ball_velocity = Game::init_ball_velocity();
            self.game_state.ball_pos = new_ball_pos;
        }

        Ok(())
    }
    fn check_ball_scored(ball_pos: &Vec2) -> bool {
        if ball_pos.x < 0.0 || ball_pos.x > ARENA_DIMENSIONS.x {
            return false
        }
        return true
    }
    fn init_ball_velocity() -> Vec2 {
        let random: f32 = {
            let mut rng = rand::thread_rng();
            rng.gen()
        };

        let mut velocity_angle: f32 = random * 2.0 * PI;
        if (velocity_angle > (PI / 4.0) && velocity_angle < (3.0 * PI / 4.0)) || (velocity_angle < 5.0 * PI / 4.0 && velocity_angle > 7.0 * PI / 4.0) {
            velocity_angle += PI / 4.0;
        }

        return Vec2 {
            x: velocity_angle.cos(),
            y: velocity_angle.sin()
        };
    }
}

pub struct GameState {
    pub p1_pos: Vec2,
    pub p2_pos: Vec2,
    pub ball_pos: Vec2,
    pub current_score: (u32, u32),
    ball_velocity: Vec2,
    current_magnitude: f32,
    timer: f32,
}
