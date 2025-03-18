use bevy::prelude::*;
use serde::{Serialize, Deserialize};

// Character stats components
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Health {
    pub current: f32,
    pub max: f32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub recovery_rate: f32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct CharacterStats {
    pub strength: u8,
    pub agility: u8,
    pub intelligence: u8,
    pub charisma: u8,
    pub level: u8,
    pub experience: u32,
}

// Faction and relationship components
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Faction {
    pub id: String,
    pub name: String,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Reputation {
    pub faction_relations: Vec<(String, i32)>, // (faction_id, relation_value)
}

// Player-specific components
#[derive(Component, Debug, Clone)]
pub struct Player;

#[derive(Component, Debug, Clone)]
pub struct CharacterController {
    pub movement_speed: f32,
    pub rotation_speed: f32,
}

// World components
#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Settlement {
    pub name: String,
    pub prosperity: u32,
    pub garrison_size: u32,
    pub owner_faction_id: String,
}

// Combat components
#[derive(Component, Debug, Clone)]
pub struct Weapon {
    pub damage: f32,
    pub speed: f32,
    pub reach: f32,
    pub weapon_type: WeaponType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WeaponType {
    OneHandedSword,
    TwoHandedSword,
    Spear,
    Bow,
    Crossbow,
    Shield,
}

#[derive(Component, Debug)]
pub struct CombatAI {
    pub aggression: f32, // 0.0 to 1.0
    pub preferred_distance: f32,
}