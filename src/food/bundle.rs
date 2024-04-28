use bevy::{ecs::bundle::Bundle, sprite::SpriteBundle};

use crate::{Amount, Food};

#[derive(Bundle)]
pub struct FoodBundle {
    pub food: Food,
    pub amount: Amount,

    pub sprite: SpriteBundle,
}
