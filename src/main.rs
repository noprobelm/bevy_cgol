use bevy::utils::Duration;
use bevy::utils::HashMap;
use bevy::{input::mouse::MouseWheel, prelude::*, window::WindowMode};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Conway's Game of Life".to_string(),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }),))
        .init_resource::<ElapsedSteps>()
        .insert_resource(ClearColor(Color::Srgba(Srgba::rgba_u8(49, 87, 113, 255))))
        .init_resource::<StepTimer>()
        .add_systems(Startup, (setup_camera, setup_world))
        .add_systems(Update, (pan_camera, ev_mouse_wheel))
        .add_systems(Update, step)
        // .add_systems(Update, cleanup_orphaned_cells.after(step))
        .add_systems(Update, color_cells.after(step))
        .run();
}

fn step(
    mut elapsed_steps: ResMut<ElapsedSteps>,
    mut cell_query: Query<(Entity, &Cell, &mut State, &Coordinates)>,
    mut step_timer: ResMut<StepTimer>,
    time: Res<Time>,
    // tree: Res<CellTree>,
) {
    step_timer.0.tick(time.delta());
    if !step_timer.0.finished() {
        return;
    }

    let map: HashMap<Coordinates, State> = cell_query
        .iter()
        .map(|(_, _, state, coordinates)| (*coordinates, *state))
        .collect();
    cell_query
        .par_iter_mut()
        .for_each(|(_, _, mut state, coordinates)| {
            let mut num_living_neighbors = 0;
            MooreNeighborhood::iter_neighbors(*coordinates).for_each(|neighbor_coordinates| {
                if let Some(neighbor_state) = map.get(&neighbor_coordinates) {
                    if neighbor_state == &State::Alive {
                        num_living_neighbors += 1;
                    }
                }
            });
            match state.as_ref() {
                State::Alive => {
                    if num_living_neighbors < 2 {
                        state.transition();
                    } else if num_living_neighbors > 3 {
                        state.transition()
                    }
                }
                State::Dead => {
                    if num_living_neighbors == 3 {
                        state.transition();
                    }
                }
            };
        });
    elapsed_steps.0 += 1;
}

#[allow(dead_code)]
fn cleanup_orphaned_cells(
    par_commands: ParallelCommands,
    cells_query: Query<(Entity, &Cell, &State, &Coordinates)>,
) {
    cells_query
        .par_iter()
        .for_each(|(entity, _, _state, _coordinates)| {
            let num_living_neighbors = 0;
            if num_living_neighbors == 0 {
                par_commands.command_scope(|mut commands| {
                    commands.entity(entity).despawn();
                });
            }
        });
}

fn color_cells(mut cells_query: Query<(&Cell, &mut Sprite, &State)>) {
    cells_query
        .par_iter_mut()
        .for_each(|(_, mut sprite, state)| match state {
            State::Alive => {
                sprite.color = Color::Srgba(Srgba::rgba_u8(246, 174, 45, 255));
            }
            State::Dead => {
                sprite.color = Color::Srgba(Srgba::NONE);
            }
        });
}

fn setup_world(mut commands: Commands) {
    let mut rng = rand::rng();
    for y in -200..200 {
        for x in -200..200 {
            let state = if rng.random_bool(0.1) {
                State::Alive
            } else {
                State::Dead
            };
            commands.spawn((
                Sprite {
                    color: Color::srgba(255., 0., 0., 0.),
                    ..default()
                },
                Cell,
                state,
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

#[derive(Clone, Eq, PartialEq, Debug, Default, Resource, Reflect)]
pub struct CellMap {
    map: HashMap<Coordinates, Alive>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Component, Reflect)]
pub struct Coordinates(pub IVec2);

pub enum MooreNeighborhood {
    North,
    Northeast,
    East,
    Southeast,
    South,
    Southwest,
    West,
    Northwest,
}

impl MooreNeighborhood {
    pub fn relative_position(&self, coordinates: Coordinates) -> Coordinates {
        let delta = match self {
            MooreNeighborhood::North => IVec2::new(0, 1),
            MooreNeighborhood::Northeast => IVec2::new(1, 1),
            MooreNeighborhood::East => IVec2::new(1, 0),
            MooreNeighborhood::Southeast => IVec2::new(1, -1),
            MooreNeighborhood::South => IVec2::new(0, -1),
            MooreNeighborhood::Southwest => IVec2::new(-1, -1),
            MooreNeighborhood::West => IVec2::new(-1, 0),
            MooreNeighborhood::Northwest => IVec2::new(-1, 1),
        };
        Coordinates(coordinates.0 + delta)
    }

    pub fn iter_neighbors(coordinates: Coordinates) -> impl Iterator<Item = Coordinates> {
        [
            IVec2::new(0, 1),
            IVec2::new(1, 1),
            IVec2::new(1, 0),
            IVec2::new(1, -1),
            IVec2::new(0, -1),
            IVec2::new(-1, -1),
            IVec2::new(-1, 0),
            IVec2::new(-1, 1),
        ]
        .into_iter()
        .map(move |delta| Coordinates(coordinates.0 + delta))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Component, Reflect)]
pub struct MainCamera;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Component, Reflect)]
pub struct Cell;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Component, Reflect)]
pub enum State {
    Dead,
    #[default]
    Alive,
}

impl State {
    fn transition(&mut self) {
        *self = match self {
            State::Alive => State::Dead,
            State::Dead => State::Alive,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Alive;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Component, Reflect)]
#[component(storage = "SparseSet")]
pub struct Dead;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, Resource, Reflect)]
pub struct ElapsedSteps(u64);

#[derive(Clone, Eq, PartialEq, Debug, Resource, Reflect)]
pub struct StepTimer(Timer);

impl Default for StepTimer {
    fn default() -> Self {
        StepTimer(Timer::new(Duration::from_millis(100), TimerMode::Repeating))
    }
}
