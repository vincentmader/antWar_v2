use std::f32::consts::PI;

use bevy::{
    input::keyboard::KeyboardInput,
    input::mouse::{MouseScrollUnit, MouseWheel},
    input::ButtonState,
    prelude::*,
    window::PrimaryWindow,
};

mod velocity;
use velocity::Velocity;

mod players;

const BACKGROUND_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);

#[derive(Component)]
enum Player {
    Random,
}

enum AntAction {
    Rotate(Quat),
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (ant_action, confine_ant_movement.after(ant_action)))
        .add_systems(Update, handle_keyboard_events)
        .add_systems(Update, handle_mouse_events)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
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
    let (mut projection, mut transform) = projection_query.get_single_mut().unwrap();
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

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let _window = window_query.get_single().unwrap();

    let sprite_size = Vec2 { x: 1.0, y: 1.0 };
    let initial_position = Vec3::default();

    commands.spawn(Camera2dBundle::default());

    for _ in 0..500 {
        let ant_size = 30.0 + 15.0 * rand::random::<f32>();
        let initial_speed = 200.0 + 100. * rand::random::<f32>();
        let initial_rotation = 2.0 * PI * rand::random::<f32>();
        let initial_velocity = Velocity(Vec3 {
            x: initial_speed * initial_rotation.cos(),
            y: initial_speed * initial_rotation.sin(),
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
            initial_velocity.clone(),
            Player::Random,
        ));
    }
}

fn ant_action(time: Res<Time>, mut ant_data: Query<(&mut Velocity, &mut Transform, &Player)>) {
    ant_data
        .par_iter_mut()
        .for_each(|(mut velocity, mut transform, player)| {
            let action = match player {
                Player::Random => players::Random::ant_action(&velocity.0),
            };

            // TODO: verify action??

            match action {
                AntAction::Rotate(rotation) => {
                    transform.rotate(rotation);
                    velocity.rotate(rotation);

                    transform.translation += velocity.0 * time.delta_seconds();
                }
            }
        });
}

fn confine_ant_movement(
    mut ant_query: Query<(&mut Velocity, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    let width = window.width();
    let height = window.height();

    ant_query
        .par_iter_mut()
        .for_each(|(mut velocity, mut transform)| {
            let x = transform.translation.x;
            let y = transform.translation.y;

            if x < -width / 2.0 + 20.0 {
                let rot = velocity.bounce_rotate(Vec3::X);
                transform.rotate(rot);

                transform.translation.x = -transform.translation.x - width + 40.;
            } else if x > width / 2.0 - 20.0 {
                let rot = velocity.bounce_rotate(Vec3::X);
                transform.rotate(rot);
                transform.translation.x = -transform.translation.x + width - 40.;
            }
            if y < -height / 2.0 + 20.0 {
                let rot = velocity.bounce_rotate(Vec3::Y);
                transform.rotate(rot);
                transform.translation.y = -transform.translation.y - height + 40.;
            } else if y > height / 2.0 - 20.0 {
                let rot = velocity.bounce_rotate(Vec3::Y);
                transform.rotate(rot);
                transform.translation.y = -transform.translation.y + height - 40.;
            }
        });
}
