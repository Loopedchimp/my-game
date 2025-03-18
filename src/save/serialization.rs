use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use crate::core::components::*;

pub struct SaveSystemPlugin;

impl Plugin for SaveSystemPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SaveGameConfig>()
            .add_event::<SaveGameEvent>()
            .add_event::<LoadGameEvent>()
            .add_systems(Update, (handle_save_game, handle_load_game));
    }
}

#[derive(Resource)]
pub struct SaveGameConfig {
    pub save_directory: PathBuf,
}

impl Default for SaveGameConfig {
    fn default() -> Self {
        Self {
            save_directory: Path::new("saves").to_path_buf(),
        }
    }
}

// Events for saving and loading
#[derive(Event)]
pub struct SaveGameEvent {
    pub save_name: String,
}

#[derive(Event)]
pub struct LoadGameEvent {
    pub save_name: String,
}

// Main save game data structure
#[derive(Serialize, Deserialize)]
pub struct GameSave {
    pub version: String,
    pub save_date: String,
    pub world_data: WorldData,
    pub player_data: PlayerData,
    pub factions_data: Vec<FactionData>,
    pub settlements_data: Vec<SettlementData>,
}

// Sub-structures for different game elements
#[derive(Serialize, Deserialize)]
pub struct WorldData {
    pub game_time: f64,
    pub day: u32,
    pub seed: u64,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerData {
    pub name: String,
    pub position: (f32, f32),
    pub stats: CharacterStats,
    pub health: Health,
    pub stamina: Stamina,
    pub reputation: Vec<(String, i32)>,
    pub inventory: Vec<ItemData>,
}

#[derive(Serialize, Deserialize)]
pub struct FactionData {
    pub id: String,
    pub name: String,
    pub relations: Vec<(String, i32)>,
}

#[derive(Serialize, Deserialize)]
pub struct SettlementData {
    pub name: String,
    pub position: (f32, f32),
    pub owner_faction: String,
    pub prosperity: u32,
    pub garrison: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ItemData {
    pub name: String,
    pub item_type: String,
    pub value: u32,
    pub stats: Vec<(String, f32)>,
}

// Systems for handling save/load
fn handle_save_game(
    mut save_events: EventReader<SaveGameEvent>,
    config: Res<SaveGameConfig>,
    query_player: Query<(&Transform, &CharacterStats, &Health, &Stamina), With<Player>>,
    query_settlements: Query<(&Settlement, &WorldPosition)>,
    // Add other queries for game data you want to save
) {
    for event in save_events.read() {
        info!("Saving game: {}", event.save_name);
        
        // Ensure save directory exists
        if !config.save_directory.exists() {
            fs::create_dir_all(&config.save_directory).expect("Failed to create save directory");
        }
        
        // Create save file path
        let save_path = config.save_directory.join(format!("{}.sav", event.save_name));
        
        // Collect player data
        let player_data = if let Ok((transform, stats, health, stamina)) = query_player.get_single() {
            PlayerData {
                name: "Player".to_string(),
                position: (transform.translation.x, transform.translation.z),
                stats: stats.clone(),
                health: health.clone(),
                stamina: stamina.clone(),
                reputation: Vec::new(), // Fill from reputation component
                inventory: Vec::new(),  // Fill from inventory component
            }
        } else {
            error!("No player found when saving game!");
            continue;
        };
        
        // Collect settlement data
        let mut settlements_data = Vec::new();
        for (settlement, position) in query_settlements.iter() {
            settlements_data.push(SettlementData {
                name: settlement.name.clone(),
                position: (position.x, position.y),
                owner_faction: settlement.owner_faction_id.clone(),
                prosperity: settlement.prosperity,
                garrison: settlement.garrison_size,
            });
        }
        
        // Create the save data structure
        let game_save = GameSave {
            version: "1.0".to_string(),
            save_date: chrono::Local::now().to_string(),
            world_data: WorldData {
                game_time: 0.0, // Fill from game time resource
                day: 1,         // Fill from calendar resource
                seed: 12345,    // Fill from world generation seed
            },
            player_data,
            factions_data: Vec::new(), // Fill from faction queries
            settlements_data,
        };
        
        // Serialize and save to file
        let serialized = serde_json::to_string_pretty(&game_save)
            .expect("Failed to serialize save data");
            
        let mut file = fs::File::create(save_path)
            .expect("Failed to create save file");
            
        file.write_all(serialized.as_bytes())
            .expect("Failed to write save file");
            
        info!("Game saved successfully");
    }
}

fn handle_load_game(
    mut load_events: EventReader<LoadGameEvent>,
    config: Res<SaveGameConfig>,
    mut commands: Commands,
    // Add resources and queries needed for loading
) {
    for event in load_events.read() {
        info!("Loading game: {}", event.save_name);
        
        // Create save file path
        let save_path = config.save_directory.join(format!("{}.sav", event.save_name));
        
        if !save_path.exists() {
            error!("Save file does not exist: {:?}", save_path);
            continue;
        }
        
        // Read file
        let mut file = fs::File::open(save_path)
            .expect("Failed to open save file");
            
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read save file");
            
        // Deserialize
        let game_save: GameSave = match serde_json::from_str(&contents) {
            Ok(save) => save,
            Err(e) => {
                error!("Failed to deserialize save file: {}", e);
                continue;
            }
        };
        
        // Clear existing world entities that should be recreated
        // commands.remove_resource::<...>();
        // entities.for_each(...);
        
        // Recreate world state from save
        // Create player
        commands.spawn((
            Player,
            CharacterStats {
                strength: game_save.player_data.stats.strength,
                agility: game_save.player_data.stats.agility,
                intelligence: game_save.player_data.stats.intelligence,
                charisma: game_save.player_data.stats.charisma,
                level: game_save.player_data.stats.level,
                experience: game_save.player_data.stats.experience,
            },
            Health {
                current: game_save.player_data.health.current,
                max: game_save.player_data.health.max,
            },
            Stamina {
                current: game_save.player_data.stamina.current,
                max: game_save.player_data.stamina.max,
                recovery_rate: game_save.player_data.stamina.recovery_rate,
            },
            // Add transform, controller, etc.
        ));
        
        // Recreate settlements
        for settlement_data in game_save.settlements_data {
            commands.spawn((
                Settlement {
                    name: settlement_data.name,
                    prosperity: settlement_data.prosperity,
                    garrison_size: settlement_data.garrison,
                    owner_faction_id: settlement_data.owner_faction,
                },
                WorldPosition {
                    x: settlement_data.position.0,
                    y: settlement_data.position.1,
                },
                // Add other components
            ));
        }
        
        // Recreate factions, items, NPCs, etc.
        
        info!("Game loaded successfully");
    }
}