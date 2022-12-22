use bevy::prelude::{Bundle, Component};

#[derive(Bundle)]
pub struct UnitBundle {
    unit: Unit,
    name: Name,
    stats: Stats,
}

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Stats {
    pub hit_points: i32,
}
