use bevy::utils::Duration;
use bevy::{prelude::*, window::WindowMode};
use bevy_spatial::{kdtree::KDTree2, AutomaticUpdate, SpatialAccess, SpatialStructure};
use rayon::prelude::*;

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
            AutomaticUpdate::<Cell>::new()
                .with_spatial_ds(SpatialStructure::KDTree2)
                .with_frequency(Duration::from_millis(200)),
        ))
        .init_resource::<Cells>()
        .add_systems(Startup, setup_camera)
        .add_systems(Update, step)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn step(
    par_commands: ParallelCommands,
    cell_query: Query<(Entity, &Cell, Option<&Alive>, &Coordinates)>,
    tree: Res<CellTree>,
) {
    //     let mut num_alive = 0;

    cell_query
        .par_iter()
        .for_each(|(entity, _, alive, coordinates)| {
            let mut num_living_neighbors = 0;
            tree.within_distance(coordinates.0.as_vec2(), 2.)
                .iter()
                .for_each(|(_, neighbor_entity)| {
                    if let Some(neighbor_entity) = neighbor_entity {
                        if entity == *neighbor_entity {
                            return;
                        }
                        let (_, _, neighbor_alive, _) = cell_query.get(*neighbor_entity).unwrap();
                        if neighbor_alive.is_some() {
                            num_living_neighbors += 1;
                        }
                    };
                });
            if (alive.is_some()) && (num_living_neighbors != 2 || num_living_neighbors != 3) {
                par_commands.command_scope(|mut commands| {
                    commands.entity(entity).remove::<Alive>();
                });
            } else if (alive.is_none()) && (num_living_neighbors == 3) {
                par_commands.command_scope(|mut commands| {
                    commands.entity(entity).insert(Alive);
                });
            }
        });
}

fn cleanup_orphaned_cells(cell_query: Query<(&Cell, &Coordinates)>) {}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Component, Reflect)]
#[reflect(Component)]
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

#[derive(Component)]
pub struct Cell {
    alive: bool,
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Alive;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Dead;

pub type CellTree = KDTree2<Cell>;

#[derive(Clone, Eq, PartialEq, Hash, Debug, Default, Resource, Reflect)]
pub struct Cells {
    cells: Vec<Entity>,
}
