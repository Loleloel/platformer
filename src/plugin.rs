use bevy::prelude::*;
use crate::{
    player::*, 
    input::keyboard_input, 
    keybinds::MovementAction,
    input::{movement, apply_movement_damping},
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
            )
                .chain(),
        );
    }
}
