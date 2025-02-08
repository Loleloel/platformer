use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::keybinds::{handle_keybind_change, GameAction, KeyBindings};

/// Configuration resource for player physics and movement parameters
#[derive(Resource)]
pub struct PlayerConfig {
    /// Acceleration due to gravity (negative for downward force)
    gravity: f32,
    /// Maximum falling speed
    terminal_velocity: f32,
    /// Initial movement speed
    initial_speed: f32,
    /// Initial jump force
    initial_jump_force: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            gravity: -1500.0,
            terminal_velocity: -1000.0,
            initial_speed: 200.0,
            initial_jump_force: 400.0,
        }
    }
}

/// Represents the current state of the player's movement
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlayerState {
    Jumping,
    Standing,
    Falling,
}

/// Main player component containing movement and physics properties
#[derive(Component, Debug)]
pub struct Player {
    /// Horizontal movement speed
    speed: f32,
    /// Vertical force applied when jumping
    jump_force: f32,
    /// Whether the player can perform a double jump
    can_double_jump: bool,
    /// Current vertical velocity
    vertical_velocity: f32,
}

// Better encapsulation with private fields and public methods
impl Player {
    /// Returns the current horizontal movement speed
    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    /// Sets whether the player can perform a double jump
    pub fn set_can_double_jump(&mut self, value: bool) {
        self.can_double_jump = value;
    }

    /// Returns whether the player can currently perform a double jump
    pub fn can_double_jump(&self) -> bool {
        self.can_double_jump
    }

    /// Updates the player's movement for the current frame
    /// Returns the current velocity vector
    pub fn update_movement(&mut self, delta: f32, config: &PlayerConfig) -> Vec2 {
        // Missing ceiling collision check from original implementation
        self.apply_gravity(delta, config);
        // We should return full velocity vector including horizontal
        Vec2::new(0.0, self.vertical_velocity)
    }

    /// Applies gravity force to the player
    /// 
    /// # Arguments
    /// * `delta` - Time elapsed since last frame
    /// * `config` - Reference to player configuration
    fn apply_gravity(&mut self, delta: f32, config: &PlayerConfig) {
        self.vertical_velocity += config.gravity * delta;
        self.vertical_velocity = self.vertical_velocity.max(config.terminal_velocity);
    }

    /// Initiates a jump by setting vertical velocity
    pub fn jump(&mut self) {
        self.vertical_velocity = self.jump_force;
    }

    /// Handles collision detection and updates player state
    /// 
    /// # Arguments
    /// * `collision` - Collision output from the physics engine
    /// 
    /// # Returns
    /// The new PlayerState based on collision results
    pub fn handle_collision(&mut self, collision: &KinematicCharacterControllerOutput) -> PlayerState {
        if collision.grounded {
            // Only zero out velocity if we're actually falling
            // This prevents canceling jumps that just started
            if self.vertical_velocity < 0.0 {
                self.vertical_velocity = 0.0;
            }
            self.can_double_jump = true;
            PlayerState::Standing
        } else if self.vertical_velocity < 0.0 {
            PlayerState::Falling
        } else {
            PlayerState::Jumping
        }
    }

    /// Handles ceiling collision detection and updates player state
    /// 
    /// # Arguments
    /// * `output` - Collision output from the physics engine
    pub fn handle_ceiling_collision(&mut self, output: &KinematicCharacterControllerOutput) {
        if output.effective_translation.y < output.desired_translation.y 
            && self.vertical_velocity > 0.0 
        {
            self.vertical_velocity = 0.0;
        }
    }
}

/// Bundle of components required for the player entity
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    state: PlayerState,
    sprite: Sprite,
    transform: Transform,
    rigid_body: RigidBody,
    collider: Collider,
    controller: KinematicCharacterController,
}

