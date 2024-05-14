use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};

use crate::{
    ant::{components::Cargo, CargoType},
    Abilities, AntAction, Food, Pheromone, Player,
};

const PHR_DROP_PROB: f32 = 0.03;
const RANDOM_MOVE_PROB: f32 = 0.60;
const F: f32 = 0.;

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
        // ====== check for home colony
        let own_colony = colonies
            .iter()
            .find_map(|(dist, ply)| if *ply == player { Some(dist) } else { None });

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

        if let Cargo::Food {
            typ: _f,
            amount: _a,
        } = cargo
        {
            let own_pheromones = get_own_pheromones(pheromones, &player, &Pheromone::Red);
            let p = rand::random::<f32>();
            // ====== drop green pheromone or rotate in red pheromone direction
            if p < PHR_DROP_PROB {
                AntAction::DropPheromone(Pheromone::Green)
            } else if PHR_DROP_PROB < p && p < RANDOM_MOVE_PROB + PHR_DROP_PROB {
                AntAction::Rotate(generate_random_angle(0.1))
            } else if own_pheromones.is_empty() {
                AntAction::Rotate(generate_random_angle(0.1))
            } else {
                let new_direction = get_pheromone_attraction(&own_pheromones);

                let rot = Quat::from_rotation_arc(*direction, new_direction.normalize())
                    // * generate_random_angle(0.8)
                     ;
                AntAction::Rotate(rot)
            }
        } else {
            let own_pheromones = get_own_pheromones(pheromones, &player, &Pheromone::Green);
            let nearest_food = get_nearest_food(foods);
            let p = rand::random::<f32>();

            if p < PHR_DROP_PROB {
                AntAction::DropPheromone(Pheromone::Red)
            } else if PHR_DROP_PROB < p && p < RANDOM_MOVE_PROB + PHR_DROP_PROB {
                AntAction::Rotate(generate_random_angle(0.1))
            } else {
                let has_detected_pheromones = !own_pheromones.is_empty();

                if let Some(food) = nearest_food {
                    if food.0.length() < 10.0 {
                        AntAction::PickupCargo(CargoType::Food(food.1), 10)
                    } else {
                        let rot = Quat::from_rotation_arc(*direction, food.0.normalize());
                        AntAction::Rotate(rot)
                    }
                } else if has_detected_pheromones {
                    let new_direction = get_pheromone_attraction(&own_pheromones);
                    let rot = Quat::from_rotation_arc(*direction, new_direction.normalize())
                        // * generate_random_angle(0.8)
                         ;
                    AntAction::Rotate(rot)
                } else {
                    AntAction::Rotate(generate_random_angle(0.1))
                }
            }

            // ====== drop red pheromone or rotate in green pheromone direction
            // if own_pheromones.is_empty() {
            // AntAction::Rotate(generate_random_angle(0.25))
            // } else {
            //     let new_direction = get_pheromone_attraction(&own_pheromones);

            //     let rot = Quat::from_rotation_arc(*direction, new_direction.normalize())
            //         // * generate_random_angle()
            //         ;
            //     AntAction::Rotate(rot)
            // }
        }
    }
}

fn get_own_pheromones(
    pheromones: &[(Vec3, Pheromone, Player)],
    player: &Player,
    pheromone: &Pheromone,
) -> Vec<Vec3> {
    pheromones
        .par_iter()
        .filter_map(|(dir, phr, ply)| {
            if *ply == *player && *phr == *pheromone {
                Some(*dir)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}

fn get_pheromone_attraction(own_pheromones: &[Vec3]) -> Vec3 {
    own_pheromones
        .into_par_iter()
        .map(|r| {
            let abs = r.length();
            0.1 / (abs * abs) * *r
        })
        .sum::<Vec3>()
}
fn generate_random_angle(angle: f32) -> Quat {
    let rot = 2. * PI * (rand::random::<f32>() - 0.5) * angle;
    Quat::from_rotation_z(rot)
}
fn get_nearest_food(foods: &[(Vec3, Food)]) -> Option<&(Vec3, Food)> {
    foods
        .iter()
        .reduce(|x, y| if x.0.length() < y.0.length() { x } else { y })
}
