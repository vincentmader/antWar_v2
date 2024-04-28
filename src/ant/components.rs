use bevy::{
    ecs::component::Component,
    math::{Quat, Vec3},
    prelude::{Deref, DerefMut},
};

use crate::{Amount, Food};

use super::abilities::Vision;

#[derive(Component)]
pub struct Ant;

#[derive(Component)]
pub struct Abilities {
    pub vision: Vision,
    // max speed
    pub speed: f32,
}

// #[derive(Component, Deref, DerefMut)]
// pub struct Age(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct Speed(pub f32);

#[derive(Component, Deref, DerefMut)]
pub struct Direction(pub Vec3);

#[derive(Component)]
pub enum Cargo {
    Food { typ: Food, amound: Amount },
    Empty,
}

impl Direction {
    pub fn bounce_rotation(&self, normal: Vec3) -> Quat {
        let new_direction = self.0 - 2. * normal * self.0;

        // OPTIMIZE: assume normalization of self??
        Quat::from_rotation_arc(self.0.normalize(), new_direction.normalize())
    }

    pub fn rotate(&mut self, rotation: Quat) {
        self.0 = rotation * self.0;
    }
}
