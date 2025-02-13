use bevy::{prelude::*, window::Window};

pub fn get_window_bounds(window_query: Query<&Window>) -> Vec2 {
    let window = window_query.single();
    Vec2::new(window.width(), window.height())
}