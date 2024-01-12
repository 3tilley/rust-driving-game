use crate::car::{Car, PhysicsConstants};
use crate::car_progress::CarProgress;
use crate::input::InputProvider;
use crate::track::Track;

pub fn run_until(cars: Vec<(&mut Car, &mut dyn InputProvider)>, track: &Track, time_per_tick_s: f32) {
    let cars_progress = cars.iter().map(|c| {
        (c.0, c.1, CarProgress::new(0.0))
    }).collect::<Vec<_>>();

    let mut ticks = 0;
    let mut time = 0.0;

    let physics = PhysicsConstants::default();

    loop {
        for (car, input, mut progress) in cars_progress {
            let key_input= Some(input.get_input());
            let _change = car.update_position(&physics, time_per_tick_s, key_input);
            car.update_state(track, &mut progress, time);
        }
    }
}