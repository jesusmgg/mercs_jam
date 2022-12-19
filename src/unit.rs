use bevy::prelude::{Bundle, Component, Sprite};

#[derive(Bundle)]
pub struct UnitBundle {
    unit: Unit,
    pub name: Name,
    pub stats: Stats,
    pub sprite: Sprite,
}

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Stats {
    pub hit_points: i32,
}
