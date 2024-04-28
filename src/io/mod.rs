use bevy::{
    app::{App, Plugin, Update},
    ecs::{
        event::EventReader,
        query::With,
        system::{Query, Res},
    },
    input::{
        keyboard::{KeyCode, KeyboardInput},
        mouse::{MouseScrollUnit, MouseWheel},
        ButtonState,
    },
    math::Vec3,
    render::camera::{Camera, OrthographicProjection},
    time::Time,
    transform::components::Transform,
    window::{PrimaryWindow, Window},
};

pub struct IOPlugin;

impl Plugin for IOPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_keyboard_events)
            .add_systems(Update, handle_mouse_events);
    }
}

fn handle_mouse_events(
    mut mouse_input_events: EventReader<MouseWheel>,
    mut projection_query: Query<(&mut OrthographicProjection, &mut Transform), With<Camera>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for event in mouse_input_events.read() {
        let scroll_factor = match event.unit {
            MouseScrollUnit::Line => 50.0 * event.y,
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
