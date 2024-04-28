use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    ecs::{
        schedule::IntoSystemConfigs,
        system::{Commands, Res},
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
        app.add_systems(Startup, setup.after(crate::world::setup));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, world_size: Res<WorldSize>) {
    let mut rng = rand::thread_rng();

    for _ in 0..5 {
        let position = Vec3::new(
            rng.gen_range((-world_size.0 / 2f32)..(world_size.0 / 2.)),
            rng.gen_range((-world_size.1 / 2f32)..(world_size.1 / 2.)),
            -1.0,
        );

        commands.spawn(FoodBundle {
            food: Food::Fungi,
            amount: Amount(10.0),

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
