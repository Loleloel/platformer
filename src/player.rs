use bevy::prelude::*;
use avian2d::{math::*, prelude::*};
use crate::input::MovementBundle;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct CharacterController;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct MovementAcceleration(pub Scalar);

#[derive(Component)]
pub struct MovementDampingFactor(pub Scalar);

#[derive(Component)]
pub struct JumpForce(pub Scalar);

#[derive(Component)]
pub struct MaxSlopeAngle(pub Scalar);

#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    locked_axes: LockedAxes,
    movement: MovementBundle,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    sprite: Sprite,
    transform: Transform,
    controller: CharacterControllerBundle,
    friction: Friction,
    restitution: Restitution,
}

impl PlayerBundle {
    pub fn spawn_player(mut commands: Commands) {
        commands.spawn((
            PlayerBundle::default(),
            ColliderDensity(2.0),
            GravityScale(1.5),));
    }
}
    
impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            player: Player,
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.5),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -100.0, 0.0),
            controller: CharacterControllerBundle::new(Collider::rectangle(32.0, 32.0)).with_movement(
                5000.0,
                0.80,
                650.0,
                (30.0 as Scalar).to_radians(),
            ),
            friction: Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
            restitution: Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        }
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(caster_shape, Vector::ZERO, 0.0, Dir2::NEG_Y)
                .with_max_distance(10.0),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: MovementBundle {
                acceleration: MovementAcceleration(0.5),
                damping: MovementDampingFactor(0.7),
                jump_impulse: JumpForce(1500.0),
                max_slope_angle: MaxSlopeAngle(PI),
            },
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}

/// Updates the [`Grounded`] status for character controllers.
pub fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>,
    >,
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_to(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}
