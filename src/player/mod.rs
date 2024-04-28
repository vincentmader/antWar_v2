use bevy::{ecs::component::Component, math::Vec3};

use crate::{
    ant::components::Cargo,
    players::{Natural, Random},
    Abilities, AntAction, Food, Pheromone,
};

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Random,
    Natural,
}

impl Player {
    pub fn ant_action(
        &self,
        direction: &Vec3,
        speed: f32,
        age: f32,
        cargo: &Cargo,
        player: Player,
        abilities: &Abilities,

        pheromones: &[(Vec3, Pheromone, Player)],
        food: &[(Vec3, Food)],
        colonies: &[(Vec3, Player)],
    ) -> AntAction {
        match self {
            Player::Random => Random::ant_action(
                direction, speed, age, cargo, player, abilities, pheromones, food, colonies,
            ),
            Player::Natural => Natural::ant_action(
                direction, speed, age, cargo, player, abilities, pheromones, food, colonies,
            ),
        }
    }
}
