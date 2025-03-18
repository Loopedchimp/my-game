use bevy::prelude::*;
use crate::core::states::{GameState, WorldMapState};
use crate::core::components::WorldPosition;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app
            // Register the world map substate
            .add_sub_state::<WorldMapState>()
            
            // Add systems that run only in WorldMap state
            .add_systems(
                Update, 
                (
                    update_world_map,
                    handle_world_map_input,
                )
                .run_if(in_state(GameState::WorldMap))
            )
            
            // Systems for different world map substates
            .add_systems(
                Update,
                handle_encounter.run_if(in_state(WorldMapState::Encounter))
            )
            
            // Systems for entering/exiting world map
            .add_systems(OnEnter(GameState::WorldMap), setup_world_map)
            .add_systems(OnExit(GameState::WorldMap), cleanup_world_map);
    }
}

// World map systems
fn setup_world_map(mut commands: Commands) {
    info!("Setting up world map");
    // Create map visuals
    // Setup world map camera
    // Initialize settlements, etc.
}

fn cleanup_world_map() {
    info!("Cleaning up world map");
    // Clean up entities specific to world map
}

fn update_world_map() {
    // Update time, weather, etc.
}

fn handle_world_map_input() {
    // Handle player input on world map
}

fn handle_encounter() {
    // Handle random encounters
}