use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_light_2d::prelude::{LightOccluder2d, LightOccluder2dShape};
use crate::configs::get_window_bounds;

#[derive(Component)]
pub struct Platform;

#[derive(Bundle)]
pub struct PlatformBundle {
    platform: Platform,
    sprite: Sprite,
    transform: Transform,
    rigidbody: RigidBody,
    collider: Collider,
    light_occluder: LightOccluder2d,
}

impl PlatformBundle {
    pub fn new(width: f32, height: f32, pos_x: f32, pos_y: f32, assets: &Res<AssetServer>) -> Self {
        Self {
            platform: Platform,
            sprite: Sprite {
                image: assets.load("platform_1.png"),
                image_mode: SpriteImageMode::Tiled { tile_x: true, tile_y: false, stretch_value: 1.0 },
                custom_size: Some(Vec2::new(width, height)),
                ..default()
            },
            transform: Transform::from_xyz(pos_x, pos_y, 0.0),
            rigidbody: RigidBody::Static,
            collider: Collider::rectangle(width, height),
            light_occluder: LightOccluder2d { shape: LightOccluder2dShape::Rectangle { half_size: Vec2::new(width / 2.0, height / 2.0) }}
        }
    }
}

pub fn spawn_initial_platform(mut commands: Commands, assets: Res<AssetServer>, window_query:Query<&Window> ) {
    commands.spawn(PlatformBundle::new(get_window_bounds(window_query).x, 32.0, 0.0, -175.0, &assets));
    commands.spawn(PlatformBundle::new(320.0, 32.0, 175.0, -32.0, &assets));
    commands.spawn(PlatformBundle::new(320.0, 32.0, -175.0, 0.0, &assets));
}
