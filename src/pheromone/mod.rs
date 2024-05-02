use bevy::{
    app::{Plugin, Startup, Update},
    asset::Handle,
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        query::With,
        schedule::IntoSystemConfigs,
        system::{Commands, ParallelCommands, Query, Res},
    },
    math::Vec3,
    prelude::default,
    render::mesh::Mesh,
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    transform::components::Transform,
};

use crate::{Age, Player, WorldSize};

pub struct PheromonePlugin;

impl Plugin for PheromonePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup, setup.after(crate::world::setup))
            .add_systems(Update, pheromone_removal);
    }
}

fn setup(mut commands: Commands, world_size: Res<WorldSize>) {
    // commands.insert_resource(PheromoneList {
    //     data: RwLock::new(BTreeMap::new()),
    //     units_per_item: (world_size.0 / 10., world_size.1 / 10.),
    // });
}

#[derive(Bundle)]
pub struct PheromoneBundle {
    pheromone: Pheromone,
    player: Player,
    age: Age,

    material_mesh: MaterialMesh2dBundle<ColorMaterial>,
}

#[derive(Component, Copy, Clone, PartialEq, Eq)]
pub enum Pheromone {
    Green,
    Red,
}

// #[derive(Resource)]
// pub struct PheromoneList {
//     data: RwLock<BTreeMap<(u32, u32), Vec<Entity>>>,
//     units_per_item: (f32, f32),
// }

// impl PheromoneList {
//     pub fn coords_to_ids(&self, x: f32, y: f32) -> (u32, u32) {
//         (
//             (x / self.units_per_item.0).floor() as u32,
//             (y / self.units_per_item.1).floor() as u32,
//         )
//     }
//     pub fn insert(&self, (x, y): (f32, f32), entity: Entity) {
//         let (x_unit, y_unit) = self.coords_to_ids(x, y);

//         // BUG: is unwrap save here?
//         let mut map = self.data.write().unwrap();

//         let unit_vec = map.get_mut(&(x_unit, y_unit));
//         match unit_vec {
//             Some(v) => {
//                 v.push(entity);
//             }
//             None => {
//                 map.insert((x_unit, y_unit), vec![entity]);
//             }
//         }
//     }

//     // pub fn par_get(&self, keys: &[(u32, u32)]) -> impl ParallelIterator + 'static {
//     //     // BUG: is unwrap save here?
//     //     let data = self.data.read().unwrap();

//     //     keys.into_par_iter()
//     //         .filter_map(|k| data.get(k))
//     //         .map(|v| v.par_iter())
//     //         .flatten()
//     // }
//     pub fn data_read(&self) -> RwLockReadGuard<'_, BTreeMap<(u32, u32), Vec<Entity>>> {
//         self.data.read().unwrap()
//     }
// }

impl Pheromone {
    pub fn spawn(
        commands: &mut Commands,

        pheromone: Self,
        player: Player,
        translation: Vec3,

        shape: Handle<Mesh>,
        color_material: Handle<ColorMaterial>,
        // pheromone_list: &PheromoneList,
        // world_size: &WorldSize,
    ) {
        let shape = Mesh2dHandle(shape);
        let entity = commands
            .spawn(PheromoneBundle {
                pheromone,
                player,
                age: Age(0.0),
                material_mesh: MaterialMesh2dBundle {
                    mesh: shape,
                    material: color_material,
                    transform: Transform {
                        translation,
                        scale: Vec3::new(0.1, 0.1, 0.1),
                        ..default()
                    },
                    ..default()
                },
            })
            .id();

        // pheromone_list.insert((translation.x, translation.y), entity);
    }
}

fn pheromone_removal(commands: ParallelCommands, query: Query<(Entity, &Age), With<Pheromone>>) {
    query.par_iter().for_each(|(entity, age)| {
        if **age > 3. {
            commands.command_scope(move |mut commands| {
                commands.entity(entity).despawn();
            });
        }
    })
}
