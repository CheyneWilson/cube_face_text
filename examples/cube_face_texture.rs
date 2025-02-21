//! Shows how to render UI to a texture. Useful for displaying UI in 3D space.

use bevy::prelude::*;
use cube_text_example::{generate_cube_faces, EXAMPLE_CUBE_FACES};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,

    mut images: ResMut<Assets<Image>>,
) {


    let cube_faces = generate_cube_faces(&mut commands, &mut images, asset_server, EXAMPLE_CUBE_FACES);

    let rect = meshes.add(Rectangle::new(600.0, 100.0));
    let material_handle2 = materials.add(ColorMaterial {
        texture: Some(cube_faces.clone()),
        ..default()
    });

    commands.spawn((
        Mesh2d(rect),
        MeshMaterial2d(material_handle2),
    ));

    commands.spawn((
        Camera2d,
        Camera {
            order: 2,
            ..default()
        },
    ));
}
