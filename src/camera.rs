use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::{
    default, Assets, Camera2dBundle, Color, Handle, OrthographicProjection, Query, Res, Transform,
    Windows, Without,
};
use bevy_ecs_ldtk::{LdtkLevel, LevelSelection};

pub fn new_game_camera() -> Camera2dBundle {
    let mut camera = Camera2dBundle::default();
    camera.camera_2d.clear_color = ClearColorConfig::Custom(Color::BLACK);

    camera
}

pub fn fit_camera(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform)>,
    level_query: Query<(&Transform, &Handle<LdtkLevel>), Without<OrthographicProjection>>,
    level_selection: Res<LevelSelection>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
    windows: Res<Windows>,
) {
    let window = windows.get_primary().unwrap();
    let aspect_ratio = window.width() / window.height();

    let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

    for (level_transform, level_handle) in &level_query {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level = &ldtk_level.level;
            if level_selection.is_match(&0, level) {
                let level_width = level.px_wid as f32;
                let level_height = level.px_hei as f32;
                let level_ratio = level_width / level_height;

                orthographic_projection.scaling_mode = bevy::render::camera::ScalingMode::None;
                orthographic_projection.bottom = 0.;
                orthographic_projection.left = 0.;

                // Level wider than window
                if level_ratio > aspect_ratio {
                    orthographic_projection.top = level_height / aspect_ratio;
                    orthographic_projection.right = level_width;
                    camera_transform.translation.x = 0.;
                    camera_transform.translation.y =
                        -((orthographic_projection.top - level_height) / 2.);
                }
                // Level taller than window (or square)
                else {
                    orthographic_projection.top = level_height;
                    orthographic_projection.right = level_width * aspect_ratio;
                    camera_transform.translation.x =
                        -((orthographic_projection.right - level_width) / 2.);

                    camera_transform.translation.y = 0.;
                }
            }
        }
    }
}
