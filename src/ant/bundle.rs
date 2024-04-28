use bevy::{ecs::bundle::Bundle, sprite::SpriteBundle};

use crate::{Age, Player};

use super::components::{Abilities, Ant, Cargo, Direction, Speed};

#[derive(Bundle)]
pub struct AntBundle {
    pub _type: Ant,

    pub age: Age,
    pub speed: Speed,
    // TODO: direction really necessary?? => use transform.rotation instead??
    pub direction: Direction,
    pub cargo: Cargo,

    // TODO: add home colony
    // pub colony: Colony,
    pub player: Player,

    pub abilities: Abilities,

    pub sprite: SpriteBundle,
}
