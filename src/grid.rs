use bevy::prelude::{Component, Query};

pub struct Grid {
    pub width: usize,
    pub height: usize,
}

#[derive(Component)]
pub struct GridCell {}

pub fn draw_cell_coordinates(mut query: Query<(&mut GridCell,)>) {}