impl PlayerBundle {
    /// Creates a new PlayerBundle with default values
    /// 
    /// # Arguments
    /// * `config` - Reference to player configuration
    fn new(config: &PlayerConfig) -> Self {
        Self {
            player: Player {
                speed: config.initial_speed,
                jump_force: config.initial_jump_force * 1.5, // Store the adjusted jump force
                can_double_jump: true,
                vertical_velocity: 0.0,
            },
            state: PlayerState::Falling,
            sprite: Sprite {
                color: Color::srgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(32.0, 32.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 50.0, 0.0),
            rigid_body: RigidBody::KinematicVelocityBased,
            collider: Collider::cuboid(16.0, 16.0),
            controller: KinematicCharacterController {
                translation: Some(Vec2::ZERO),
                slide: true,
                ..default()
            },
        }
    }
}

/// System for spawning the player entity
fn spawn_player(mut commands: Commands, config: Res<PlayerConfig>) {
    commands.spawn(PlayerBundle::new(&config));
}

// More efficient system with better error handling
/// System for handling player movement
/// 
/// Updates player position based on input and physics
fn player_movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    config: Res<PlayerConfig>,
    keybinds: Res<KeyBindings>,
    mut query: Query<(&mut Player, &mut KinematicCharacterController, &KinematicCharacterControllerOutput, &mut PlayerState)>,
) {
    let Ok((mut player, mut controller, output, mut state)) = query.get_single_mut() else {
        return;
    };

    let delta = time.delta_secs();
    
    // Handle ceiling collisions before movement
    player.handle_ceiling_collision(output);
    
    let velocity = player.update_movement(delta, &config);
    let horizontal_velocity = handle_horizontal_movement(&player, &input, &keybinds);

    // Set translation based on velocity * delta for proper physics movement
    controller.translation = Some(Vec2::new(
        horizontal_velocity * delta,
        velocity.y * delta
    ));

    *state = player.handle_collision(output);
}

// More robust jump handling
/// System for handling player jump input
/// 
/// Processes jump and double jump mechanics
fn player_jump(
    input: Res<ButtonInput<KeyCode>>,
    keybinds: Res<KeyBindings>,
    mut query: Query<(&mut Player, &PlayerState)>,
) {
    let Ok((mut player, state)) = query.get_single_mut() else {
        return;
    };

    if !keybinds.is_action_just_pressed(GameAction::Jump, &input) {
        return;
    }

    match *state {
        PlayerState::Standing => {
            player.jump();
        }
        PlayerState::Jumping | PlayerState::Falling if player.can_double_jump() => {
            player.set_can_double_jump(false);
            player.jump();
        }
        _ => {}
    }
}

/// Calculates horizontal movement based on input
/// 
/// # Arguments
/// * `player` - Reference to Player component
/// * `input` - Current keyboard input state
/// * `keybinds` - Current key bindings configuration
/// 
/// # Returns
/// The horizontal movement value (-1.0, 0, or 1.0) multiplied by player speed
fn handle_horizontal_movement(
    player: &Player,
    input: &ButtonInput<KeyCode>,
    keybinds: &KeyBindings,
) -> f32 {
    match (
        keybinds.is_action_pressed(GameAction::MoveLeft, input),
        keybinds.is_action_pressed(GameAction::MoveRight, input),
    ) {
        (true, false) => -player.get_speed(),
        (false, true) => player.get_speed(),
        _ => 0.0,
    }
}

/// Plugin that sets up all player-related systems and resources
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerConfig>()
           .add_plugins((
               RapierPhysicsPlugin::<NoUserData>::default(),
               RapierDebugRenderPlugin::default(),
           ))
           .add_systems(Startup, spawn_player)
           .add_systems(
               Update,
               (
                   // First handle jump input before any physics
                   player_jump,
                   // Then handle movement which includes gravity
                   player_movement,
               )
                   .chain()
                   .after(handle_keybind_change)  // Missing dependency from original
                   .before(PhysicsSet::StepSimulation)
           );
    }
}
