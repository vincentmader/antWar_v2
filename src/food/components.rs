use bevy::ecs::component::Component;

#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Food {
    Fungi,
}
