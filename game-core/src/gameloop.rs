use crate::car::{Car, CarState, PhysicsConstants};
use crate::car_progress::CarProgress;
use crate::input::InputProvider;
use crate::track::Track;

pub const TIME_PER_TICK: f32 = 1.0 / 240.0;

pub fn run_until(cars: Vec<(&mut Car, &mut Box<dyn InputProvider>)>, track: &Track, time_per_tick_s: f32) -> Vec<CarProgress> {
    let mut progress = vec![CarProgress::new(0.0); cars.len()];
    let mut cars_progress = cars.into_iter().zip(progress.iter_mut()).collect::<Vec<_>>();

    let mut ticks = 0;
    let mut time = 0.0;

    let physics = PhysicsConstants::default();


    loop {
        let mut still_racing = false;
        // for ((ref mut car, &mut input), mut progress) in cars_progress.iter() {
        for item in cars_progress.iter_mut() {
            // println!("Tick: {}", ticks);
            let car: &mut Car = &mut item.0.0;
            let input = &item.0.1;
            let progress : &mut CarProgress = item.1;

            // if ticks % 240 == 0 {
            //     println!("{}: {:?} - {:?}", car.label, car.pos, car.state);
            // }
            let key_input= Some(input.get_input());
            let _change = car.update_position(&physics, time_per_tick_s, key_input);
            car.update_state(track, progress, time);
            still_racing = still_racing || (car.state == CarState::Racing || car.state == CarState::StartLine);
        }
        if !still_racing {
            break;
        }
        ticks += 1;
        time += time_per_tick_s;

    }
     return progress
}