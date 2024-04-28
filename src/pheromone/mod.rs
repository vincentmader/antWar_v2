use bevy::{
    app::{Plugin, Update},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::With,
        system::{ParallelCommands, Query},
    },
};

use crate::Age;

pub struct PheromonePlugin;

impl Plugin for PheromonePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, pheromone_removal);
    }
}

#[derive(Bundle)]
pub struct PheromoneBundle {
    pub pheromone: Pheromone,
    pub age: Age,
}

#[derive(Component, Copy, Clone, PartialEq, Eq)]
pub enum Pheromone {
    Green,
    Red,
}

fn pheromone_removal(commands: ParallelCommands, query: Query<(Entity, &Age), With<Pheromone>>) {
    query.par_iter().for_each(|(entity, age)| {
        if **age > 3. {
            commands.command_scope(move |mut commands| {
                commands.entity(entity).despawn();
            });
        }
    })
}
