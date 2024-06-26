use std::f32::consts::PI;

use bevy::{
    asset::Handle,
    core::TaskPoolThreadAssignmentPolicy,
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseScrollUnit, MouseWheel},
        ButtonState,
    },
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    tasks::available_parallelism,
    window::PrimaryWindow,
};

mod velocity;
use velocity::Direction;
use velocity::Speed;

mod players;

const BACKGROUND_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);

#[derive(Component, Clone)]
enum Player {
    Random,
    Natural,
}

#[derive(Component, Copy, Clone)]
enum Pheromone {
    Green,
    Red,
}

#[derive(Component, Copy, Clone)]
enum Food {
    Fungi(u16),
}
impl Food {
    pub fn get_amount(&self) -> u16 {
        match self {
            Self::Fungi(amount) => *amount,
        }
    }
    pub fn decrease_amount(&mut self, amount: u16) {
        *self = match self {
            Self::Fungi(initial) => Self::Fungi(*initial - amount),
        }
    }
}

#[derive(Component)]
struct Amount(f32);

#[derive(Component)]
struct Age(f32);

#[derive(Resource, Default)]
struct WorldSize(f32, f32);

#[derive(Component)]
struct VisionDistance(f32);

#[derive(Component)]
enum Inventory {
    Food(u16),
}

#[derive(Resource)]
struct ColorHandles {
    red: Handle<ColorMaterial>,
    green: Handle<ColorMaterial>,
}

enum AntAction {
    Rotate(Quat),
    DropPheromone(Pheromone),
    Accelerate(f32),
    PickUpFood(u16),
    DropFood(u16),
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(TaskPoolPlugin {
            task_pool_options: TaskPoolOptions {
                compute: TaskPoolThreadAssignmentPolicy {
                    min_threads: available_parallelism(),
                    max_threads: std::usize::MAX, // unlimited max threads
                    percent: 1.0,                 // this value is irrelevant in this case
                },
                // keep the defaults for everything else
                ..default()
            },
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .init_resource::<WorldSize>()
        .add_systems(Startup, setup)
        .add_systems(Update, (ant_action, confine_ant_movement.after(ant_action)))
        .add_systems(Update, handle_keyboard_events)
        .add_systems(Update, handle_mouse_events)
        .add_systems(Update, increase_age)
        .add_systems(Update, pheromone_removal)
        .add_systems(Update, food_removal)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut world_size: ResMut<WorldSize>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let window = window_query.get_single().unwrap();
    world_size.0 = window.width();
    world_size.1 = window.height();

    let sprite_size = Vec2 { x: 1.0, y: 1.0 };
    let initial_position = Vec3::new(0.0, 0.0, 0.0);

    let red = materials.add(Color::rgba(1.0, 0.0, 0.0, 1.0));
    let green = materials.add(Color::rgba(0.0, 1.0, 0.0, 1.0));
    commands.insert_resource(ColorHandles { red, green });
    commands.spawn(Camera2dBundle::default());

    for _ in 0..500 {
        let ant_size = 30.0 + 15.0 * rand::random::<f32>();
        let initial_speed = 200.0 + 100. * rand::random::<f32>();
        let initial_rotation = 2.0 * PI * rand::random::<f32>();
        let initial_direction = Direction(Vec3 {
            x: initial_rotation.cos(),
            y: initial_rotation.sin(),
            z: 0.0,
        });

        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("img/ant.png"),
                transform: Transform {
                    translation: initial_position,
                    rotation: Quat::from_rotation_z(initial_rotation - PI / 2.0),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(sprite_size * ant_size),
                    ..default()
                },
                ..default()
            },
            initial_direction.clone(),
            Speed(initial_speed),
            Age(0.0),
            Player::Natural,
            VisionDistance(50.0),
            Inventory::Food(0),
        ));
    }

    for _ in 0..5 {
        let initial_position = Vec3 {
            x: (rand::random::<f32>() - 0.5) * world_size.0,
            y: (rand::random::<f32>() - 0.5) * world_size.1,
            z: -1.0,
        };
        commands.spawn((
            Food::Fungi(200), // TODO Duplicate definition of amount?
            Amount(10.0),
            SpriteBundle {
                texture: asset_server.load("img/fungus.png"),
                transform: Transform {
                    translation: initial_position,
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(sprite_size * 30.0),
                    ..default()
                },
                ..default()
            },
        ));
    }
}

