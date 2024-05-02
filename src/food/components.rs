use bevy::ecs::component::Component;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub enum Food {
    Fungi,
}
