use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};

use crate::{AntAction, Food, Pheromone, Player};

pub struct Natural;

impl Natural {
    pub fn ant_action(
        direction: &Vec3,
        nearby_pheromones: &[(Pheromone, Player, Vec3)],
        nearby_food: &[(Food, Vec3)],
    ) -> AntAction {
        let nearest_food = nearby_food.iter().reduce(|nearest, next| {
            if nearest.1.length() > next.1.length() {
                next
            } else {
                nearest
            }
        });
        if let Some(nearest_food) = nearest_food {
            if nearest_food.1.length() < 10.0 {
                return AntAction::Accelerate(0.0);
            } else {
                let da = nearest_food.1.angle_between(*direction);
                return AntAction::Rotate(Quat::from_rotation_z(da));
            }
        }

        let p = rand::random::<f32>();
        if p < 0.99 {
            let da = (rand::random::<f32>() - 0.5) * PI / 12.;
            AntAction::Rotate(Quat::from_rotation_z(da))
        } else {
            let p = rand::random::<f32>();
            let pheromone = if p > 0.5 {
                Pheromone::Green
            } else {
                Pheromone::Red
            };
            AntAction::DropPheromone(pheromone)
        }
    }
}
