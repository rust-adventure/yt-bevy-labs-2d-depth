//! A shader and a material that uses it.

use bevy::{
    color::palettes::tailwind::*,
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{
        Material2d, Material2dPlugin, MaterialMesh2dBundle,
    },
};

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "material.wgsl";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::from_xyz(-100., 128., 0.)
            .with_scale(Vec3::splat(256.)),
        material: materials.add(CustomMaterial {
            color: GREEN_400.into(),
            color_texture: Some(
                asset_server.load("icon.png"),
            ),
        }),
        ..default()
    });

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::from_xyz(100., 128., 1.)
            .with_scale(Vec3::splat(256.)),
        material: materials.add(CustomMaterial {
            color: GREEN_400.into(),
            color_texture: Some(
                asset_server.load("icon.png"),
            ),
        }),
        ..default()
    });

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(-100., -128., 0.),
        texture: asset_server.load("icon.png"),
        ..default()
    });

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(100., -128., 0.),
        texture: asset_server.load("icon.png"),
        ..default()
    });
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}
