use bevy::{
    prelude::*,
};
use rust_driving_game::car::{Car, PhysicsConstants};
use rust_driving_game::coordinates::{Boundary, Vec2d};
use rust_driving_game::input::{Accelerator, Direction, KeyInput};
use rust_driving_game::track::{ParallelRectSection, Track};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        // .insert_non_send_resource(track)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            move_car,
            check_state,
            ))
        .run();
}

const CAR_SIZE: Vec3 = Vec3::new(2.0, 5.0, 0.0);
const CAR_COLOUR: Color = Color::rgb(0.3, 0.3, 0.7);
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const TEXT_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const SCORE_COLOR: Color = Color::rgb(1.0, 0.5, 0.5);
// const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);
const WALL_COLOR: Color = Color::BLACK;

// This bundle is a collection of the components that define a "wall" in our game
#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
}

const WALL_WIDTH: f32 = 5.0;

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    pub fn new(loc_1: Vec2d, loc_2: Vec2d) -> WallBundle {
        let start_loc = Vec2::new(loc_1.x, loc_1.y).extend(0.0);
        let end_loc = Vec2::new(loc_2.x, loc_2.y).extend(0.0);
        let len = loc_1.distance(loc_2);
        let mut transform = Transform {
            // We need to convert our Vec2 into a Vec3, by giving it a z-coordinate
            // This is used to determine the order of our sprites
            translation: start_loc,
            // The z-scale of 2D objects must always be 1.0,
            // or their ordering will be affected in surprising ways.
            // See https://github.com/bevyengine/bevy/issues/4149
            scale: Vec2::new(WALL_WIDTH, len).extend(1.0),
            ..default()
        };
        transform.translation -= Vec3::new(0.0, len / 2.0, 0.0);
        transform.rotate_z((end_loc - start_loc).angle_between(Vec3::Y));
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform,
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
        }
    }
}



fn make_track(
    // world: &mut World
) -> TrackComponent {
    let track_sect = ParallelRectSection {
        left_x: -20.0,
        right_x: 20.0,
        top_y: 50.0,
        bottom_y: -10.0,
    };
    let track = Track{
        start: Default::default(),
        finish_line: Boundary::horizontal(45.0, true),
        sections: vec![Box::new(track_sect)],
    };
    // world.insert_non_send_resource(TrackComponent(track));
    TrackComponent(track)
}

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

    // Scoreboard
    commands.spawn(
        TextBundle::from_sections([
            TextSection::new(
                "Time: ",
                TextStyle {
                    font_size: SCOREBOARD_FONT_SIZE,
                    color: TEXT_COLOR,
                    ..default()
                },
            ),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
            TextSection::from_style(TextStyle {
                font_size: SCOREBOARD_FONT_SIZE,
                color: SCORE_COLOR,
                ..default()
            }),
        ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: SCOREBOARD_TEXT_PADDING,
                left: SCOREBOARD_TEXT_PADDING,
                ..default()
            }),
    );

    let track = make_track();
    track.0.sections.iter().for_each(|section| {
        section.edges().iter().for_each(|edge| {
            commands.spawn(WallBundle::new(edge.0, edge.1));
        })
    });

    commands.spawn((
        make_track()
    ));
}

#[derive(Component)]
struct CarComponent(Car);

#[derive(Component)]
// #[derive()]
struct TrackComponent(Track);

fn move_car(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CarComponent)>,
    time: Res<Time>,
) {
    let mut car = query.single_mut();
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
    let (x_d, y_d, theta_d) = car.1.0.update_position(&consts, time.delta_seconds(), key_input);
    car.0.rotate_local_z(theta_d);
    car.0.translation += Vec3::new(x_d, y_d, 0.0);
}

fn check_state(
    mut car_query: Query<&CarComponent>,
    track_res: Query<&TrackComponent>,
    mut text_query: Query<&mut Text>,
) {
    let car = car_query.single();
    let track = track_res.single();
    let state = car.0.get_state(&track.0);
    let mut text = text_query.single_mut();
    text.sections[2] = state.to_string().into();


}