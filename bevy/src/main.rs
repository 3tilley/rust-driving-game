use bevy::prelude::*;
// use bevy_ninepatch::*;
use bevy_debug_grid::*;
use rust_driving_game_core::car::{Car, CarState, PhysicsConstants};
use rust_driving_game_core::car_progress::CarProgress;
use rust_driving_game_core::coordinates::{LineType, Vec2d};
use rust_driving_game_core::default_tracks;
// use rust_driving_game_core::debug_grid::spawn_floor_grid;
use rust_driving_game_core::input::{Accelerator, Direction, KeyInput};
use rust_driving_game_core::track::Track;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        // .insert_non_send_resource(track)
        .add_systems(Startup, (setup, spawn_floor_grid))
        .add_systems(FixedUpdate, (move_car, check_state, reset_car))
        .run();
}

const CAR_SIZE: Vec3 = Vec3::new(2.0, 5.0, 0.0);
const CAR_COLOUR: Color = Color::rgb(0.3, 0.3, 0.7);
const SCOREBOARD_FONT_SIZE: f32 = 40.0;
const SCOREBOARD_TEXT_PADDING: Val = Val::Px(5.0);
const STATEBOARD_TEXT_PADDING: Val = Val::Px(40.0);
const TEXT_COLOR: Color = Color::BLACK;
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
        transform.translation += Vec3::new(0.0, len / 2.0, 0.0);
        let mut bundle = WallBundle {
            sprite_bundle: SpriteBundle {
                transform,
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
        };
        // transform.rotate_z((end_loc - start_loc).angle_between(Vec3::Y));
        bundle.sprite_bundle.transform.rotate_around(
            start_loc,
            Quat::from_rotation_z((end_loc - start_loc).angle_between(Vec3::Y)),
        );
        bundle
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut nine_patches: ResMut<Assets<NinePatchBuilder<()>>>,
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
        },
        CarComponent(Car::default()),
        CarProgressComponent(CarProgress::default()),
    ));

    // Scoreboard
    commands.spawn((
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
        }), ScoreBoard)
    );

    commands.spawn((TextBundle::from_section("",
        TextStyle {
            font_size: SCOREBOARD_FONT_SIZE,
            color: SCORE_COLOR,
            ..default()
        }).with_style(
        Style {
            position_type: PositionType::Absolute,
            top: STATEBOARD_TEXT_PADDING,
            left: SCOREBOARD_TEXT_PADDING,
            ..default()
        },
    ), StateBoard));

    let track = default_tracks::make_track();
    track.0.sections.iter().for_each(|section| {
        section.edges().iter().for_each(|edge| {
            commands.spawn(WallBundle::new(edge.0, edge.1));
        })
    });

    commands.spawn(default_tracks::make_track());

    // Finish line sprite
    let finish_line_handle = asset_server.load("finish-line-64x64.png");
    // let nine_patch_handle = nine_patches.add(NinePatchBuilder::by_margins(16,16,16,16));
    // let texture = NineSliceMaterial:: from_slice(
    //     asset_server.load("panel_atlas.png"),
    //     Rect::new(0., 0., 32., 32.),
    // );
    let (finish_pos, finish_scale) = match track.0.finish_line.line_type {
        LineType::Horizontal(y) => (Vec3::new(0.0, y, 0.0), Vec3::new(50.0, 4.0, 0.0) / 8.0),
        LineType::Vertical(x) => (Vec3::new(x, 0.0, 0.0), Vec3::new(4.0, 50.0, 0.0) / 8.0),
        LineType::Diagonal(_, _) => unimplemented!(),
    };
    let finish_line_bundle = SpriteBundle {
        transform: Transform {
            translation: finish_pos,
            scale: finish_scale,
            ..default()
        },
        texture: finish_line_handle,
        sprite: Sprite {
            flip_x: false,
            flip_y: false,
            custom_size: None,
            rect: None,
            anchor: Default::default(),
            ..default()
        },
        ..default()
    };
    commands.spawn(finish_line_bundle);
}

#[derive(Component)]
struct CarComponent(Car);

#[derive(Component)]
struct CarProgressComponent(CarProgress);

#[derive(Component)]
// #[derive()]
struct TrackComponent(Track);

#[derive(Component)]
struct StateBoard;

#[derive(Component)]
struct ScoreBoard;

fn move_car(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CarComponent, &mut CarProgressComponent)>,
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
        if acc.is_some() || dir.is_some() {
            Some(KeyInput::new(acc, dir))
        } else {
            None
        }
    };
    match car.1 .0.state {
        CarState::StartLine if key_input.is_some() => {
            car.1 .0.state = CarState::Racing;
            car.2 .0.start_time = time.elapsed_seconds();
        }
        CarState::Racing => {
            let consts = PhysicsConstants::default();
            let (x_d, y_d, theta_d) =
                car.1
                     .0
                    .update_position(&consts, time.delta_seconds(), key_input);
            car.0.rotate_local_z(theta_d);
            car.0.translation += Vec3::new(x_d, y_d, 0.0);
        }
        _ => {}
    }
}

fn reset_car(
    keyboard_input: Res<Input<KeyCode>>,
    mut car_query: Query<(&mut Transform, &mut CarComponent, &mut CarProgressComponent)>,
    track_query: Query<&TrackComponent>,
) {
    let track = track_query.single();
    let mut car_result = car_query.single_mut();
    let start_pos = track.0.start;
    if keyboard_input.pressed(KeyCode::R) {
        car_result.1 .0.reset(start_pos);
        car_result.2 .0 = CarProgress::default();
        car_result.0.translation = Vec3::new(start_pos.x, start_pos.y, 0.0);
        car_result.0.rotation = Quat::from_rotation_z(0.0);
    }
}

fn check_state(
    mut car_query: Query<(&mut CarComponent, &mut CarProgressComponent)>,
    track_res: Query<&TrackComponent>,
    mut score_query: Query<(&mut Text), (With<ScoreBoard>, Without<StateBoard>)>,
    mut state_query: Query<(&mut Text), With<StateBoard>>,
    time: Res<Time>,
) {
    let mut car = car_query.single_mut();
    let track = track_res.single();
    car.0
         .0
        .update_state(&track.0, &mut car.1 .0, time.elapsed_seconds());
    let state = car.0 .0.state;
    let mut timer = score_query.single_mut();
    let mut state_board = state_query.single_mut();
    if state == CarState::Racing {
        timer.sections[1].value = format!("{:.4}", time.elapsed_seconds() - car.1 .0.start_time);
    }
    state_board.sections[0].value = state.to_string().into();
}
