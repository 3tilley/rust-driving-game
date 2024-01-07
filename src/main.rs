use bevy::{
    prelude::*,
};
use rust_driving_game::car::{Car, PhysicsConstants};
use rust_driving_game::input::{Accelerator, Direction, KeyInput};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            move_car,
            ))
        .run();
}

const CAR_SIZE: Vec3 = Vec3::new(2.0, 5.0, 0.0);
const CAR_COLOUR: Color = Color::rgb(0.3, 0.3, 0.7);


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: CAR_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: CAR_COLOUR,
                ..default()
            },
            ..default()
        }
        ,CarComponent(Car::default())));
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
        Some(KeyInput::new(acc, dir))
    };
    let consts = PhysicsConstants::default();
    let (x_d, y_d, theta_d) = car_transform.1.0.update_position(&consts, time.delta_seconds(), key_input);
    car_transform.0.rotate_local_z(theta_d);
    car_transform.0.translation += Vec3::new(x_d, y_d, 0.0);
}