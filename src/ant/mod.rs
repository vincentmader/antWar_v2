use std::f32::consts::PI;

use bevy::{
    app::{App, Plugin, Startup, Update},
    asset::AssetServer,
    ecs::{
        query::{With, Without},
        schedule::IntoSystemConfigs,
        system::{Commands, ParallelCommands, Query, Res},
    },
    math::{Quat, Vec2, Vec3},
    prelude::default,
    sprite::{Sprite, SpriteBundle},
    time::Time,
    transform::components::Transform,
};
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{
    world::resources::{ColorHandles, PheromoneMeshHandle, WorldSize},
    Age, Amount, Colony, Food, Pheromone, Player,
};

use self::{
    abilities::Vision,
    bundle::AntBundle,
    components::{Abilities, Ant, Cargo, Direction, Speed},
};

mod abilities;
mod bundle;
pub mod components;

pub enum CargoType {
    // type of food and position
    Food(Food),
}
pub enum AntAction {
    Rotate(Quat),
    DropPheromone(Pheromone),
    Accelerate(f32),
    DropCargo,
    PickupCargo(CargoType, u64),
}

pub struct AntPlugin;

impl Plugin for AntPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup.after(crate::world::setup))
            // app.add_systems(Startup, setup)
            .add_systems(Update, ant_actions)
            .add_systems(Update, movement.after(ant_actions))
            .add_systems(Update, confine_ant_movement.after(movement));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    for _ in 0..500 {
        let ant_size = 20. + 10. * rng.gen::<f32>();

        // let ant_initial_position = Vec3::new(
        //     rng.gen_range((-world_size.0 / 2.)..(world_size.0 / 2.)),
        //     rng.gen_range((-world_size.1 / 2.)..(world_size.1 / 2.)),
        //     0.,
        // );
        let ant_initial_position = Vec3::new(0., 0., 0.);
        let ant_initial_speed = 150. + 70. * rng.gen::<f32>();
        let ant_initial_rotation = Quat::from_rotation_z(2.0 * PI * rng.gen::<f32>());
        let ant_initial_direction = ant_initial_rotation * Vec3::Y;

        commands.spawn(AntBundle {
            _type: Ant,

            age: Age(0.),
            speed: Speed(ant_initial_speed),
            direction: Direction(ant_initial_direction),
            cargo: Cargo::Empty,

            player: Player::Natural,

            abilities: Abilities {
                vision: Vision { distance: 50. },
                speed: 300.,
            },

            sprite: SpriteBundle {
                texture: asset_server.load("img/ant64.png"),
                transform: Transform {
                    translation: ant_initial_position,
                    rotation: ant_initial_rotation,
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2::new(1., 1.) * ant_size),
                    ..default()
                },
                ..default()
            },
        });
    }
}

fn movement(mut ant_query: Query<(&mut Transform, &Speed, &Direction)>, time: Res<Time>) {
    ant_query
        .par_iter_mut()
        .for_each(|(mut transform, speed, direction)| {
            transform.translation += **direction * **speed * time.delta_seconds()
        })
}

fn confine_ant_movement(
    mut ant_query: Query<(&mut Transform, &mut Direction), With<Ant>>,
    world_size: Res<WorldSize>,
) {
    let width = world_size.0;
    let height = world_size.1;

    ant_query
        .par_iter_mut()
        .for_each(|(mut transform, mut direction)| {
            let x = transform.translation.x;
            let y = transform.translation.y;

            if x < -width / 2.0 + 20.0 {
                let rot = direction.bounce_rotation(Vec3::X);
                direction.rotate(rot);
                transform.rotate(rot);
                transform.translation.x = -transform.translation.x - width + 40.;
            } else if x > width / 2.0 - 20.0 {
                let rot = direction.bounce_rotation(Vec3::X);
                direction.rotate(rot);
                transform.rotate(rot);
                transform.translation.x = -transform.translation.x + width - 40.;
            }
            if y < -height / 2.0 + 20.0 {
                let rot = direction.bounce_rotation(Vec3::Y);
                direction.rotate(rot);
                transform.rotate(rot);
                transform.translation.y = -transform.translation.y - height + 40.;
            } else if y > height / 2.0 - 20.0 {
                let rot = direction.bounce_rotation(Vec3::Y);
                direction.rotate(rot);
                transform.rotate(rot);
                transform.translation.y = -transform.translation.y + height - 40.;
            }
        });
}

