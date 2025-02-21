use bevy::asset::{AssetServer, Assets, Handle, RenderAssetUsages};
use bevy::color::Color;
use bevy::hierarchy::ChildBuilder;
use bevy::image::Image;
use bevy::prelude::{
    default, AlignItems, BackgroundColor, BuildChildren, Camera, Camera2d, ChildBuild, Commands,
    FlexDirection, Font, JustifyContent, Node, Res, ResMut, TargetCamera, Text, TextColor,
    TextFont, Val,
};
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};

pub struct CubeFace {
    text: &'static str,
    color: Color,
}

impl CubeFace {
    pub const fn new(text: &'static str, color: Color) -> Self {
        Self { text, color }
    }
}

pub const EXAMPLE_CUBE_FACES: [CubeFace; 6] = [
    CubeFace::new("X", Color::srgb(1., 0., 0.)),
    CubeFace::new("-X ", Color::srgb(0.5, 0., 0.)),
    CubeFace::new("Y", Color::srgb(0., 1., 0.)),
    CubeFace::new("-Y ", Color::srgb(0., 0.5, 0.)),
    CubeFace::new("Z", Color::srgb(0., 0., 1.)),
    CubeFace::new("-Z ", Color::srgb(0., 0., 0.5)),
];

pub fn generate_cube_faces(
    commands: &mut Commands,
    images: &mut ResMut<Assets<Image>>,
    asset_server: Res<AssetServer>,
    faces: [CubeFace; 6],
) -> Handle<Image> {
    let size = Extent3d {
        width: 6 * 512,
        height: 512,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Bgra8UnormSrgb,
        RenderAssetUsages::default(),
    );
    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);
    let font = asset_server.load("fonts/FiraSans-Medium.ttf");

    let texture_camera = commands
        .spawn((
            Camera2d,
            Camera {
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Node {
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            TargetCamera(texture_camera),
        ))
        .with_children(|builder| {
            for face in faces {
                spawn_cube_face(builder, font.clone(), face.color, &face.text);
            }
        });

    image_handle
}

/// Create a square face
///
///
fn spawn_cube_face(
    builder: &mut ChildBuilder,
    font: Handle<Font>,
    background_color: Color,
    text: &str,
) {
    builder
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.),
                height: Val::Percent(100.),
                ..default()
            },
            BackgroundColor(background_color),
        ))
        .with_children(|builder| {
            builder.spawn((
                Text::new(text),
                TextFont {
                    font,
                    font_size: 300.,
                    ..default()
                },
                TextColor::BLACK,
            ));
        });
}
