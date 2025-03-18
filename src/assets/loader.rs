use bevy::prelude::*;
use bevy::asset::LoadState;
use std::collections::HashSet;

use crate::core::states::{GameState, check_loading_complete};

// Custom asset collection for game assets
#[derive(Resource, Default)]
pub struct GameAssets {
    // Character models
    pub character_models: Vec<Handle<Scene>>,
    // Textures
    pub terrain_textures: Vec<Handle<Image>>,
    // Audio
    pub music_tracks: Vec<Handle<AudioSource>>,
    pub sound_effects: Vec<Handle<AudioSource>>,
    // UI elements
    pub ui_textures: Vec<Handle<Image>>,
    pub fonts: Vec<Handle<Font>>,
    
    // Tracking for load status
    pub asset_handles: HashSet<HandleId>,
}

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<GameAssets>()
            .add_systems(Startup, load_assets)
            .add_systems(
                Update, 
                check_loading_complete.run_if(in_state(GameState::Loading))
            );
    }
}

fn load_assets(
    mut game_assets: ResMut<GameAssets>,
    asset_server: Res<AssetServer>,
) {
    info!("Loading game assets...");
    
    // Add all the asset handles we need to load
    
    // Character models
    let character_handle = asset_server.load("models/character.glb");
    game_assets.character_models.push(character_handle.clone());
    game_assets.asset_handles.insert(character_handle.id());
    
    // Terrain textures
    let terrain_handle = asset_server.load("textures/terrain.png");
    game_assets.terrain_textures.push(terrain_handle.clone());
    game_assets.asset_handles.insert(terrain_handle.id());
    
    // Music
    let music_handle = asset_server.load("audio/background_music.ogg");
    game_assets.music_tracks.push(music_handle.clone());
    game_assets.asset_handles.insert(music_handle.id());
    
    // UI
    let font_handle = asset_server.load("fonts/game_font.ttf");
    game_assets.fonts.push(font_handle.clone());
    game_assets.asset_handles.insert(font_handle.id());
    
    // And so on for other assets...
}

// A system to check if all assets have loaded
pub fn check_asset_loading(
    game_assets: Res<GameAssets>,
    asset_server: Res<AssetServer>,
) -> bool {
    // Get load states for all handles
    let load_states = asset_server.get_group_load_state(game_assets.asset_handles.iter().copied());
    
    // Only return true if all assets are loaded
    match load_states {
        LoadState::Loaded => true,
        _ => false,
    }
}

// Helper function to get a random asset from a collection
pub fn get_random_asset<T: Clone>(assets: &[Handle<T>]) -> Option<Handle<T>> {
    if assets.is_empty() {
        return None;
    }
    
    let idx = rand::random::<usize>() % assets.len();
    Some(assets[idx].clone())
}