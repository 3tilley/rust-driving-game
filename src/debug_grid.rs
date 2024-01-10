use bevy::prelude::*;
use bevy_debug_grid::*;
pub fn spawn_floor_grid(mut commands: Commands) {
    // Floor grid
    commands.spawn((
        Grid {
            spacing: 5.0_f32,
            count: 32,
            color: Color::BLACK,
            ..default()
        },
        SubGrid {
            count: 4,
            color: Color::GRAY,
        },
        GridAxis::new_rgb(),
        TrackedGrid {
            alignment: GridAlignment::X,
            ..default()
        },
        TransformBundle::from_transform(
            Transform::from_xyz(0.0, 0.0, 0.1).with_rotation(Quat::from_rotation_x(1.0))
        ),
        VisibilityBundle::default(),
    ));

    // // Point light
    // commands.spawn(PointLightBundle {
    //     transform: Transform::from_xyz(4.0_f32, 4.0_f32, 4.0_f32),
    //     ..default()
    // });
}