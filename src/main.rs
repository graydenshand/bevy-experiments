//! A simple 3D scene with light shining over a cube sitting on a plane.

use core::f32;
use std::f32::consts::PI;

use bevy::{color::palettes::css::BLUE, input::keyboard, prelude::*};
use std::{f32::consts::FRAC_PI_2, ops::Range};

mod grid;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            grid::GridPlugin {
                size_x: 100.,
                size_y: 1000.,
                interval_x: 10.,
                interval_y: 10.,
            },
        ))
        .insert_resource(PositionLoggingTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .insert_resource(RectangleColor { handle: None })
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input_system, position_logging_system))
        .run();
}

#[derive(Component)]
struct PositionText;

#[derive(Component)]
struct BlueRectangle;

#[derive(Resource)]
struct RectangleColor {
    handle: Option<Handle<StandardMaterial>>,
}

const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
const PITCH_RANGE: Range<f32> = -PITCH_LIMIT..PITCH_LIMIT;
const BLUE_RECT_POSITION: Vec3 = Vec3::from_array([0., 10., 0.]);
const ACTION_RADIUS: f32 = 30.;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut rect_color: ResMut<RectangleColor>,
) {
    // cube
    let rect_color_handle = materials.add(Color::srgb_u8(0, 0, 255));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 20.0, 1.0))),
        MeshMaterial3d(rect_color_handle.clone()),
        Transform::from_xyz(
            BLUE_RECT_POSITION.x,
            BLUE_RECT_POSITION.y,
            BLUE_RECT_POSITION.z,
        ),
        BlueRectangle,
    ));

    rect_color.handle = Some(rect_color_handle);

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: f32::MAX,
            range: 1000.,
            radius: 10.,
            ..default()
        },
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
    // camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 20., 40.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // position text
    commands.spawn((Text::new("Position: "), PositionText));
}

#[derive(Resource)]
struct PositionLoggingTimer(Timer);

fn position_logging_system(
    time: Res<Time>,
    mut timer: ResMut<PositionLoggingTimer>,
    camera: Single<&mut Transform, With<Camera>>,
    mut text: Single<&mut Text, With<PositionText>>,
) {
    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
    if timer.0.tick(time.delta()).just_finished() {
        **text = Text::new(format!("Position: x={:.2}, y={:.2}, z={:.2}\n Rotation: yaw={yaw:.2}, pitch={pitch:.2}, roll={roll:.2}", camera.translation.x, camera.translation.y, camera.translation.z ));
    }
}

fn keyboard_input_system(
    mut camera: Single<&mut Transform, With<Camera>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    grid: Res<grid::Grid>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    rect_color: ResMut<RectangleColor>,
) {
    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
    let mut delta_pitch = 0.;
    let mut delta_yaw = 0.;
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        delta_yaw = -1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        delta_yaw = 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        delta_pitch = 1.;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        delta_pitch = -1.;
    }
    // Establish the new yaw and pitch, preventing the pitch value from exceeding our limits.
    let new_pitch =
        (pitch + delta_pitch * time.delta_secs()).clamp(PITCH_RANGE.start, PITCH_RANGE.end);
    camera.rotation = Quat::from_euler(
        EulerRot::YXZ,
        yaw + (delta_yaw * time.delta_secs()),
        new_pitch,
        roll,
    );

    let mut horizontal_translation = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyA) {
        horizontal_translation = camera.local_x() * -1.;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        horizontal_translation = camera.local_x() * 1.;
    }

    let mut verticle_translation = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::KeyS) {
        let mut delta = camera.forward() * 1.;
        delta[1] = 0.;
        verticle_translation = -delta;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        let mut delta = camera.forward() * 1.;
        delta[1] = 0.;
        verticle_translation = delta;
    }
    camera.translation = (camera.translation + horizontal_translation + verticle_translation)
        .clamp(
            Vec3::from_array([-grid.size_x / 2., 20., -grid.size_y / 2.]),
            Vec3::from_array([grid.size_x / 2., 20., grid.size_y / 2.]),
        );

    if let Some(handle) = &rect_color.handle {
        if let Some(mat) = materials.get_mut(handle) {
            if camera.translation.distance(BLUE_RECT_POSITION) < ACTION_RADIUS {
                mat.base_color = Color::srgb_u8(255, 0, 0);
            } else {
                mat.base_color = Color::srgb_u8(0, 0, 255);
            }
        };
    }
}
