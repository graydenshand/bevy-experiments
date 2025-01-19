//! A simple 3D scene with light shining over a cube sitting on a plane.

use core::f32;
use std::f32::consts::PI;

use bevy::{input::keyboard, prelude::*};
use std::{f32::consts::FRAC_PI_2, ops::Range};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CameraSettings>()
        .insert_resource(PositionLoggingTimer(Timer::from_seconds(
            0.1,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, setup)
        .add_systems(Update, (keyboard_input_system, position_logging_system))
        .run();
}

#[derive(Debug, Resource)]
struct CameraSettings {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }
}

#[derive(Component)]
struct PositionText;

const GRID_SIZE_X: f32 = 1000.;
const GRID_SIZE_Y: f32 = 1000.;
const GRID_INTERVAL_X: f32 = 10.0;
const GRID_INTERVAL_Y: f32 = 10.0;
const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
const PITCH_RANGE: Range<f32> = -PITCH_LIMIT..PITCH_LIMIT;
/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // X-axis grid lines
    for i in ((-1. * GRID_SIZE_X / 2.) as i32..=(GRID_SIZE_X / 2.) as i32)
        .step_by(GRID_INTERVAL_X as usize)
    {
        // Map grid line index to x_position. index_0=-GRID_SIZE_X / 2, index_max=GRID_SIZE_X / 2 - GRID_INTERVAL_X
        commands.spawn((
            Mesh3d(meshes.add(Rectangle::new(1., GRID_SIZE_Y + 1.))),
            MeshMaterial3d(materials.add(Color::BLACK)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                .with_translation(Vec3::from_array([i as f32, 0., 0.])),
        ));
    }
    for j in ((-1. * GRID_SIZE_Y / 2.) as i32..=(GRID_SIZE_Y / 2.) as i32)
        .step_by(GRID_INTERVAL_Y as usize)
    {
        commands.spawn((
            Mesh3d(meshes.add(Rectangle::new(GRID_SIZE_X + 1., 1.))),
            MeshMaterial3d(materials.add(Color::BLACK)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                .with_translation(Vec3::from_array([0., 0., j as f32])),
        ));
    }

    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 20.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 10., 0.0),
    ));
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
    // camera_settings: Res<CameraSettings>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (yaw, pitch, roll) = camera.rotation.to_euler(EulerRot::YXZ);
    let mut delta_pitch = 0.;
    let mut delta_yaw = 0.;
    if keyboard_input.pressed(KeyCode::KeyL) {
        delta_yaw = -1.;
    }
    if keyboard_input.pressed(KeyCode::KeyJ) {
        delta_yaw = 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyI) {
        delta_pitch = 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyK) {
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
            Vec3::from_array([-GRID_SIZE_X / 2., 20., -GRID_SIZE_Y / 2.]),
            Vec3::from_array([GRID_SIZE_X / 2., 20., GRID_SIZE_Y / 2.]),
        );
}
