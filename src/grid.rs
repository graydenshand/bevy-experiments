use bevy::prelude::*;

pub struct GridPlugin {
    pub size_x: f32,
    pub size_y: f32,
    pub interval_x: f32,
    pub interval_y: f32,
}
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid {
            size_x: self.size_x,
            size_y: self.size_y,
            interval_x: self.interval_x,
            interval_y: self.interval_y,
        });
        app.add_systems(Startup, setup);
    }
}

#[derive(Resource)]
pub struct Grid {
    pub size_x: f32,
    pub size_y: f32,
    pub interval_x: f32,
    pub interval_y: f32,
}

fn setup(
    grid: Res<Grid>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // X-axis grid lines
    for i in ((-1. * grid.size_x / 2.) as i32..=(grid.size_x / 2.) as i32)
        .step_by(grid.interval_x as usize)
    {
        commands.spawn((
            Mesh3d(meshes.add(Rectangle::new(1., grid.size_y + 1.))),
            MeshMaterial3d(materials.add(Color::BLACK)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                .with_translation(Vec3::from_array([i as f32, 0., 0.])),
        ));
    }
    // Y-axis grid lines
    for j in ((-1. * grid.size_y / 2.) as i32..=(grid.size_y / 2.) as i32)
        .step_by(grid.interval_y as usize)
    {
        commands.spawn((
            Mesh3d(meshes.add(Rectangle::new(grid.size_x + 1., 1.))),
            MeshMaterial3d(materials.add(Color::BLACK)),
            Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
                .with_translation(Vec3::from_array([0., 0., j as f32])),
        ));
    }
}
