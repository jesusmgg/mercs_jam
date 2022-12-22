use bevy::prelude::{Res, ResMut, Resource, Windows};

const WINDOW_WIDTH: usize = 960;
const WINDOW_HEIGHT: usize = 540;

pub fn setup_window(windows: ResMut<Windows>) {
    set_window_size(WINDOW_WIDTH, WINDOW_HEIGHT, windows);
}

fn set_window_size(width: usize, height: usize, mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    window.set_resolution(width as f32, height as f32);
}
