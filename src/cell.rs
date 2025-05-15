use bevy::prelude::*;

pub trait Cell: Component {
    fn neighbors(&self) -> Vec<Coordinates>;
}
