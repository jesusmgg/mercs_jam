use bevy::prelude::{Bundle, Component, Sprite};

#[derive(Bundle)]
pub struct UnitBundle {
    unit: Unit,
    name: Name,
    stats: Stats,
    sprite: Sprite,
}

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Stats {
    pub hit_points: i32,
}
