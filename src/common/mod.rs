use bevy::{
    app::{Plugin, Update},
    ecs::system::{Query, Res},
    time::Time,
};

use self::components::Age;

pub mod components;

pub struct CommonPlugin;

impl Plugin for CommonPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, increase_age);
    }
}

fn increase_age(time: Res<Time>, mut query: Query<&mut Age>) {
    query.par_iter_mut().for_each(|mut age| {
        **age += time.delta_seconds();
    });
}
