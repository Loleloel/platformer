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
use bevy::window::Window;

use crate::player::Player;

#[derive(Resource)]
pub struct WindowBounds {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

impl WindowBounds {
    fn from_window(window: &Window) -> Self {
        let half_width = window.width() / 2.0;
        let half_height = window.height() / 2.0;
        Self {
            left: -half_width,
            right: half_width,
            top: half_height,
            bottom: -half_height,
        }
    }
}

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowBounds {
                left: 0.0,
                right: 0.0,
                top: 0.0,
                bottom: 0.0,
            })
            .add_systems(Startup, (spawn_ground, setup_window_bounds))
            .add_systems(Update, constrain_player_to_window);
    }
}

fn spawn_ground(mut commands: Commands) {
    commands.spawn((
        Sprite {
            color: Color::srgb(0.3, 0.3, 0.3),
            custom_size: Some(Vec2::new(2000.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
        RigidBody::Fixed,
        Collider::cuboid(1000.0, 16.0),
        Friction::coefficient(1.0),
        Restitution::coefficient(0.0),
        ActiveEvents::COLLISION_EVENTS,
     ));
}

fn setup_window_bounds(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.single();
    commands.insert_resource(WindowBounds::from_window(window));
}

fn constrain_player_to_window(
    mut player_query: Query<(&mut Transform, &Sprite), With<Player>>,
    bounds: Res<WindowBounds>,
) {
    for (mut transform, sprite) in player_query.iter_mut() {
        // Get half size of the sprite (if custom_size is set, use that, otherwise use the default size)
        let half_size = sprite.custom_size
            .map(|size| size / 2.0)
            .unwrap_or(Vec2::new(50.0, 50.0)); // Default size if none is set

        // Adjust bounds to account for sprite size
        let adjusted_bounds = WindowBounds {
            left: bounds.left + half_size.x,
            right: bounds.right - half_size.x,
            top: bounds.top - half_size.y,
            bottom: bounds.bottom + half_size.y,
        };

        // Clamp the position
        transform.translation.x = transform.translation.x.clamp(adjusted_bounds.left, adjusted_bounds.right);
        transform.translation.y = transform.translation.y.clamp(adjusted_bounds.bottom, adjusted_bounds.top);
    }
}