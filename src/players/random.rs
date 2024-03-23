use std::f32::consts::PI;

use bevy::math::{Quat, Vec3};

use crate::AntAction;

pub struct Random;

impl Random {
    pub fn ant_action(_velocity: &Vec3) -> AntAction {
        let da = (rand::random::<f32>() - 0.5) * PI / 12.;
        AntAction::Rotate(Quat::from_rotation_z(da))
    }
}
