use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};

use crate::{AntAction, Food, Pheromone, Player};

pub struct Random;

impl Random {
    pub fn ant_action(
        _velocity: &Vec3,
        nearby_pheromones: &[(Pheromone, Player, Vec3)],
        nearby_food: &[(Food, Vec3)],
    ) -> AntAction {
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
