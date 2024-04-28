use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};

use crate::{ant::components::Cargo, Abilities, AntAction, Food, Pheromone, Player};

pub struct Random;

impl Random {
    pub fn ant_action(
        _direction: &Vec3,
        _speed: f32,
        _age: f32,
        _cargo: &Cargo,
        _player: Player,
        _abilities: &Abilities,

        _nearby_pheromones: &[(Vec3, Pheromone, Player)],
        _nearby_food: &[(Vec3, Food)],
        _colonies: &[(Vec3, Player)],
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
