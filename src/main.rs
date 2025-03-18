use bevy::prelude::*;

mod core;
mod plugins;
mod assets;
mod save;

use core::states::GameState;
use plugins::{CombatPlugin, WorldMapPlugin, MenuPlugin};
use assets::AssetsPlugin;
use save::SaveSystemPlugin;

fn main() {
    App::new()
        // Add core Bevy plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bannerlord-Bevy".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        
        // Initialize the game state
        .add_sub_state::<GameState>()
        
        // Register our custom plugins
        .add_plugins((
            AssetsPlugin,
            SaveSystemPlugin,
            MenuPlugin,
            WorldMapPlugin,
            CombatPlugin,
        ))
        
        // Add core startup systems
        .add_systems(Startup, setup)
        
        .run();
}

// Basic setup system to initialize the game world
fn setup(mut commands: Commands) {
    // Add a camera
    commands.spawn((
        Camera3d,
        Camera,
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));
    
    // Add ambient light
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(10.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        GlobalTransform::default(),
    ));
}