use bevy::prelude::{
    Bundle, Component, Sprite, SpriteBundle, SpriteSheetBundle, TextureAtlasSprite,
};

#[derive(Bundle)]
pub struct UnitBundle {
    unit: Unit,
    name: Name,
    stats: Stats,

    sprite: TextureAtlasSprite,
}

#[derive(Component)]
pub struct Unit;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Stats {
    pub hit_points: i32,
}
