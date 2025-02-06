//! Checkpoint: Corrected Ground Physics Components
//! Date: 2025-02-05 21:22:12
//! Author: Loleloel
//! Compatible with:
//! - Bevy: 0.15.x
//! - bevy_rapier2d: 0.28.x
//! 
//! Working Features:
//! - Proper ground collision setup per official docs
//! - Solid ground physics properties

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ground);
    }
}

fn spawn_ground(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(1000.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(500.0, 16.0),
        Friction::coefficient(1.0),
        Restitution::coefficient(0.0),
        ActiveEvents::COLLISION_EVENTS,
    ));
}