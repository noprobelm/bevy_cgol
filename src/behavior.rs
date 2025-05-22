use crate::CellState;
use bevy::prelude::*;

pub trait CellBehavior: Clone + Default + Component + Sync + Send {
    fn state(&self) -> CellState;
    fn change_state(&mut self, neighbors: &[&CellState]);
}
