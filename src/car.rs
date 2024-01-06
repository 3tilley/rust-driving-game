use crate::input::{Direction, KeyInput};

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
            forward_acceleration_mss: 2.0,
            braking_acceleration_mss: 4.0,
            reverse_acceleration_mss: 0.5,
            max_forward_speed_ms: 50.0,
            max_reverse_speed_ms: 10.0,
            // This is roughly 90 degrees in 1 s
            turn_rate_rs: 1.6,
        }
    }
}

pub struct Car {
    // x goes from left to right: +x points right
    x: f32,
    // y goes from bottom to top: +y points up
    y: f32,
    // This is relative to the y axis. 0 points up, 90 points right
    direction_radians: f32,
    velocity: f32,

}

impl Car {
    pub fn x_velocity(&self) -> f32 {
        self.velocity * self.direction_radians.sin()
    }

    pub fn y_velocity(&self) -> f32 {
        self.velocity * self.direction_radians.cos()
    }

    pub fn update_position(&mut self, time_s: f32, key_input: Option<KeyInput>) {

    }
}