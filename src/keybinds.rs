use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum GameAction {
    MoveLeft,
    MoveRight,
    Jump,
}

#[derive(Resource, Debug)]
pub struct KeyBindings {
    bindings: HashMap<GameAction, (KeyCode, KeyCode)>,
}

// Separate struct for serialization
#[derive(Serialize, Deserialize)]
struct SerializableBindings {
    bindings: HashMap<GameAction, (String, String)>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        // Set default key bindings
        bindings.insert(GameAction::MoveLeft, (KeyCode::KeyA, KeyCode::ArrowLeft));
        bindings.insert(GameAction::MoveRight, (KeyCode::KeyD, KeyCode::ArrowRight));
        bindings.insert(GameAction::Jump, (KeyCode::Space, KeyCode::KeyW));
        
        Self { bindings }
    }
}

impl KeyBindings {
    fn to_serializable(&self) -> SerializableBindings {
        let bindings = self
            .bindings
            .iter()
            .map(|(action, (primary, secondary))| {
                (action.clone(), (primary.to_string(), secondary.to_string()))
            })
            .collect();

        SerializableBindings { bindings }
    }

    fn from_serializable(serialized: SerializableBindings) -> Option<Self> {
        let bindings = serialized
            .bindings
            .into_iter()
            .map(|(action, (primary, secondary))| {
                // Parse string back to KeyCode
                let primary_key = KeyCode::from_str(&primary).ok()?;
                let secondary_key = KeyCode::from_str(&secondary).ok()?;
                Some((action, (primary_key, secondary_key)))
            })
            .collect::<Option<HashMap<_, _>>>()?;

        Some(Self { bindings })
    }

    fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let serializable = self.to_serializable();
        let serialized = ron::to_string(&serializable)?;
        fs::write("keybinds.ron", serialized)?;
        Ok(())
    }

    pub fn load_from_file() -> Self {
        let path = Path::new("keybinds.ron");
        if path.exists() {
            let contents = fs::read_to_string(path).expect("Failed to read keybinds file");
            let serializable: SerializableBindings = ron::from_str(&contents)
                .expect("Failed to deserialize keybinds");
            
            Self::from_serializable(serializable)
                .unwrap_or_else(|| {
                    println!("Failed to parse keybinds, using defaults");
                    Self::default()
                })
        } else {
            let default = Self::default();
            default.save_to_file().expect("Failed to save default keybinds");
            default
        }
    }

    pub fn is_action_pressed(&self, action: GameAction, input: &ButtonInput<KeyCode>) -> bool {
        self.bindings
            .get(&action)
            .map_or(false, |&(primary, secondary)| 
                input.pressed(primary) || input.pressed(secondary)
            )
    }

    pub fn is_action_just_pressed(&self, action: GameAction, input: &ButtonInput<KeyCode>) -> bool {
        self.bindings
            .get(&action)
            .map_or(false, |&(primary, secondary)| 
                input.just_pressed(primary) || input.just_pressed(secondary)
            )
    }

    pub fn set_binding(&mut self, action: GameAction, primary: KeyCode, secondary: KeyCode) {
        self.bindings.insert(action, (primary, secondary));
        self.save_to_file().expect("Failed to save key bindings");
    }

    pub fn get_bindings(&self, action: &GameAction) -> Option<&(KeyCode, KeyCode)> {
        self.bindings.get(action)
    }

    pub fn get_all_bindings(&self) -> &HashMap<GameAction, (KeyCode, KeyCode)> {
        &self.bindings
    }

    pub fn update_binding(&mut self, action: GameAction, key: KeyCode, is_primary: bool) -> Result<(), String> {
        let binding = self.bindings.entry(action).or_insert((KeyCode::Space, KeyCode::Space));
        if is_primary {
            binding.0 = key;
        } else {
            binding.1 = key;
        }
        self.save_to_file().map_err(|e| e.to_string())
    }
}

// Helper trait for KeyCode string conversion
trait KeyCodeExt {
    fn from_str(s: &str) -> Result<KeyCode, String>;
    fn to_string(&self) -> String;
}

impl KeyCodeExt for KeyCode {
    fn from_str(s: &str) -> Result<KeyCode, String> {
        KeyCode::from_str(s).map_err(|_| format!("Unknown key code: {}", s))
    }

    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyBindings>()
           .add_systems(Update, handle_keybind_change);
    }
}

pub fn handle_keybind_change(
    mut next_key_event: EventReader<KeyboardInput>,
    mut waiting_for_key: Local<Option<(GameAction, bool)>>, // (Action, is_primary)
    mut keybinds: ResMut<KeyBindings>,
) {
    if let Some((action, is_primary)) = waiting_for_key.as_ref() {
        for event in next_key_event.read() {
            if event.state.is_pressed() {
                if let Err(e) = keybinds.update_binding(action.clone(), event.key_code, *is_primary) {
                    error!("Failed to update keybinding: {}", e);
                }
                *waiting_for_key = None;
                break;
            }
        }
    }
}