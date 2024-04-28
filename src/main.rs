use std::time::Duration;

use bevy::{
    core::TaskPoolThreadAssignmentPolicy,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    tasks::available_parallelism,
};

use ant_war::{
    AntPlugin, ColonyPlugin, CommonPlugin, FoodPlugin, IOPlugin, Pheromone, PheromonePlugin,
    WorldPlugin,
};

const BACKGROUND_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);

#[derive(Component, Deref, DerefMut)]
struct LogTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(TaskPoolPlugin {
            task_pool_options: TaskPoolOptions {
                compute: TaskPoolThreadAssignmentPolicy {
                    min_threads: available_parallelism(),
                    max_threads: usize::MAX, // unlimited max threads
                    percent: 1.0,            // this value is irrelevant in this case
                },
                ..default()
            },
        }))
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(WorldPlugin)
        .add_plugins(CommonPlugin)
        .add_plugins(AntPlugin)
        .add_plugins(IOPlugin)
        .add_plugins(PheromonePlugin)
        .add_plugins(FoodPlugin)
        .add_plugins(ColonyPlugin)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup.after(ant_war::world::setup))
        .add_systems(Update, log_pheromone_number)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(LogTimer(Timer::new(
        Duration::from_secs(5),
        TimerMode::Repeating,
    )));
}

fn log_pheromone_number(
    mut timer: Query<&mut LogTimer>,
    time: Res<Time>,
    pheromone_query: Query<&Pheromone>,
) {
    let mut timer = timer.single_mut();

    timer.tick(time.delta());

    if timer.finished() {
        let phn_num = pheromone_query.iter().count();
        info!("number of pheromones: {phn_num}");
    }
}
