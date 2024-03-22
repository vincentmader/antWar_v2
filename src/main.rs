use bevy::{prelude::*, window::PrimaryWindow};
use std::f32::consts::PI;

const BACKGROUND_COLOR: Color = Color::rgba(1.0, 1.0, 1.0, 1.0);

#[derive(Component, Clone)]
struct Direction(Vec3);
impl Direction {
    fn get_angle(&self) -> f32 {
        self.0.y.atan2(self.0.x)
    }
}

fn main() {
    App::new()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .add_systems(Update, confine_player_movement)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let _window = window_query.get_single().unwrap();

    let sprite_size = Vec2 { x: 40.0, y: 40.0 };
    let initial_position = Vec3::default();
    let initial_rotation = -0.25 * std::f32::consts::PI;
    let initial_direction = Direction(Vec3 {
        x: 300.0,
        y: 300.0,
        z: 0.0,
    });

    commands.spawn(Camera2dBundle::default());
    for _ in 0..100 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("img/ant.png"),
                transform: Transform {
                    translation: initial_position,
                    rotation: Quat::from_rotation_z(initial_rotation),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(sprite_size),
                    ..default()
                },
                ..default()
            },
            initial_direction.clone(),
        ));
    }
}

fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut direction, mut transform) in &mut sprite_position {
        transform.translation.x += direction.0.x * time.delta_seconds();
        transform.translation.y += direction.0.y * time.delta_seconds();
        transform.translation.z += direction.0.z * time.delta_seconds();

        let l = direction.0.length();
        let c = direction.get_angle();
        let a = c + (rand::random::<f32>() - 0.5) * PI / 12.0;
        direction.0.x = l * a.cos();
        direction.0.y = l * a.sin();

        transform.rotate(Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), a - c));
    }
}

fn confine_player_movement(
    mut player_query: Query<(&mut Direction, &mut Transform)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for (mut direction, mut transform) in &mut player_query {
        let window = window_query.get_single().unwrap();

        let x = transform.translation.x;
        let y = transform.translation.y;
        let a = direction.get_angle();

        if x < -window.width() / 2.0 + 20.0 {
            let i = -PI / 2.0 - a;
            direction.0.x = -direction.0.x;
            transform.rotate(Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), 2.0 * i));
        } else if x > window.width() / 2.0 - 20.0 {
            let i = PI / 2.0 - a;
            direction.0.x = -direction.0.x;
            transform.rotate(Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), 2.0 * i));
        }
        if y < -window.height() / 2.0 + 20.0 {
            let i = a - PI;
            direction.0.y = -direction.0.y;
            transform.rotate(Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), -2.0 * i));
        } else if y > window.height() / 2.0 - 20.0 {
            let i = a;
            direction.0.y = -direction.0.y;
            transform.rotate(Quat::from_axis_angle(Vec3::new(0.0, 0.0, 1.0), -2.0 * i));
        }
    }
}
