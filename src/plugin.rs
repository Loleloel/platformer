use bevy::prelude::*;
use avian2d::prelude::PhysicsSet;
use crate::{
    player::*, 
    input::keyboard_input, 
    keybinds::MovementAction,
    input::{movement, apply_movement_damping},
    platforms::*,
};

pub struct CharacterControllerPlugin;

impl Plugin for CharacterControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MovementAction>()
            .add_systems(Startup, PlayerBundle::spawn_player)
            .add_systems(
            Update,
            (
                keyboard_input,
                update_grounded,
                movement,
                apply_movement_damping,
                camera_follow,
            )
                .chain()
                .before(PhysicsSet::StepSimulation),
        );
    }
}

pub struct LevelGenerationPlugin;

impl Plugin for LevelGenerationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (spawn_initial_platform, spawn_chain));
    }
}

fn spawn_chain(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: assets.load("chain_1.png"),
            image_mode: SpriteImageMode::Tiled { tile_x: false, tile_y: true, stretch_value: 0.5 },
            custom_size: Some(Vec2::new(8.0, 480.0)),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(300.0, -16.0, 1.0)));
    commands.spawn((
        Sprite {
            image: assets.load("chain_1.png"),
            image_mode: SpriteImageMode::Tiled { tile_x: false, tile_y: true, stretch_value: 0.5 },
            custom_size: Some(Vec2::new(8.0, 480.0)),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(30.0, -16.0, 1.0)));
    commands.spawn((
        Sprite {
            image: assets.load("chain_1.png"),
            image_mode: SpriteImageMode::Tiled { tile_x: false, tile_y: true, stretch_value: 0.5 },
            custom_size: Some(Vec2::new(8.0, 480.0)),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(-300.0, 16.0, 1.0)));
    commands.spawn((
        Sprite {
            image: assets.load("chain_1.png"),
            image_mode: SpriteImageMode::Tiled { tile_x: false, tile_y: true, stretch_value: 0.5 },
            custom_size: Some(Vec2::new(8.0, 480.0)),
            anchor: bevy::sprite::Anchor::BottomCenter,
            ..default()
        },
        Transform::from_xyz(-30.0, 16.0, 1.0)));
}

fn camera_follow(
    time: Res<Time>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera_query: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.get_single() else { return };
    let Ok(mut camera_transform) = camera_query.get_single_mut() else { return };

    let target = Vec3::new(
        player_transform.translation.x,
        player_transform.translation.y,
        camera_transform.translation.z
    );

    camera_transform.translation = camera_transform.translation.lerp(
        target,
        time.delta_secs() * 5.0
    );
}
