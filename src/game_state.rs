use crate::car::CarState;

pub struct GameState {
    pub ticks: u64,
    pub start_time: f32,
    pub end_time: Option<f32>,
    pub state: CarState,
}
