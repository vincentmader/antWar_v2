use bevy::{
    app::{Plugin, Startup},
    asset::AssetServer,
    ecs::{
        bundle::Bundle,
        component::Component,
        schedule::IntoSystemConfigs,
        system::{Commands, Res},
    },
    math::{Vec2, Vec3},
    prelude::default,
    sprite::{Sprite, SpriteBundle},
    transform::components::Transform,
};
use rand::Rng;

use crate::{Age, Player, WorldSize};

#[derive(Component)]
pub struct Colony;

#[derive(Bundle)]
pub struct ColonyBundle {
    pub _type: Colony,
    pub age: Age,
    pub player: Player,

    pub sprite: SpriteBundle,
}

pub struct ColonyPlugin;

impl Plugin for ColonyPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup.after(crate::world::setup));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, world_size: Res<WorldSize>) {
    let mut rng = rand::thread_rng();

    let players = [Player::Natural, Player::Random];

    for player in players {
        let colony_size = 50. + 15. * rng.gen::<f32>();

        let colony_position = Vec3::new(
            rng.gen_range((-world_size.0 / 2.)..(world_size.0 / 2.)),
            rng.gen_range((-world_size.1 / 2.)..(world_size.1 / 2.)),
            0.,
        );

        commands.spawn(ColonyBundle {
            _type: Colony,

            age: Age(0.),

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
