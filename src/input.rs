use bevy::prelude::*;
use avian2d::{
    math::{Scalar, PI, AdjustPrecision},
    prelude::LinearVelocity};
use crate::keybinds::MovementAction;
use crate::player::*;

pub fn keyboard_input(
    mut movement_event_writer: EventWriter<MovementAction>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let left = keyboard_input.any_pressed([KeyCode::KeyA, KeyCode::ArrowLeft]);
    let right = keyboard_input.any_pressed([KeyCode::KeyD, KeyCode::ArrowRight]);

    let horizontal = right as i8 - left as i8;
    let direction = horizontal as Scalar;

    if direction != 0.0 {
        movement_event_writer.send(MovementAction::Move(direction));
    }

    if keyboard_input.any_just_pressed([KeyCode::Space, KeyCode::KeyW, KeyCode::ArrowUp]) {
        movement_event_writer.send(MovementAction::Jump);
    }
}

#[derive(Bundle)]
pub struct MovementBundle {
    pub acceleration: MovementAcceleration,
    pub damping: MovementDampingFactor,
    pub jump_impulse: JumpForce,
    pub max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpForce(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(0.5, 1.5, 500.0, PI * 0.45)
    }
}

/// Responds to [`MovementAction`] events and moves character controllers accordingly.
pub fn movement(
    mut commands: Commands,
    time: Res<Time>,
    mut movement_event_reader: EventReader<MovementAction>,
    mut controllers: Query<(
        Entity,
        &MovementAcceleration,
        &JumpForce,
        &mut LinearVelocity,
        Has<Grounded>,
        Has<DoubleJump>,
    ), With<Player>>,
) {
    // Precision is adjusted so that the example works with
    // both the `f32` and `f64` features. Otherwise you don't need this.
    let delta_time = time.delta_secs_f64().adjust_precision();

    for event in movement_event_reader.read() {
        for (entity, movement_acceleration, jump_impulse, mut linear_velocity, is_grounded, can_couble_jump) in
            &mut controllers
        {
            match event {
                MovementAction::Move(direction) => {
                    linear_velocity.x += *direction * movement_acceleration.0 * delta_time;
                }
                MovementAction::Jump => {
                    if is_grounded {
                        linear_velocity.y = jump_impulse.0;
                    } else if can_couble_jump {
                        linear_velocity.y = jump_impulse.0;
                        commands.entity(entity).remove::<DoubleJump>();
                    }
                }
            }
        }
    }
}

/// Slows down movement in the X direction.
pub fn apply_movement_damping(mut query: Query<(&MovementDampingFactor, &mut LinearVelocity)>) {
    for (damping_factor, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= damping_factor.0;
    }
}