fn ant_actions(
    mut ant_query: Query<
        (
            &mut Speed,
            &Abilities,
            &mut Transform,
            &mut Direction,
            &Player,
            &Age,
            &mut Cargo,
            &Abilities,
        ),
        With<Ant>,
    >,
    phr_query: Query<(&Pheromone, &Transform, &Player), Without<Ant>>,
    // mut food_query: Query<(&mut Food, &Transform), Without<Ant>>,
    food_query: Query<(&Food, &Transform, &Amount), Without<Ant>>,
    colony_query: Query<(&Transform, &Player), (Without<Ant>, With<Colony>)>,

    commands: ParallelCommands,
    color_handles: Res<ColorHandles>,
    phr_mesh_handle: Res<PheromoneMeshHandle>,
    // phr_list: Res<PheromoneList>,
    // world_size: Res<WorldSize>,
) {
    let phr_all = phr_query.iter().collect::<Vec<_>>().into_par_iter();
    ant_query
        .iter_mut()
        .collect::<Vec<_>>()
        .into_par_iter()
        .for_each(
            // ant_query.par_iter_mut().for_each(
            // ant_query.iter_mut().for_each(
            |(
                mut ant_speed,
                Abilities { vision, speed: _ },
                mut ant_transform,
                mut ant_direction,
                ant_player,
                ant_age,
                mut ant_cargo,
                ant_abilities,
            )| {
                // let phrs = {
                //     let list = phr_list.data_read();

                //     // TODO: check units_per_item => retrieve multiple items
                //     let iter =
                //         match list.get(&phr_list.coords_to_ids(
                //             ant_transform.translation.x,
                //             ant_transform.translation.y,
                //         )) {
                //             Some(v) => v.par_iter(),
                //             None => [].par_iter(),
                //         };

                //     iter.flat_map(|e| phr_query.get(*e))
                // let phrs = phr_query
                //     .iter()
                let phrs = phr_all
                    .clone()
                    // let phrs = phrs_iter
                    // let phrs = phr_query
                    //     .iter()
                    //     .collect::<Vec<_>>()
                    //     .into_par_iter()
                    .filter_map(|(phr, phr_transform, phr_player)| {
                        let mut phr_translation = phr_transform.translation;
                        phr_translation.z = 0.;

                        let relative = phr_translation - ant_transform.translation;
                        let distance = relative.length();
                        if distance < vision.distance
                            && ant_direction.angle_between(relative) < PI / 2.0
                        {
                            Some((relative, *phr, *phr_player))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();
                // };

                // let (foods, foods_mut) = food_query
                let (foods, food_amount) = food_query
                    .iter()
                    .filter_map(|(food, food_transform, amount)| {
                        let mut food_translation = food_transform.translation;
                        food_translation.z = 0.;

                        let relative = food_translation - ant_transform.translation;
                        let distance = relative.length();
                        if distance < vision.distance
                            && ant_direction.angle_between(relative) < PI / 2.0
                        {
                            Some(((relative, *food), (relative, food, amount)))
                            // Some((relative, *food))
                        } else {
                            None
                        }
                    })
                    // .collect::<Vec<_>>();
                    .unzip::<_, _, Vec<_>, Vec<_>>();

                let colonies = colony_query
                    .iter()
                    .filter_map(|(col_transform, col_player)| {
                        let mut col_translation = col_transform.translation;
                        col_translation.z = 0.;
                        let relative = col_translation - ant_transform.translation;
                        let distance = relative.length();
                        if distance < vision.distance
                            && ant_direction.angle_between(relative) < PI / 2.0
                        {
                            Some((relative, *col_player))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>();

                let action = ant_player.ant_action(
                    &ant_direction.normalize(),
                    **ant_speed,
                    **ant_age,
                    &ant_cargo,
                    *ant_player,
                    ant_abilities,
                    &phrs,
                    &foods,
                    &colonies,
                );

                // TODO: use Events but how to send them in parallel??
                match action {
                    AntAction::DropCargo => {
                        *ant_cargo = Cargo::Empty;
                    }
                    AntAction::PickupCargo(ct, amnt) => match ct {
                        CargoType::Food(ft) => {
                            let food =
                                food_amount
                                    .into_par_iter()
                                    .find_any(|(pos, food, _amount)| {
                                        pos.length() < 10. && **food == ft
                                    });

                            if let Some((_food_pos, _food_type, amount)) = food {
                                // TODO: decrease Amount of Food

                                *ant_cargo = Cargo::Food {
                                    typ: ft,
                                    amound: amount.take(amnt),
                                }
                            }
                        }
                    },
                    AntAction::Rotate(rotation) => {
                        ant_transform.rotate(rotation);
                        ant_direction.rotate(rotation);
                    }
                    AntAction::Accelerate(speed) => {
                        **ant_speed = speed;
                    }
                    AntAction::DropPheromone(pheromone) => {
                        commands.command_scope(|mut commands| {
                            let mesh = phr_mesh_handle.0.clone();
                            let material = match pheromone {
                                Pheromone::Red => color_handles.red.clone(),
                                Pheromone::Green => color_handles.green.clone(),
                            };

                            Pheromone::spawn(
                                &mut commands,
                                pheromone,
                                *ant_player,
                                ant_transform.translation - Vec3::Z,
                                mesh,
                                material,
                                // &phr_list,
                                // &world_size,
                            );
                        });
                    }
                }
            },
        )
}
