//! A simple 3D scene with light shining over a cube sitting on a plane.

use core::f32;
use std::f32::consts::PI;

use bevy::{input::keyboard, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<CameraSettings>()
        .insert_resource(PositionLoggingTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
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
            z: 1.
        }
    }
}

#[derive(Component)]
struct PositionText;

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(20.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // rectangle base
    commands.spawn((
        Mesh3d(meshes.add(Rectangle::new(1000., 1000.))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

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
    commands.spawn((
        Text::new("Position: "),
        PositionText,
    ));
}

#[derive(Resource)]
struct PositionLoggingTimer(Timer);

fn position_logging_system(time: Res<Time>, mut timer: ResMut<PositionLoggingTimer>, camera: Single<&mut Transform, With<Camera>>, mut text: Single<&mut Text, With<PositionText>>) {
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
    camera.rotation = Quat::from_euler(EulerRot::YXZ, yaw + (delta_yaw * time.delta_secs()), pitch + (delta_pitch * time.delta_secs()), roll);
    // Need to do some trig to get the x and z component of a unit step forward when the camera is not perfectly
    // rotated along the z axis
    // 
    if keyboard_input.pressed(KeyCode::KeyA) {
        camera.translation = camera.translation - camera.local_x() * 1.;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        camera.translation = camera.translation + camera.local_x() * 1.;
    }

    if keyboard_input.pressed(KeyCode::KeyS) {
        let mut delta = camera.forward() * 1.;
        delta[1] = 0.;
        camera.translation = camera.translation - delta;
    }
    if keyboard_input.pressed(KeyCode::KeyW) {
        let mut delta = camera.forward() * 1.;
        delta[1] = 0.;
        camera.translation = camera.translation + delta;
    }
    
}
