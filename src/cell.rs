use bevy::prelude::*;

pub trait Cell: Clone + Default + Component + Sync + Send {
    fn neighbors(&self) -> &'static [IVec2];
}
