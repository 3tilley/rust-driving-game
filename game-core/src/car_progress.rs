use crate::car::CarState;

#[derive(Copy, Clone, Debug, Default)]
pub struct CarProgress {
    pub ticks: u64,
    pub start_time: f32,
    pub end_time: Option<f32>,
    pub state: CarState,
}

impl CarProgress {
    pub fn new(start_time: f32) -> CarProgress {
        CarProgress {
            start_time,
            ..default()
        }
    }
}
