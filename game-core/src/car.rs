use crate::car_progress::CarProgress;
use crate::coordinates::Vec2d;
use crate::input::{Accelerator, Direction, KeyInput};
use crate::track::Track;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default)]
pub enum CarState {
    #[default]
    StartLine,
    Finished,
    Racing,
    Crashed,
    TimedOut,
}

pub enum TerminationCondition {
    Ticks(u64),
    Seconds(f32),
}

impl TerminationCondition {
    pub fn is_timed_out(&self, ticks: u64, time: f32) -> bool {
        match self {
            TerminationCondition::Ticks(max_ticks) => max_ticks < &ticks,
            TerminationCondition::Seconds(max_s) => max_s < &time,
        }
    }
}

impl CarState {
    pub fn to_string(&self) -> String {
        match self {
            CarState::StartLine => "StartLine".to_string(),
            CarState::Finished => "Finished".to_string(),
            CarState::Racing => "Racing".to_string(),
            CarState::Crashed => "Crashed".to_string(),
            CarState::TimedOut => "TimedOut".to_string(),
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

#[derive(Clone, Default, Debug)]
pub struct Car {
    // x goes from left to right: +x points right
    // y goes from bottom to top: +y points up
    pub pos: Vec2d,
    pub previous_pos: Vec2d,
    // This is relative to the y axis. 0 points up, 90 points right
    pub direction_radians: f32,
    pub velocity: f32,
    pub state: CarState,
    pub label: String,
}

impl Car {
    pub fn new(pos: Vec2d, label: &str) -> Car {
        Car {
            pos,
            previous_pos: pos,
            direction_radians: 0.0,
            velocity: 0.0,
            state: CarState::StartLine,
            label: label.to_string(),
        }
    }

    pub fn reset(&mut self, start_line: Vec2d) {
        self.pos = start_line;
        self.previous_pos = start_line;
        self.velocity = 0.0;
        self.direction_radians = 0.0;
        self.state = CarState::StartLine;
    }
    pub fn x_velocity(&self) -> f32 {
        self.velocity * self.direction_radians.sin()
    }

    pub fn y_velocity(&self) -> f32 {
        self.velocity * self.direction_radians.cos()
    }

    pub fn update_position(
        &mut self,
        consts: &PhysicsConstants,
        delta_time_s: f32,
        key_input: Option<KeyInput>,
    ) -> Option<(f32, f32, f32)> {
        // position += velocity * delta + acceleration * delta * delta * 0.5
        if key_input.is_some() && self.state == CarState::StartLine {
            self.state = CarState::Racing;
        }

        if self.state == CarState::Racing {
            let accel = key_input.map_or(0.0, |k| match k.acceleration {
                None => 0.0,
                Some(Accelerator::Accelerate) => consts.forward_acceleration_mss,
                Some(Accelerator::Brake) => {
                    if self.velocity > 0.0 {
                        -consts.braking_acceleration_mss
                    } else {
                        -consts.reverse_acceleration_mss
                    }
                }
            });
            self.velocity += accel * delta_time_s;
            // Must be a neater way to get the sign right
            let sign = if accel > 0.0 { 1.0 } else { -1.0 };
            let pos_change = self.velocity * delta_time_s + 0.5 * delta_time_s * accel * accel * (sign);
            let capped_pos_change = if (pos_change > consts.max_forward_speed_ms * delta_time_s) {
                self.velocity * delta_time_s
            } else {
                pos_change
            };
            let direction_change = key_input.map_or(0.0, |k| match k.direction {
                None => 0.0,
                Some(Direction::Left) => -consts.turn_rate_rs,
                Some(Direction::Right) => consts.turn_rate_rs,
            });
            let theta_change = delta_time_s * direction_change;
            self.direction_radians += theta_change;
            let x_change = capped_pos_change * self.direction_radians.sin();
            let y_change = capped_pos_change * self.direction_radians.cos();
            self.previous_pos = self.pos;
            self.pos.x += x_change;
            self.pos.y += y_change;
            Some((x_change, y_change, -theta_change))
        } else {
            None
        }
    }

    pub fn update_state(&mut self, track: &Track, game_state: &mut CarProgress, game_time_s: f32) {
        match self.state {
            CarState::Racing => {
                if track.is_within_track(&self.pos) {
                    if track.is_finished(&self.pos) {
                        game_state.end_time = Some(game_time_s);
                        self.state = CarState::Finished;
                    } else if track.termination_condition.is_timed_out(game_state.ticks, game_time_s - game_state.start_time) {
                        self.state = CarState::TimedOut;
                        game_state.end_time = Some(game_time_s);
                    } else {
                        game_state.ticks += 1;
                        self.state = CarState::Racing;
                    }
                } else {
                    game_state.end_time = Some(game_time_s);
                    self.state = CarState::Crashed
                }
                game_state.state = self.state;
            }
            _ => {}
        }
    }
}
