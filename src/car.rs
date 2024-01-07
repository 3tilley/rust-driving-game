use crate::coordinates::Vec2d;
use crate::input::{Accelerator, Direction, KeyInput};
use crate::track::Track;

pub enum CarState {
    Finished,
    Racing,
    Crashed,
}

impl CarState {
    pub fn to_string(&self) -> String {
        match self {
            CarState::Finished => "Finished".to_string(),
            CarState::Racing => "Racing".to_string(),
            CarState::Crashed => "Crashed".to_string(),
        }
    }
}

pub struct PhysicsConstants {
    pub forward_acceleration_mss: f32,
    pub braking_acceleration_mss: f32,
    pub reverse_acceleration_mss: f32,
    pub max_forward_speed_ms: f32,
    pub max_reverse_speed_ms: f32,
    pub turn_rate_rs: f32,
}

impl Default for PhysicsConstants {
    fn default() -> Self {
        PhysicsConstants {
            // mph * 4/9 gives ms-1
            forward_acceleration_mss: 20.0,
            braking_acceleration_mss: 4.0,
            reverse_acceleration_mss: 0.5,
            max_forward_speed_ms: 50.0,
            max_reverse_speed_ms: 10.0,
            // This is roughly 90 degrees in 1 s
            turn_rate_rs: 1.6,
        }
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct Car {
    // x goes from left to right: +x points right
    // y goes from bottom to top: +y points up
    pub pos: Vec2d,
    // This is relative to the y axis. 0 points up, 90 points right
    pub direction_radians: f32,
    pub velocity: f32,

}

impl Car {
    pub fn x_velocity(&self) -> f32 {
        self.velocity * self.direction_radians.sin()
    }

    pub fn y_velocity(&self) -> f32 {
        self.velocity * self.direction_radians.cos()
    }

    pub fn update_position(&mut self, consts: &PhysicsConstants, delta_time_s: f32, key_input: Option<KeyInput>) -> (f32, f32, f32) {
        // position += velocity * delta + acceleration * delta * delta * 0.5
        let accel = key_input.map_or(0.0, |k| {
            match k.acceleration {
                None => 0.0,
                Some(Accelerator::Accelerate) => consts.forward_acceleration_mss,
                Some(Accelerator::Brake) => {
                    if self.velocity > 0.0 {
                        -consts.forward_acceleration_mss
                    } else {
                        -consts.reverse_acceleration_mss
                    }

                }
            }
        });
        self.velocity += accel * delta_time_s;
        let pos_change = self.velocity * delta_time_s + 0.5 * delta_time_s * accel * accel;
        let direction_change = key_input.map_or(0.0, |k| {
            match k.direction {
                None => 0.0,
                Some(Direction::Left) => -consts.turn_rate_rs,
                Some(Direction::Right) => consts.turn_rate_rs,
            }
        });
        let theta_change = delta_time_s * direction_change;
        self.direction_radians += theta_change;
        let x_change = pos_change * self.direction_radians.sin();
        let y_change =  pos_change * self.direction_radians.cos();
        self.pos.x += x_change;
        self.pos.y += y_change;
        (x_change, y_change, -theta_change)
    }

    pub fn get_state(&self, track: &Track) -> CarState {
        if track.is_within_track(&self.pos) {
            if track.is_finished(&self.pos) {
                CarState::Finished
            } else {
                CarState::Racing
            }
        } else {
            CarState::Crashed
        }
    }
}
