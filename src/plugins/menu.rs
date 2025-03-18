use bevy::prelude::*;
use crate::core::states::GameState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add menu-specific systems
            .add_systems(
                Update, 
                (
                    handle_menu_input,
                    update_menu_ui,
                )
                .run_if(in_state(GameState::MainMenu))
            )
            
            // Add systems for character creation
            .add_systems(
                Update,
                handle_character_creation.run_if(in_state(GameState::CharacterCreation))
            )
            
            // Add systems for pause menu
            .add_systems(
                Update,
                handle_pause_menu.run_if(in_state(GameState::Pause))
            )
            
            // Systems for entering/exiting menus
            .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
            .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu);
    }
}

// Menu systems
fn setup_main_menu(mut commands: Commands) {
    info!("Setting up main menu");
    // Create menu UI
    // Setup menu buttons
}

fn cleanup_main_menu() {
    info!("Cleaning up main menu");
    // Clean up menu entities
}

fn handle_menu_input() {
    // Handle button clicks and menu navigation
}

fn update_menu_ui() {
    // Update menu animations or dynamic content
}

fn handle_character_creation() {
    // Handle character creation UI and logic
}

fn handle_pause_menu() {
    // Handle pause menu interactions
}