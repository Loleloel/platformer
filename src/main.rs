mod plugin;
mod player;
mod input;
mod keybinds;
mod platforms;
mod configs;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use bevy_light_2d::prelude::*;
use plugin::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(20.0),
            Light2dPlugin,
            CharacterControllerPlugin,
            LevelGenerationPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert_resource(Gravity(Vector::NEG_Y * 1000.0))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
) {
    // Camera
    commands.spawn((
        Camera2d,
        player::MainCamera,
        AmbientLight2d {
            color: Color::srgb(1.0, 1.0, 1.0),
            brightness: 0.5,
        },
    ));
}