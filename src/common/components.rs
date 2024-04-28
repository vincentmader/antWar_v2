use bevy::{
    ecs::component::Component,
    prelude::{Deref, DerefMut},
};

#[derive(Component, Deref, DerefMut)]
pub struct Age(pub f32);

#[derive(Component, Deref, DerefMut, Copy, Clone)]
pub struct Amount(pub f32);
