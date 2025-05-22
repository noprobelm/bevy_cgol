pub mod behavior;
pub mod cell;
pub mod components;
pub mod resources;

use bevy::prelude::*;
use bevy::utils::HashMap;

pub use behavior::*;
pub use cell::*;
pub use components::*;
pub use resources::*;

pub struct LifePlugin<C, B>
where
    C: Cell,
    B: CellBehavior,
{
    pub cell: C,
    pub behavior: B,
}

impl<C, B> Default for LifePlugin<C, B>
where
    C: Cell,
    B: CellBehavior,
{
    fn default() -> Self {
        LifePlugin {
            cell: C::default(),
            behavior: B::default(),
        }
    }
}

impl<C, B> Plugin for LifePlugin<C, B>
where
    C: Cell,
    B: CellBehavior,
{
    fn build(&self, app: &mut App) {
        app.init_resource::<ElapsedSteps>()
            .init_resource::<StepTimer>()
            .add_systems(Update, new_step::<C, B>)
            .add_systems(Update, color_cells.after(step));
    }
}

pub fn new_step<C, B>(
    mut cell_query: Query<(Entity, &C, &mut B, &Coordinates)>,
    mut step_timer: ResMut<StepTimer>,
    time: Res<Time>,
) where
    C: Cell,
    B: CellBehavior,
{
    step_timer.0.tick(time.delta());
    if !step_timer.0.finished() {
        return;
    }
    let map: HashMap<IVec2, CellState> = cell_query
        .iter()
        .map(|(_, _, state, coordinates)| (coordinates.0, state.state()))
        .collect();
    cell_query
        .par_iter_mut()
        .for_each(|(_, cell, mut state, coordinates)| {
            let neighbor_states: Vec<&CellState> = cell
                .neighbors()
                .iter()
                .filter_map(|relative_coordinates| {
                    let neighbor_coordinates = coordinates.0 + relative_coordinates;
                    map.get(&neighbor_coordinates)
                })
                .collect();
            state.change_state(&neighbor_states); // Pass as a slice of references
        });
}
pub fn step(
    mut elapsed_steps: ResMut<ElapsedSteps>,
    mut cell_query: Query<(Entity, &MooreCell, &mut CellState, &Coordinates)>,
    mut step_timer: ResMut<StepTimer>,
    time: Res<Time>,
    // tree: Res<CellTree>,
) {
    step_timer.0.tick(time.delta());
    if !step_timer.0.finished() {
        return;
    }

    let map: HashMap<Coordinates, CellState> = cell_query
        .iter()
        .map(|(_, _, state, coordinates)| (*coordinates, *state))
        .collect();
    cell_query
        .par_iter_mut()
        .for_each(|(_, _, mut state, coordinates)| {
            let mut num_living_neighbors = 0;
            MooreNeighborhood::iter_neighbors(*coordinates).for_each(|neighbor_coordinates| {
                if let Some(neighbor_state) = map.get(&neighbor_coordinates) {
                    if neighbor_state == &CellState::Alive {
                        num_living_neighbors += 1;
                    }
                }
            });
            match state.as_ref() {
                CellState::Alive => {
                    if num_living_neighbors < 2 {
                        state.transition();
                    } else if num_living_neighbors > 3 {
                        state.transition()
                    }
                }
                CellState::Dead => {
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
    cells_query: Query<(Entity, &MooreCell, &CellState, &Coordinates)>,
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

fn color_cells(mut cells_query: Query<(&MooreCell, &mut Sprite, &CellState)>) {
    cells_query
        .par_iter_mut()
        .for_each(|(_, mut sprite, state)| match state {
            CellState::Alive => {
                sprite.color = Color::Srgba(Srgba::rgba_u8(246, 174, 45, 255));
            }
            CellState::Dead => {
                sprite.color = Color::Srgba(Srgba::NONE);
            }
        });
}
