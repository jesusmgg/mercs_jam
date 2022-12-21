use bevy::prelude::{
    default, App, AssetServer, Assets, Bundle, Camera2dBundle, Commands, Component, Deref,
    DerefMut, Handle, ImagePlugin, PluginGroup, Query, Res, ResMut, SpriteSheetBundle,
    TextureAtlas, TextureAtlasSprite, Time, Timer, TimerMode, Transform, Vec2, Vec3,
};

#[derive(Bundle)]
pub struct AnimatedSpriteBundle {
    sprite: SpriteSheetBundle,
    indices: AnimationSpriteIndices,
    timer: AnimationTimer,
}

impl AnimatedSpriteBundle {
    pub fn new(
        sprite_sheet_handle: Handle<TextureAtlas>,
        index_start: usize,
        index_end: usize,
    ) -> Self {
        AnimatedSpriteBundle {
            sprite: SpriteSheetBundle {
                texture_atlas: sprite_sheet_handle,
                sprite: TextureAtlasSprite::new(index_start),
                transform: Transform::from_scale(Vec3::splat(8.)),
                ..default()
            },
            indices: AnimationSpriteIndices {
                start: index_start,
                end: index_end,
            },
            timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        }
    }

    pub fn set_sprite_index(&mut self, index: usize) {
        self.sprite.sprite.index = index;
    }

    pub fn sprite_index(&self) -> usize {
        self.sprite.sprite.index
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationSpriteIndices {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationTimer,
        &AnimationSpriteIndices,
        &mut TextureAtlasSprite,
    )>,
) {
    for (mut timer, sprite_indices, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index += 1;
            if sprite.index > sprite_indices.end {
                sprite.index = sprite_indices.start;
            }
        }
    }
}
