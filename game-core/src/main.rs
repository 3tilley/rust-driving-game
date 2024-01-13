use rust_driving_game_core::car::Car;
use rust_driving_game_core::default_tracks::make_track;
use rust_driving_game_core::gameloop::{run_until, TIME_PER_TICK};
use rust_driving_game_core::input::{InputProvider, KeyInput, SingleInput};

fn main() {
    println!("Hello, world!");

    let track = make_track();

    let mut inputs: Vec<Box<dyn InputProvider>> = vec![
        KeyInput::from_directions(true, false, false, false),
        KeyInput::from_directions(false, true, false, false),
        KeyInput::from_directions(false, false, true, false),
        KeyInput::from_directions(false, false, false, true),
    ].into_iter().map(|k| Box::new(SingleInput::from(k)) as Box<dyn InputProvider>).collect();
    let mut cars = ["Up", "Down", "Left", "Right"].map(|label| Car::new(track.start, label)).to_vec();
    let car_input = cars.iter_mut().zip(inputs.iter_mut()).collect::<Vec<_>>();
    let progress  = run_until(car_input, &track, TIME_PER_TICK);
    for (i, car) in cars.iter().enumerate() {
        println!("Car: {}. {} in {}", car.label, car.state.to_string(), progress[i].end_time.unwrap() - progress[i].start_time)
    }
}