fn handle_mouse_events(
    mut mouse_input_events: EventReader<MouseWheel>,
    mut projection_query: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for event in mouse_input_events.read() {
        let scroll_factor = match event.unit {
            MouseScrollUnit::Line => 5.0 * event.y,
            MouseScrollUnit::Pixel => event.y,
        };
        let (mut projection, mut transform) = projection_query.get_single_mut().unwrap();
        projection.scale *= 1.0 + scroll_factor / 1000.0;
        if projection.scale >= 1.0 {
            projection.scale = 1.0;
        }
        println!("{}", scroll_factor);

        let window = window_query.get_single().unwrap();
        let scale = projection.scale;

        if (1.0 - scale) * window.width() / 2.0 - transform.translation.x < 0.0 {
            transform.translation.x = (1.0 - scale) * window.width() / 2.0;
        } else if (1.0 - scale) * window.width() / 2.0 + transform.translation.x < 0.0 {
            transform.translation.x = -(1.0 - scale) * window.width() / 2.0;
        } else if (1.0 - scale) * window.height() / 2.0 - transform.translation.y < 0.0 {
            transform.translation.y = (1.0 - scale) * window.height() / 2.0;
        } else if (1.0 - scale) * window.height() / 2.0 + transform.translation.y < 0.0 {
            transform.translation.y = -(1.0 - scale) * window.height() / 2.0;
        }
    }
}
fn handle_keyboard_events(
    mut keyboard_input_events: EventReader<KeyboardInput>,
    mut projection_query: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let (projection, mut transform) = projection_query.get_single_mut().unwrap();
    let scale = projection.scale;
    let pan_speed = time.delta_seconds() * scale * 5000.0;
    for event in keyboard_input_events.read() {
        if event.state == ButtonState::Pressed {
            match event.key_code {
                KeyCode::ArrowRight | KeyCode::KeyL => {
                    if (1.0 - scale) * window.width() / 2.0 - transform.translation.x > 0.0 {
                        transform.translation += pan_speed * Vec3::X;
                    }
                }
                KeyCode::ArrowLeft | KeyCode::KeyH => {
                    if (1.0 - scale) * window.width() / 2.0 + transform.translation.x > 0.0 {
                        transform.translation -= pan_speed * Vec3::X;
                    }
                }
                KeyCode::ArrowUp | KeyCode::KeyK => {
                    if (1.0 - scale) * window.height() / 2.0 - transform.translation.y > 0.0 {
                        transform.translation += pan_speed * Vec3::Y;
                    }
                }
                KeyCode::ArrowDown | KeyCode::KeyJ => {
                    if (1.0 - scale) * window.height() / 2.0 + transform.translation.y > 0.0 {
                        transform.translation -= pan_speed * Vec3::Y;
                    }
                }
                _ => {}
            }
        }
    }
}

