use bevy::{asset::Handle, ecs::system::Resource, render::mesh::Mesh, sprite::ColorMaterial};

#[derive(Resource, Default)]
pub struct WorldSize(pub f32, pub f32);

#[derive(Resource)]
pub struct ColorHandles {
    pub red: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
}

#[derive(Resource)]
pub struct PheromoneMeshHandle(pub Handle<Mesh>);
