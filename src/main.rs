
//! Shows how to render UI to a texture. Useful for displaying UI in 3D space.

mod cube_mesh;

use crate::cube_mesh::create_cube_mesh;
use bevy::prelude::*;
use core::f32::consts::PI;
use cube_text_example::{generate_cube_faces, EXAMPLE_CUBE_FACES};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, rotator_system)
        .run();
}

// Marks the cube, to which the UI texture is applied.
#[derive(Component)]
struct Cube;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let cube_handle = meshes.add(create_cube_mesh());
    let cube_faces_texture = generate_cube_faces(&mut commands, &mut images, asset_server, EXAMPLE_CUBE_FACES);

    // This material has the texture that has been rendered.
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(cube_faces_texture),
        unlit: true,
        ..default()
    });

    // Cube with material containing the rendered UI texture.
    commands.spawn((
        Mesh3d(cube_handle),
        MeshMaterial3d(material_handle),
        Transform::from_xyz(0.0, 0.0, 1.5)
            .with_scale(Vec3::splat(2.0))
            .with_rotation(Quat::from_rotation_x(-PI / 5.0)),
        Cube,
    ));

    // // The main pass camera.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 2,
            ..default()
        },
    ));
}

const ROTATION_SPEED: f32 = 0.5;

fn rotator_system(time: Res<Time>, mut query: Query<&mut Transform, With<Cube>>) {
    for mut transform in &mut query {
        transform.rotate_x(1.0 * time.delta_secs() * ROTATION_SPEED);
        transform.rotate_y(0.7 * time.delta_secs() * ROTATION_SPEED);
    }
}