fn ant_action(
    mut commands: Commands,
    time: Res<Time>,
    mut ant_data: Query<
        (
            &mut Direction,
            &mut Speed,
            &mut Transform,
            &Player,
            &VisionDistance,
        ),
        (Without<Food>, Without<Pheromone>),
    >,
    mut meshes: ResMut<Assets<Mesh>>,
    color_handles: Res<ColorHandles>,
    pheromone_query: Query<(&Pheromone, &Transform, &Player)>,
    mut food_query: Query<(&mut Food, &Transform)>,
) {
    ant_data.iter_mut().for_each(
        |(mut ant_direction, mut ant_speed, mut ant_transform, ant_player, vision_distance)| {
            let nearby_pheromones: Vec<_> = pheromone_query
                .iter()
                .filter_map(|(pheromone, pheromone_transform, pheromone_player)| {
                    let relative = pheromone_transform.translation - ant_transform.translation;
                    let distance = relative.length();
                    if distance < vision_distance.0
                        && ant_direction.0.angle_between(relative) < PI / 2.0
                    {
                        Some((*pheromone, ant_player.clone(), relative))
                    } else {
                        None
                    }
                })
                .collect();

            let mut nearby_food_ref: Vec<_> = food_query
                .iter_mut()
                .filter_map(|(food, food_transform)| {
                    let relative = food_transform.translation - ant_transform.translation;
                    let distance = relative.length();
                    if distance < vision_distance.0
                        && ant_direction.0.angle_between(relative) < PI / 2.0
                    {
                        Some((food, relative))
                    } else {
                        None
                    }
                })
                .collect();

            let nearby_food: Vec<_> = nearby_food_ref
                .iter()
                .filter_map(|(food, relative)| {
                    let distance = relative.length();
                    if distance < vision_distance.0
                        && ant_direction.0.angle_between(*relative) < PI / 2.0
                    {
                        Some(((*food).clone(), *relative))
                    } else {
                        None
                    }
                })
                .collect();

            // TODO: verify action??

            let action = match ant_player {
                Player::Random => players::Random::ant_action(
                    &ant_direction.0,
                    &nearby_pheromones,
                    &nearby_food,
                    ant_speed.0,
                ),
                Player::Natural => players::Natural::ant_action(
                    ant_direction.0,
                    &nearby_pheromones,
                    &nearby_food,
                    ant_speed.0,
                ),
            };

            match action {
                AntAction::PickUpFood(amount) => {
                    nearby_food_ref.iter_mut().for_each(|(food, relative)| {
                        let amount = amount.min(food.get_amount());
                        if relative.length() < 10.0 && amount > 0 {
                            println!("{}", food.get_amount());
                            food.decrease_amount(amount);
                            *ant_speed = Speed(0.0);
                        }
                    })
                }
                AntAction::DropFood(amount) => {}
                AntAction::Rotate(rotation) => {
                    ant_transform.rotate(rotation);
                    ant_direction.rotate(rotation);
                }
                AntAction::Accelerate(speed) => {
                    ant_speed.0 = speed;
                }
                AntAction::DropPheromone(pheromone) => {
                    let shape = Mesh2dHandle(meshes.add(Circle { radius: 15.0 }));
                    let material = match pheromone {
                        Pheromone::Red => color_handles.red.clone(),
                        Pheromone::Green => color_handles.green.clone(),
                    };

                    commands.spawn((
                        pheromone,
                        ant_player.clone(),
                        Age(0.0),
                        MaterialMesh2dBundle {
                            mesh: shape,
                            material,
                            transform: Transform {
                                translation: ant_transform.translation - Vec3::Z,
                                scale: Vec3::new(0.1, 0.1, 0.1),
                                ..default()
                            },
                            ..default()
                        },
                    ));
                }
            }

            ant_transform.translation += ant_direction.0 * ant_speed.0 * time.delta_seconds();
        },
    );
}

fn increase_age(time: Res<Time>, mut query: Query<&mut Age>) {
    query.par_iter_mut().for_each(|mut age| {
        age.0 += time.delta_seconds();
    });
}

fn food_removal(mut commands: Commands, query: Query<(Entity, &Food)>) {
    // todo: Add parallelism.
    query
        .into_iter()
        // .filter_map(|(entity, age)| if age.0 > 3. { Some(entity) } else { None })
        .filter_map(|(entity, food)| {
            if food.get_amount() == 0 {
                Some(entity)
            } else {
                None
            }
        })
        .for_each(|entity| {
            commands.entity(entity).despawn();
        });
}

fn pheromone_removal(mut commands: Commands, query: Query<(Entity, &Age), With<Pheromone>>) {
    // todo: Add parallelism.
    query
        .into_iter()
        // .filter_map(|(entity, age)| if age.0 > 3. { Some(entity) } else { None })
        .filter_map(|(entity, age)| if age.0 > 5. { Some(entity) } else { None })
        .for_each(|entity| {
            commands.entity(entity).despawn();
        });
}

fn confine_ant_movement(
    mut ant_query: Query<(&mut Direction, &Speed, &mut Transform)>,
    world_size: ResMut<WorldSize>,
) {
    let width = world_size.0;
    let height = world_size.1;

    ant_query
        .par_iter_mut()
        .for_each(|(mut ant_direction, ant_speed, mut ant_transform)| {
            let x = ant_transform.translation.x;
            let y = ant_transform.translation.y;

            if x < -width / 2.0 + 20.0 {
                let rot = ant_direction.bounce_rotate(Vec3::X);
                ant_transform.rotate(rot);

                ant_transform.translation.x = -ant_transform.translation.x - width + 40.;
            } else if x > width / 2.0 - 20.0 {
                let rot = ant_direction.bounce_rotate(Vec3::X);
                ant_transform.rotate(rot);
                ant_transform.translation.x = -ant_transform.translation.x + width - 40.;
            }
            if y < -height / 2.0 + 20.0 {
                let rot = ant_direction.bounce_rotate(Vec3::Y);
                ant_transform.rotate(rot);
                ant_transform.translation.y = -ant_transform.translation.y - height + 40.;
            } else if y > height / 2.0 - 20.0 {
                let rot = ant_direction.bounce_rotate(Vec3::Y);
                ant_transform.rotate(rot);
                ant_transform.translation.y = -ant_transform.translation.y + height - 40.;
            }
        });
}
