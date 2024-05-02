use std::collections::HashMap;

use bevy::{
    ecs::component::Component,
    prelude::{Deref, DerefMut},
};

use crate::Food;

#[derive(Component, Deref, DerefMut, Default)]
pub struct FoodStorage(HashMap<Food, u64>);
