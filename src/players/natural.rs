use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    ant::{components::Cargo, CargoType},
    Abilities, AntAction, Food, Pheromone, Player,
};

const PHR_DROP_PROB: f32 = 0.03;

pub struct Natural;

impl Natural {
    pub fn ant_action(
        direction: &Vec3,
        _speed: f32,
        _age: f32,
        cargo: &Cargo,
        player: Player,
        _abilities: &Abilities,

        pheromones: &[(Vec3, Pheromone, Player)],
        foods: &[(Vec3, Food)],
        colonies: &[(Vec3, Player)],
    ) -> AntAction {
        if let Cargo::Food {
            typ: _f,
            amound: _a,
        } = cargo
        {
            // ====== check for home colony
            let own_colony =
                colonies.iter().find_map(
                    |(dist, ply)| {
                        if *ply == player {
                            Some(dist)
                        } else {
                            None
                        }
                    },
                );

            if let Some(col_direction) = own_colony
                && col_direction.length() < 10.
            {
                return AntAction::DropCargo;
            } else if let Some(col_direction) = own_colony
                && col_direction.angle_between(*direction) > 0.1
            {
                let rot = Quat::from_rotation_arc(*direction, col_direction.normalize());
                return AntAction::Rotate(rot);
            }

            // ====== drop green pheromone or rotate in red pheromone direction
            let own_pheromones = pheromones
                .par_iter()
                .filter_map(|(dir, phr, ply)| {
                    if *ply == player && *phr == Pheromone::Red {
                        Some(*dir)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            let p = rand::random::<f32>();
            if p < PHR_DROP_PROB {
                AntAction::DropPheromone(Pheromone::Green)
            } else if own_pheromones.is_empty() {
                let da = (rand::random::<f32>() - 0.5) * PI / 20.;
                let rotation = Quat::from_rotation_z(da);

                AntAction::Rotate(rotation)
            } else {
                let new_direction = own_pheromones
                    .into_par_iter()
                    .map(|r| {
                        let abs = r.length();
                        0.1 / (abs * abs) * r
                    })
                    .sum::<Vec3>();

                let rot = Quat::from_rotation_arc(*direction, new_direction.normalize());
                AntAction::Rotate(rot)
            }
        } else {
            // ====== check for nearest food
            let neares_food = foods.iter().reduce(|acc_dist, dist| {
                if acc_dist.0.length() > dist.0.length() {
                    dist
                } else {
                    acc_dist
                }
            });

            if let Some((food_direction, food_type)) = neares_food
                && food_direction.length() < 10.
            {
                return AntAction::PickupCargo(CargoType::Food(*food_type), 1);
            } else if let Some((food_direction, _ft)) = neares_food
                && food_direction.angle_between(*direction) > 0.1
            {
                let rot = Quat::from_rotation_arc(*direction, food_direction.normalize());
                return AntAction::Rotate(rot);
            }

            let p = rand::random::<f32>();

            // ====== drop red pheromone or rotate in green pheromone direction
            let own_pheromones = pheromones
                .par_iter()
                .filter_map(|(dir, phr, ply)| {
                    if *ply == player && *phr == Pheromone::Green {
                        Some(*dir)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if p < PHR_DROP_PROB {
                AntAction::DropPheromone(Pheromone::Red)
            } else if own_pheromones.is_empty() {
                let da = (rand::random::<f32>() - 0.5) * PI / 20.;
                let rotation = Quat::from_rotation_z(da);

                AntAction::Rotate(rotation)
            } else {
                let new_direction = own_pheromones
                    .into_par_iter()
                    .map(|r| {
                        let abs = r.length();
                        0.1 / (abs * abs) * r
                    })
                    .sum::<Vec3>();

                let rot = Quat::from_rotation_arc(*direction, new_direction.normalize());
                AntAction::Rotate(rot)
            }
        }
    }
}
