use bevy::prelude::*;
mod player;
mod physics;
mod game_state;
// mod animation;
// mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(player::PlayerPlugin)
        .add_plugins(physics::PhysicsPlugin)
        .insert_resource(State::new(game_state::GameState::default()))  // Changed this line
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());  // Also note: Camera2dBundle instead of Camera2d
}
