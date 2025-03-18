use bevy::prelude::*;

/// Main game states that control which systems are active
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Loading,        // Initial loading screen
    MainMenu,       // Main menu
    CharacterCreation,
    WorldMap,       // Strategic map view
    Settlement,     // Inside a settlement
    Combat,         // During battles
    Inventory,      // Inventory management
    Dialogue,       // Conversation with NPCs
    Pause,          // Pause menu
}

/// SubStates for specific game modes that need their own state machine
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum CombatState {
    #[default]
    Preparation,    // Deployment phase
    Active,         // Active combat
    Victory,        // Post-battle victory
    Defeat,         // Post-battle defeat
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum WorldMapState {
    #[default]
    Free,           // Free movement
    Encounter,      // Random encounter
    Siege,          // Siege preparation
}

// State transition systems
pub fn check_loading_complete(
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
) {
    // Check if assets are done loading
    if asset_server.get_load_state(AssetServer::HANDLED_PATH) == bevy::asset::LoadState::Loaded {
        next_state.set(GameState::MainMenu);
    }
}

// Example state transition system for combat
pub fn check_combat_victory(
    mut next_combat_state: ResMut<NextState<CombatState>>,
    // You would include resources/queries to determine victory conditions
) {
    // Logic to check if combat is won
    // if victory_condition {
    //     next_combat_state.set(CombatState::Victory);
    // }
}