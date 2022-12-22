mod camera;
mod grid;
mod sprites;
mod unit;
mod window;

use crate::camera::new_game_camera;
use crate::sprites::AnimatedSpriteBundle;
use crate::window::setup_window;
use bevy::prelude::{
    default, App, AssetServer, Assets, Camera2dBundle, Commands, Component, Deref, DerefMut,
    GlobalTransform, Handle, ImagePlugin, PluginGroup, Query, Res, ResMut, SpriteSheetBundle,
    TextureAtlas, TextureAtlasSprite, Time, Timer, TimerMode, Transform, Vec2, Vec3,
};
use bevy::DefaultPlugins;
use bevy_ecs_ldtk::{LdtkPlugin, LdtkWorldBundle, LevelSelection};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(LdtkPlugin)
        .add_startup_system(setup)
        .add_startup_system(setup_window)
        .insert_resource(LevelSelection::Index(0))
        .add_system(sprites::animate_sprite)
        .add_system(camera::fit_camera)
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

    commands.spawn(new_game_camera());
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("maps/levels.ldtk"),
        ..Default::default()
    });
    commands.spawn(AnimatedSpriteBundle::new(texture_atlas_handle, 46, 47));
}
