use bevy::{
    prelude::*,
};
use rust_driving_game::car::Car;
use rust_driving_game::input::{Accelerator, KeyInput};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        // .add_startup_system(setup.system())
        .add_system(FixedUpdate, (
            move_car,
            ))
        .run();
}

#[derive(Component)]
struct CarComponent(Car);

fn move_car(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CarComponent)>,
    time: Res<Time>,
) {
    let mut car_transform = query.single_mut();
    let key_input = {
        let is_up = keyboard_input.pressed(KeyCode::Up);
        let is_down = keyboard_input.pressed(KeyCode::Down);
        let is_left = keyboard_input.pressed(KeyCode::Left);
        let is_right = keyboard_input.pressed(KeyCode::Right);
        let acc = Accelerator::from_up_down(is_up, is_down);
        let dir = Direction::from_left_right(is_left, is_right);
        KeyInput::new(acc, dir)
    };
    let transform = car_transform.



}