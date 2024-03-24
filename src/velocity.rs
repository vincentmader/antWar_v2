use bevy::{
    ecs::component::Component,
    math::{Quat, Vec3},
};

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component, Clone)]
pub struct Direction(pub Vec3);

impl Direction {
    pub fn bounce_rotate(&mut self, normal: Vec3) -> Quat {
        let new_velocity = self.0 - 2. * normal * self.0;

        let rotation = Quat::from_rotation_arc(self.0.normalize(), new_velocity.normalize());
        *self = Direction(new_velocity);

        rotation
    }

    pub fn rotate(&mut self, rotation: Quat) {
        self.0 = rotation * self.0;
    }
}
