use bevy::{
    app::{Plugin, Startup},
    asset::Assets,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        query::With,
        system::{Commands, Query, ResMut},
    },
    math::primitives::Circle,
    render::{color::Color, mesh::Mesh},
    sprite::ColorMaterial,
    window::{PrimaryWindow, Window},
};

use self::resources::{ColorHandles, PheromoneMeshHandle, WorldSize};

pub mod resources;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup);
    }
}

pub fn setup(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // ====== set WorldSize
    let window = window_query.get_single().unwrap();
    commands.insert_resource(WorldSize(window.width(), window.height()));

    // ====== set ColorHandles
    let red = materials.add(Color::rgba(1.0, 0.0, 0.0, 1.0));
    let green = materials.add(Color::rgba(0.0, 1.0, 0.0, 1.0));
    commands.insert_resource(ColorHandles { red, green });

    // ====== set PheromoneMeshHandle
    let pheromone_mesh = meshes.add(Circle { radius: 15.0 });
    commands.insert_resource(PheromoneMeshHandle(pheromone_mesh));

    // ====== spawn Camera
    commands.spawn(Camera2dBundle::default());
}
