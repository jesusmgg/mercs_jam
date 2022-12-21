use bevy::prelude::{
    App, AssetServer, Assets, Camera2dBundle, Commands, Component, Deref, DerefMut, Handle,
    ImagePlugin, PluginGroup, Query, Res, ResMut, SpriteSheetBundle, TextureAtlas,
    TextureAtlasSprite, Time, Timer, TimerMode, Transform, Vec2, Vec3,
};
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationSpriteIndices {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

pub fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &AnimationSpriteIndices,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, sprite_indices, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index += 1;
            if sprite.index > sprite_indices.end {
                sprite.index = sprite_indices.start;
            }
        }
    }
}
