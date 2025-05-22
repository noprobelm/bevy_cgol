use crate::behavior::CellBehavior;
use crate::cell::Cell;
use bevy::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Component, Reflect)]
pub struct Coordinates(pub IVec2);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Component, Reflect)]
pub struct MooreCell;

impl Cell for MooreCell {
    fn neighbors(&self) -> &'static [IVec2] {
        const MOORE_NEIGHBORS: [IVec2; 8] = [
            IVec2::new(0, 1),
            IVec2::new(1, 1),
            IVec2::new(1, 0),
            IVec2::new(1, -1),
            IVec2::new(0, -1),
            IVec2::new(-1, -1),
            IVec2::new(-1, 0),
            IVec2::new(-1, 1),
        ];
        &MOORE_NEIGHBORS
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Component, Reflect)]
pub struct ConwayCell {
    pub state: CellState,
    pub birth_rules: Vec<u8>,
    pub survival_rules: Vec<u8>,
}

impl Default for ConwayCell {
    fn default() -> Self {
        ConwayCell {
            state: CellState::Dead,
            birth_rules: vec![3],
            survival_rules: vec![2, 3],
        }
    }
}

impl Cell for ConwayCell {
    fn neighbors(&self) -> &'static [IVec2] {
        const MOORE_NEIGHBORS: [IVec2; 8] = [
            IVec2::new(0, 1),
            IVec2::new(1, 1),
            IVec2::new(1, 0),
            IVec2::new(1, -1),
            IVec2::new(0, -1),
            IVec2::new(-1, -1),
            IVec2::new(-1, 0),
            IVec2::new(-1, 1),
        ];
        &MOORE_NEIGHBORS
    }
}

impl CellBehavior for ConwayCell {
    fn state(&self) -> CellState {
        self.state
    }
    fn change_state(&mut self, neighbors: &[&CellState]) {
        let num_living_neighbors = neighbors.iter().fold(0, |acc, state| {
            if *state == &CellState::Alive {
                acc + 1
            } else {
                acc
            }
        });
        match self.state() {
            CellState::Alive => {
                if !self.survival_rules.contains(&num_living_neighbors) {
                    self.state.transition()
                }
            }
            CellState::Dead => {
                if self.birth_rules.contains(&num_living_neighbors) {
                    self.state.transition()
                }
            }
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Default, Component, Reflect)]
pub enum CellState {
    Dead,
    #[default]
    Alive,
}

impl CellState {
    pub fn transition(&mut self) {
        *self = match self {
            CellState::Alive => CellState::Dead,
            CellState::Dead => CellState::Alive,
        }
    }
}
