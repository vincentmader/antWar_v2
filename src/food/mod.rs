use std::sync::atomic::Ordering;

use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        entity::Entity,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, ParallelCommands, Query, Res},
    },
    math::{Vec2, Vec3},
    prelude::default,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};
use rand::Rng;

use crate::{Amount, Food, WorldSize};

use self::bundle::FoodBundle;

pub mod bundle;
pub mod components;

pub struct FoodPlugin;

impl Plugin for FoodPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup.after(crate::world::setup))
            .add_systems(Update, food_removal);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, world_size: Res<WorldSize>) {
    let mut rng = rand::thread_rng();

    for _ in 0..50 {
        let position = Vec3::new(
            rng.gen_range((-world_size.0 / 2f32)..(world_size.0 / 2.)),
            rng.gen_range((-world_size.1 / 2f32)..(world_size.1 / 2.)),
            -1.0,
        );

        commands.spawn(FoodBundle {
            food: Food::Fungi,
            amount: 500.into(),

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
}

fn food_removal(commands: ParallelCommands, query: Query<(Entity, &Amount), With<Food>>) {
    query.par_iter().for_each(|(entity, amount)| {
        if amount.load(Ordering::Relaxed) == 0 {
            commands.command_scope(move |mut commands| {
                commands.entity(entity).despawn();
            });
        }
    })
}
