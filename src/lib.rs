#![feature(let_chains)]

// ====== Plugins
mod ant;
mod colony;
mod common;
mod food;
mod io;
mod pheromone;
pub mod world;

mod player;
mod players;

pub use ant::{
    components::{Abilities, Ant, Direction, Speed},
    AntAction, AntPlugin,
};
pub use colony::{Colony, ColonyBundle, ColonyPlugin};
pub use common::{
    components::{Age, Amount},
    CommonPlugin,
};
pub use food::{components::Food, FoodPlugin};
pub use io::IOPlugin;
pub use pheromone::{Pheromone, PheromoneBundle, PheromonePlugin};
pub use world::{
    resources::{ColorHandles, PheromoneMeshHandle, WorldSize},
    WorldPlugin,
};

pub use player::Player;
