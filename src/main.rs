mod unit;

use bevy::prelude::{
    default, App, AssetServer, Assets, Camera2dBundle, Commands, Component, Deref, DerefMut,
    Handle, ImagePlugin, PluginGroup, Query, Res, ResMut, SpriteSheetBundle, TextureAtlas,
    TextureAtlasSprite, Time, Timer, TimerMode, Transform, Vec2, Vec3,
};
use bevy::DefaultPlugins;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_startup_system(setup)
        .add_system(animate_sprite)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("spritesheets/tbj4_compilation_sf.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8., 8.), 16, 8, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(46),
            transform: Transform::from_scale(Vec3::splat(8.)),
            ..default()
        },
        AnimationSpriteIndices { start: 46, end: 47 },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
struct AnimationSpriteIndices {
    start: usize,
    end: usize,
}

fn animate_sprite(
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
