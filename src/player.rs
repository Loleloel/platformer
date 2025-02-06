use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

const GRAVITY: f32 = -1500.0;
const TERMINAL_VELOCITY: f32 = -1000.0;

#[derive(Component)]
pub struct Player {
    pub speed: f32,
    pub jump_force: f32,
    pub can_double_jump: bool,
    pub vertical_velocity: f32,
    pub is_jumping: bool,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 200.0,
            jump_force: 500.0,
            can_double_jump: true,
            vertical_velocity: 0.0,
            is_jumping: false,
        }
    }
}

fn spawn_player(mut commands: Commands) {
    // Spawn the player with revised configuration
    commands.spawn((
        Sprite {
            color: Color::srgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(32.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 50.0, 0.0),
        Player::default(),
        RigidBody::KinematicVelocityBased,
        Collider::cuboid(16.0, 16.0),
        KinematicCharacterController {
            translation: Some(Vec2::ZERO), // Initialize with no movement
            slide: true, // Enable sliding along surfaces
            ..default()
        },
        Velocity::zero(),
        LockedAxes::ROTATION_LOCKED,
    ));
}

fn player_movement(
    time: Res<Time>,
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &mut KinematicCharacterController, &KinematicCharacterControllerOutput)>,
) {
    for (mut player, mut controller, output) in query.iter_mut() {
        // Horizontal movement
        let mut horizontal_velocity = 0.0;
        if input.pressed(KeyCode::KeyA) || input.pressed(KeyCode::ArrowLeft) {
            horizontal_velocity -= player.speed;
        }
        if input.pressed(KeyCode::KeyD) || input.pressed(KeyCode::ArrowRight) {
            horizontal_velocity += player.speed;
        }

        // Apply gravity only when not grounded
        if !output.grounded {
            player.vertical_velocity += GRAVITY * time.delta_secs();
            player.vertical_velocity = player.vertical_velocity.max(TERMINAL_VELOCITY);
        } else {
            player.vertical_velocity = 0.0;
            player.can_double_jump = true;
            player.is_jumping = false;
        }

        // Calculate movement translation
        let movement = Vec2::new(
            horizontal_velocity * time.delta_secs(),
            player.vertical_velocity * time.delta_secs()
        );

        // Update the character controller translation
        controller.translation = Some(movement);
    }
}

fn player_jump(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Player, &KinematicCharacterControllerOutput)>,
) {
    for (mut player, output) in query.iter_mut() {
        // Handle jumping
        if input.just_pressed(KeyCode::Space) {
            if output.grounded {
                player.vertical_velocity = player.jump_force;
                player.is_jumping = true;
            } else if player.can_double_jump {
                player.vertical_velocity = player.jump_force;
                player.can_double_jump = false;
            }
        }

        // Ceiling collision
        if output.effective_translation.y < output.desired_translation.y 
            && player.vertical_velocity > 0.0 
        {
            player.vertical_velocity = 0.0;
        }
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
           .add_plugins(RapierDebugRenderPlugin::default())
           .add_systems(Startup, spawn_player)
           .add_systems(Update, (
               player_movement,
               player_jump,
           ).chain().before(PhysicsSet::StepSimulation));
    }
}
