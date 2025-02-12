mod plugin;
mod player;
mod input;
mod keybinds;

use avian2d::{math::*, prelude::*};
use bevy::prelude::*;
use plugin::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PhysicsPlugins::default().with_length_unit(20.0),
            CharacterControllerPlugin,
        ))
        .insert_resource(ClearColor(Color::srgb(0.05, 0.05, 0.1)))
        .insert_resource(Gravity(Vector::NEG_Y * 900.0))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    // Platforms
    commands.spawn((
        Sprite {
            image: assets.load("platform_1.png"),
            image_mode: SpriteImageMode::Tiled { tile_x: true, tile_y: false, stretch_value: 1.0 },
            custom_size: Some(Vec2::new(2000.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -175.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(2000.0, 32.0),
    ));
    commands.spawn((
        Sprite {
            image: assets.load("platform_1.png"),
            image_mode: SpriteImageMode::Tiled { tile_x: true, tile_y: false, stretch_value: 1.0 },
            custom_size: Some(Vec2::new(320.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(175.0, -35.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(320.0, 32.0),
    ));
    commands.spawn((
        Sprite {
            image: assets.load("platform_1.png"),
            image_mode: SpriteImageMode::Tiled { tile_x: true, tile_y: false, stretch_value: 1.0 },
            custom_size: Some(Vec2::new(320.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(-175.0, 0.0, 0.0),
        RigidBody::Static,
        Collider::rectangle(320.0, 32.0),
    ));

    // Camera
    commands.spawn(Camera2d);
}