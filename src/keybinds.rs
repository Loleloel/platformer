use bevy::prelude::Event;
use avian2d::math::Scalar;

#[derive(Event)]
pub enum MovementAction {
    Move(Scalar),
    Jump,
}