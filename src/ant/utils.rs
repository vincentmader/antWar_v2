use super::components::Cargo;
use crate::food::bundle::FoodBundle;
use bevy::{
    asset::AssetServer,
    ecs::system::{Commands, Res},
    math::{Vec2, Vec3},
    prelude::default,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};

pub fn spawn_cargo(
    mut commands: Commands,
    asset_server: &Res<AssetServer>,
    cargo: &Cargo,
    position: Vec3,
) {
    match cargo {
        Cargo::Food { typ, amount } => {
            commands.spawn(FoodBundle {
                food: *typ,
                amount: amount.clone(),
                sprite: SpriteBundle {
                    texture: asset_server.load("img/fungus.png"),
                    transform: Transform {
                        translation: position,
                        ..default()
                    },
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(1., 1.) * 30.0),
                        ..default()
                    },
                    ..default()
                },
            });
        }
        Cargo::Empty => {} // NOTE: Do nothing here.
    }
}
