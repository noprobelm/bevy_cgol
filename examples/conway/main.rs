use bevy::{input::mouse::MouseWheel, prelude::*, window::WindowMode};
use rand::Rng;

use bevy_cgol::{CellState, ConwayState, Coordinates, LifePlugin, MooreCell};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Conway's Game of Life".to_string(),
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            }),
            LifePlugin::<MooreCell, ConwayState> {
                cell: MooreCell,
                behavior: ConwayState::default(),
            },
        ))
        .insert_resource(ClearColor(Color::Srgba(Srgba::rgba_u8(49, 87, 113, 255))))
        .add_systems(Startup, (setup_camera, setup_world))
        .add_systems(Update, (pan_camera, ev_mouse_wheel))
        .run();
}

fn setup_world(mut commands: Commands) {
    let mut rng = rand::rng();
    for y in -200..200 {
        for x in -200..200 {
            let state = if rng.random_bool(0.1) {
                CellState::Alive
            } else {
                CellState::Dead
            };
            commands.spawn((
                Sprite {
                    color: Color::srgba(255., 0., 0., 0.),
                    ..default()
                },
                MooreCell,
                ConwayState {
                    state,
                    birth_rules: vec![3],
                    survival_rules: vec![2, 3],
                },
                Coordinates(IVec2::new(x, y)),
                Transform::from_translation(Vec3::new(x as f32, y as f32, 0.)),
            ));
        }
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            near: -1000.0,
            scale: 0.11,
            ..OrthographicProjection::default_2d()
        },
        MainCamera,
    ));
}

pub fn pan_camera(
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut transform = camera_query.single_mut();

    if keys.pressed(KeyCode::KeyW) {
        transform.translation.y += 2.;
    }

    if keys.pressed(KeyCode::KeyA) {
        transform.translation.x -= 2.;
    }

    if keys.pressed(KeyCode::KeyS) {
        transform.translation.y -= 2.;
    }

    if keys.pressed(KeyCode::KeyD) {
        transform.translation.x += 2.;
    }
}

pub fn ev_mouse_wheel(
    mut ev_scroll: EventReader<MouseWheel>,
    mut camera_query: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    if !ev_scroll.is_empty() {
        let mut projection = camera_query.single_mut();
        ev_scroll.read().for_each(|ev| {
            let zoom = -(ev.y / 100.);
            if projection.scale + zoom > 0.01 {
                projection.scale += zoom;
            }
        });
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Component, Reflect)]
pub struct MainCamera;
