use std::ops::Neg;
use bevy::{
    prelude::*,
};
// use bevy_ninepatch::*;
use bevy_nine_slice_ui::NineSliceMaterial;
use bevy_debug_grid::*;
use rust_driving_game::car::{Car, CarState, PhysicsConstants};
use rust_driving_game::coordinates::{Boundary, LineType, Vec2d};
use rust_driving_game::input::{Accelerator, Direction, KeyInput};
use rust_driving_game::track::{ParallelRectSection, Track};
use rust_driving_game::debug_grid::spawn_floor_grid;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, DebugGridPlugin::without_floor_grid()))
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        // .insert_non_send_resource(track)
        .add_systems(Startup, (setup, spawn_floor_grid))
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
        bundle.sprite_bundle.transform.rotate_around(start_loc, Quat::from_rotation_z((end_loc - start_loc).angle_between(Vec3::Y)));
        bundle
    }
}



fn make_track(
    // world: &mut World
) -> TrackComponent {
    let track_sect = ParallelRectSection {
        left_x: -50.0,
        right_x: 50.0,
        top_y: 380.0,
        bottom_y: -10.0,
    };
    let track = Track{
        start: Default::default(),
        finish_line: Boundary::horizontal(350.0, true),
        sections: vec![Box::new(track_sect)],
    };
    // world.insert_non_send_resource(TrackComponent(track));
    TrackComponent(track)
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

    commands.spawn(
        make_track()
    );

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
        if acc.is_some() || dir.is_some() {
            Some(KeyInput::new(acc, dir))
        } else {
            None
        }
    };
    match car.1.0.state {
        CarState::StartLine if key_input.is_some() => car.1.0.state = CarState::Racing,
        CarState::Racing => {
            let consts = PhysicsConstants::default();
            let (x_d, y_d, theta_d) = car.1.0.update_position(&consts, time.delta_seconds(), key_input);
            car.0.rotate_local_z(theta_d);
            car.0.translation += Vec3::new(x_d, y_d, 0.0);
        }
        _ => {}
    }
}

fn check_state(
    mut car_query: Query<&mut CarComponent>,
    track_res: Query<&TrackComponent>,
    mut text_query: Query<&mut Text>,
) {
    let mut car = car_query.single_mut();
    let track = track_res.single();
    car.0.update_state(&track.0);
    let state = car.0.state;
    let mut text = text_query.single_mut();
    text.sections[2].value = state.to_string().into();

}