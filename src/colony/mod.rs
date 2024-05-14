use std::{collections::HashMap, sync::atomic::Ordering};

use bevy::{
    app::{Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        schedule::IntoSystemConfigs,
        system::{Commands, ParallelCommands, Query, Res},
    },
    math::{Vec2, Vec3},
    prelude::default,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};
use rand::Rng;
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator,
};

use crate::{Age, Amount, Food, Player, WorldSize};

use self::components::FoodStorage;
mod components;

#[derive(Component)]
pub struct Colony;

#[derive(Bundle)]
pub struct ColonyBundle {
    pub _type: Colony,
    pub age: Age,
    pub player: Player,
    pub food_storage: FoodStorage,

    pub sprite: SpriteBundle,
}

pub struct ColonyPlugin;

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup.after(crate::world::setup))
            .add_systems(Update, fill_colony_storage);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, world_size: Res<WorldSize>) {
    let mut rng = rand::thread_rng();

    let players = [Player::Natural, Player::Random];

    for player in players {
        let colony_size = 50. + 15. * rng.gen::<f32>();

        let colony_position = Vec3::new(
            0., 0., // TODO
            // rng.gen_range((-world_size.0 / 2.)..(world_size.0 / 2.)),
            // rng.gen_range((-world_size.1 / 2.)..(world_size.1 / 2.)),
            0.,
        );

        commands.spawn(ColonyBundle {
            _type: Colony,
            age: Age(0.),
            food_storage: FoodStorage::default(),

            player,

            sprite: SpriteBundle {
                texture: asset_server.load("img/colony.png"),
                transform: Transform {
                    translation: colony_position,
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1., 1.) * colony_size),
                    ..default()
                },
                ..default()
            },
        });
    }
}

fn fill_colony_storage(
    commands: ParallelCommands,
    mut colony_query: Query<(&Colony, &mut FoodStorage, &Transform)>,
    food_query: Query<(Entity, &Food, &Amount, &Transform)>,
) {
    let food_iter = food_query.iter().collect::<Vec<_>>().into_par_iter();
    colony_query
        .par_iter_mut()
        .for_each(|(colony, mut food_storage, colony_transform)| {
            let foods = food_iter
                .clone()
                .filter(|(entity, food, amount, food_transform)| {
                    (colony_transform.translation - food_transform.translation).length() < 10.
                })
                .collect::<Vec<_>>();
            foods
                .into_iter()
                .for_each(|(entity, food, amount, food_transform)| {
                    match food_storage.get_mut(food) {
                        Some(original_amount) => {
                            *original_amount += amount.load(Ordering::SeqCst);
                        }
                        None => {
                            food_storage.insert(*food, amount.load(Ordering::SeqCst));
                        }
                    }
                    commands.command_scope(|mut commands| {
                        commands.entity(entity).despawn();
                    });
                });
        })
}
