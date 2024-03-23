use bevy::{
    ecs::component::Component,
    math::{Quat, Vec3},
};

#[derive(Component, Clone)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub fn bounce_rotate(&mut self, normal: Vec3) -> Quat {
        let new_velocity = self.0 - 2. * normal * self.0;

        let rotation = Quat::from_rotation_arc(self.0.normalize(), new_velocity.normalize());
        *self = Velocity(new_velocity);

        rotation
    }

    pub fn rotate(&mut self, rotation: Quat) {
        self.0 = rotation * self.0;
    }
}
