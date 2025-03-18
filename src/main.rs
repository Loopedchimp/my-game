// Cargo.toml
//
// [package]
// name = "bannerlord_character_system"
// version = "0.1.0"
// edition = "2021"
//
// [dependencies]
// bevy = "0.15.3" 
// rand = "0.8.5"
// bevy_egui = "0.28" 
// serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"

use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Character Core Components
#[derive(Component, Clone, Serialize, Deserialize, Debug)]
pub struct Character {
    name: String,
    gender: Gender,
    culture: Culture,
    age: u32,
    clan_level: u32,
    renown: f32,
    influence: f32,
    gold: u32,
    attributes: Attributes,
    skills: HashMap<SkillType, Skill>,
    equipment: Equipment,
    traits: Vec<Trait>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Gender {
    Male,
    Female,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Culture {
    Vlandian,
    Battanian,
    Sturgian,
    Empire,
    Aserai,
    Khuzait,
    Custom(String),
}

// Character Attributes
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Attributes {
    vigor: u32,      // Physical strength and endurance
    control: u32,    // Precision and dexterity
    endurance: u32,  // Health and stamina
    cunning: u32,    // Tactics and cleverness
    social: u32,     // Charisma and leadership
    intelligence: u32, // Learning and problem-solving
}

// Character Skills
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq, Hash)]
pub enum SkillType {
    // Combat Skills
    OneHanded,
    TwoHanded,
    Polearm,
    Bow,
    Crossbow,
    Throwing,
    Riding,
    Athletics,
    
    // Non-Combat Skills
    Crafting,
    Tactics,
    Scouting,
    Roguery,
    Charm,
    Leadership,
    Trade,
    Steward,
    Medicine,
    Engineering,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Skill {
    level: u32,
    focus_points: u32,
    experience: f32,
}

// Equipment System
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Equipment {
    head: Option<Item>,
    body: Option<Item>,
    legs: Option<Item>,
    hands: Option<Item>,
    main_weapon: Option<Item>,
    secondary_weapon: Option<Item>,
    ranged_weapon: Option<Item>,
    mount: Option<Item>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Item {
    id: String,
    name: String,
    item_type: ItemType,
    tier: u32,
    value: u32,
    weight: f32,
    stats: ItemStats,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ItemType {
    Weapon(WeaponType),
    Armor(ArmorType),
    Mount,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum WeaponType {
    OneHanded,
    TwoHanded,
    Polearm,
    Bow,
    Crossbow,
    Throwing,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum ArmorType {
    Head,
    Body,
    Legs,
    Hands,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ItemStats {
    armor: Option<u32>,
    damage: Option<u32>,
    speed: Option<f32>,
    handling: Option<f32>,
    accuracy: Option<f32>,
    range: Option<u32>,
}

// Character Traits
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Trait {
    id: String,
    name: String,
    description: String,
    effects: Vec<TraitEffect>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct TraitEffect {
    target: TraitEffectTarget,
    modifier: f32,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum TraitEffectTarget {
    Attribute(String),
    Skill(SkillType),
    Relationship(String),
    ClanInfluence,
    TradePrice,
    TroopMorale,
}

// Character Resources
#[derive(Resource)]
pub struct CharacterDatabase {
    pub characters: Vec<Entity>,
    pub active_character: Option<Entity>,
}

// Character Creator State
#[derive(Resource)]
pub struct CharacterCreatorState {
    creation_stage: CreationStage,
    temp_character: Character,
    attribute_points: u32,
    focus_points: u32,
}

#[derive(PartialEq, Eq)]
pub enum CreationStage {
    Basic,       // Name, gender, culture
    Attributes,  // Allocate attribute points
    Skills,      // Set skill focus points
    Equipment,   // Initial equipment selection
    Traits,      // Character traits
    Review,      // Review and finalize
}

// Character Creation Plugin
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
               primary_window: Some(Window {
                   title: "Bannerlord Character System".into(),
                   resolution: (1280., 720.).into(),
                   ..default()
               }),
               ..default()
           }))
           .add_plugins(EguiPlugin)
           .init_resource::<CharacterDatabase>()
           .init_resource::<CharacterCreatorState>()
           .add_systems(Startup, setup)
           .add_systems(Update, (character_creator_ui, character_view_ui));
    }
}

// Initialize default resources
impl Default for CharacterDatabase {
    fn default() -> Self {
        Self {
            characters: Vec::new(),
            active_character: None,
        }
    }
}

impl Default for CharacterCreatorState {
    fn default() -> Self {
        Self {
            creation_stage: CreationStage::Basic,
            temp_character: Character::default(),
            attribute_points: 15,
            focus_points: 10,
        }
    }
}

// Character implementation
impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Unnamed".to_string(),
            gender: Gender::Male,
            culture: Culture::Vlandian,
            age: 25,
            clan_level: 0,
            renown: 0.0,
            influence: 0.0,
            gold: 1000,
            attributes: Attributes::default(),
            skills: SkillType::default_skills(),
            equipment: Equipment::default(),
            traits: Vec::new(),
        }
    }
}

impl Default for Attributes {
    fn default() -> Self {
        Self {
            vigor: 2,
            control: 2,
            endurance: 2,
            cunning: 2,
            social: 2,
            intelligence: 2,
        }
    }
}

impl SkillType {
    pub fn default_skills() -> HashMap<SkillType, Skill> {
        let mut skills = HashMap::new();
        
        // Combat skills
        skills.insert(SkillType::OneHanded, Skill::default());
        skills.insert(SkillType::TwoHanded, Skill::default());
        skills.insert(SkillType::Polearm, Skill::default());
        skills.insert(SkillType::Bow, Skill::default());
        skills.insert(SkillType::Crossbow, Skill::default());
        skills.insert(SkillType::Throwing, Skill::default());
        skills.insert(SkillType::Riding, Skill::default());
        skills.insert(SkillType::Athletics, Skill::default());
        
        // Non-combat skills
        skills.insert(SkillType::Crafting, Skill::default());
        skills.insert(SkillType::Tactics, Skill::default());
        skills.insert(SkillType::Scouting, Skill::default());
        skills.insert(SkillType::Roguery, Skill::default());
        skills.insert(SkillType::Charm, Skill::default());
        skills.insert(SkillType::Leadership, Skill::default());
        skills.insert(SkillType::Trade, Skill::default());
        skills.insert(SkillType::Steward, Skill::default());
        skills.insert(SkillType::Medicine, Skill::default());
        skills.insert(SkillType::Engineering, Skill::default());
        
        skills
    }
    
    pub fn get_attribute(&self) -> &'static str {
        match self {
            SkillType::OneHanded | SkillType::TwoHanded | SkillType::Polearm => "vigor",
            SkillType::Bow | SkillType::Crossbow | SkillType::Throwing => "control",
            SkillType::Riding | SkillType::Athletics => "endurance",
            SkillType::Tactics | SkillType::Roguery | SkillType::Scouting => "cunning",
            SkillType::Charm | SkillType::Leadership | SkillType::Trade => "social",
            SkillType::Crafting | SkillType::Steward | SkillType::Medicine | SkillType::Engineering => "intelligence",
        }
    }
}

impl Default for Skill {
    fn default() -> Self {
        Self {
            level: 0,
            focus_points: 0,
            experience: 0.0,
        }
    }
}

impl Default for Equipment {
    fn default() -> Self {
        Self {
            head: None,
            body: None,
            legs: None,
            hands: None,
            main_weapon: None,
            secondary_weapon: None,
            ranged_weapon: None,
            mount: None,
        }
    }
}

// Helper UI functions
fn attribute_selector(ui: &mut egui::Ui, name: &str, value: &mut u32, points: &mut u32, description: &str) {
    ui.horizontal(|ui| {
        ui.label(format!("{}: {}", name, value));
        ui.add_space(10.0);
        
        if *value > 2 && ui.button("-").clicked() {
            *value -= 1;
            *points += 1;
        }
        
        if *points > 0 && ui.button("+").clicked() {
            *value += 1;
            *points -= 1;
        }
        
        ui.label(description);
    });
}

fn skill_selector(ui: &mut egui::Ui, name: &str, skill: &mut Skill, focus_points: &mut u32, description: &str) {
    ui.horizontal(|ui| {
        ui.label(format!("{}: {}", name, skill.level));
        ui.add_space(10.0);
        
        if skill.focus_points > 0 && ui.button("-").clicked() {
            skill.focus_points -= 1;
            *focus_points += 1;
        }
        
        if *focus_points > 0 && ui.button("+").clicked() {
            skill.focus_points += 1;
            *focus_points -= 1;
        }
        
        ui.label(format!("Focus: {}", skill.focus_points));
        ui.add_space(10.0);
        ui.label(description);
    });
}

// Helper functions for character view and testing
fn calculate_character_level(character: &Character) -> u32 {
    // Calculate level based on total skill levels and attribute points
    let skill_sum: u32 = character.skills.values().map(|skill| skill.level).sum();
    let attribute_sum = character.attributes.vigor + 
                        character.attributes.control + 
                        character.attributes.endurance + 
                        character.attributes.cunning + 
                        character.attributes.social + 
                        character.attributes.intelligence;
    
    // Simple formula: (skill_sum / 10) + (attribute_sum * 2)
    (skill_sum / 10) + (attribute_sum * 2)
}

fn display_skill(ui: &mut egui::Ui, name: &str, skill: &Skill) {
    ui.horizontal(|ui| {
        ui.label(format!("{}: {}", name, skill.level));
        if skill.focus_points > 0 {
            ui.label(format!("(Focus: {})", skill.focus_points));
        }
    });
}

fn display_item(ui: &mut egui::Ui, slot: &str, item: &Item) {
    ui.collapsing(format!("{}: {}", slot, item.name), |ui| {
        ui.label(format!("Tier: {} | Value: {} | Weight: {:.1}", 
            item.tier, item.value, item.weight));
        
        match &item.stats {
            ItemStats { armor: Some(armor), .. } => {
                ui.label(format!("Armor: {}", armor));
            },
            ItemStats { damage: Some(damage), speed: Some(speed), handling: Some(handling), range: Some(range), .. } => {
                ui.label(format!("Damage: {} | Speed: {:.0} | Handling: {:.0}", 
                    damage, speed, handling));
                ui.label(format!("Range: {}", range));
            },
            _ => {}
        }
    });
}

fn simulate_battle(character: &Character) -> String {
    // A very simple battle simulation for testing
    let mut rng = rand::thread_rng();
    
    // Calculate character's combat power
    let one_handed = character.skills.get(&SkillType::OneHanded).map_or(0, |s| s.level);
    let two_handed = character.skills.get(&SkillType::TwoHanded).map_or(0, |s| s.level);
    let polearm = character.skills.get(&SkillType::Polearm).map_or(0, |s| s.level);
    let weapon_skill = one_handed.max(two_handed).max(polearm);
    
    let athletics = character.skills.get(&SkillType::Athletics).map_or(0, |s| s.level);
    let tactics = character.skills.get(&SkillType::Tactics).map_or(0, |s| s.level);
    
    // Basic combat formula
    let combat_power = character.attributes.vigor * 5 + 
                        character.attributes.endurance * 3 + 
                        weapon_skill * 2 + 
                        athletics + 
                        tactics;
    
    // Random battle outcome with character's power as weight
    let enemy_power = 100 + rng.gen_range(0..200); // Random enemy strength
    
    if combat_power > enemy_power {
        format!("Victory! Combat power: {} vs Enemy: {}", combat_power, enemy_power)
    } else if combat_power > enemy_power / 2 {
        format!("Narrow defeat. Combat power: {} vs Enemy: {}", combat_power, enemy_power)
    } else {
        format!("Defeat. Combat power: {} vs Enemy: {}", combat_power, enemy_power)
    }
}

// System functions
fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    
    // Create sample traits
    let brave = Trait {
        id: "brave".to_string(),
        name: "Brave".to_string(),
        description: "This character is courageous in battle and faces danger without fear.".to_string(),
        effects: vec![
            TraitEffect {
                target: TraitEffectTarget::Skill(SkillType::Leadership),
                modifier: 0.1,
            },
            TraitEffect {
                target: TraitEffectTarget::TroopMorale,
                modifier: 0.05,
            },
        ],
    };
    
    let merciful = Trait {
        id: "merciful".to_string(),
        name: "Merciful".to_string(),
        description: "This character shows mercy to defeated enemies and spares the innocent.".to_string(),
        effects: vec![
            TraitEffect {
                target: TraitEffectTarget::Relationship("Nobility".to_string()),
                modifier: 0.1,
            },
            TraitEffect {
                target: TraitEffectTarget::Skill(SkillType::Charm),
                modifier: 0.05,
            },
        ],
    };
    
    // Create sample items
    let iron_helmet = Item {
        id: "iron_helmet".to_string(),
        name: "Iron Helmet".to_string(),
        item_type: ItemType::Armor(ArmorType::Head),
        tier: 3,
        value: 550,
        weight: 2.5,
        stats: ItemStats {
            armor: Some(20),
            damage: None,
            speed: None,
            handling: None,
            accuracy: None,
            range: None,
        },
    };
    
    let iron_sword = Item {
        id: "iron_sword".to_string(),
        name: "Iron Sword".to_string(),
        item_type: ItemType::Weapon(WeaponType::OneHanded),
        tier: 2,
        value: 650,
        weight: 1.2,
        stats: ItemStats {
            armor: None,
            damage: Some(29),
            speed: Some(93.0),
            handling: Some(85.0),
            accuracy: None,
            range: Some(100),
        },
    };
    
    // Create a sample character
    let mut equipment = Equipment::default();
    equipment.head = Some(iron_helmet);
    equipment.main_weapon = Some(iron_sword);
    
    let sample_character = Character {
        name: "Ragnar".to_string(),
        gender: Gender::Male,
        culture: Culture::Sturgian,
        age: 32,
        clan_level: 2,
        renown: 150.0,
        influence: 25.0,
        gold: 2500,
        attributes: Attributes {
            vigor: 5,
            control: 3,
            endurance: 4,
            cunning: 3,
            social: 2,
            intelligence: 3,
        },
        skills: {
            let mut skills = SkillType::default_skills();
            skills.get_mut(&SkillType::OneHanded).unwrap().level = 65;
            skills.get_mut(&SkillType::Riding).unwrap().level = 45;
            skills.get_mut(&SkillType::Leadership).unwrap().level = 30;
            skills
        },
        equipment,
        traits: vec![brave, merciful],
    };
    
    // Spawn the sample character
    let entity = commands.spawn(sample_character).id();
    
    // Update character database
    commands.insert_resource(CharacterDatabase {
        characters: vec![entity],
        active_character: Some(entity),
    });
}

fn character_creator_ui(
    mut commands: Commands,
    mut egui_context: Query<&mut EguiContext>,
    mut creator_state: ResMut<CharacterCreatorState>,
    mut char_database: ResMut<CharacterDatabase>,
) {
    let mut context = egui_context.single_mut();
    
    egui::Window::new("Character Creator").show(context.get_mut(), |ui| {
        ui.heading("Create Your Character");
        
        // Navigation controls
        ui.horizontal(|ui| {
            if creator_state.creation_stage != CreationStage::Basic {
                if ui.button("Previous").clicked() {
                    creator_state.creation_stage = match creator_state.creation_stage {
                        CreationStage::Attributes => CreationStage::Basic,
                        CreationStage::Skills => CreationStage::Attributes,
                        CreationStage::Equipment => CreationStage::Skills,
                        CreationStage::Traits => CreationStage::Equipment,
                        CreationStage::Review => CreationStage::Traits,
                        _ => CreationStage::Basic,
                    };
                }
            }
            
            if creator_state.creation_stage != CreationStage::Review {
                if ui.button("Next").clicked() {
                    creator_state.creation_stage = match creator_state.creation_stage {
                        CreationStage::Basic => CreationStage::Attributes,
                        CreationStage::Attributes => CreationStage::Skills,
                        CreationStage::Skills => CreationStage::Equipment,
                        CreationStage::Equipment => CreationStage::Traits,
                        CreationStage::Traits => CreationStage::Review,
                        _ => CreationStage::Review,
                    };
                }
            } else {
                if ui.button("Create Character").clicked() {
                    let character = creator_state.temp_character.clone();
                    let entity = commands.spawn(character).id();
                    char_database.characters.push(entity);
                    char_database.active_character = Some(entity);
                    
                    // Reset the creator state
                    *creator_state = CharacterCreatorState::default();
                }
            }
        });
        
        ui.separator();
        
        // Content based on current stage
        match creator_state.creation_stage {
            CreationStage::Basic => {
                ui.heading("Basic Information");
                
                ui.horizontal(|ui| {
                    ui.label("Name: ");
                    let mut name = creator_state.temp_character.name.clone();
                    if ui.text_edit_singleline(&mut name).changed() {
                        creator_state.temp_character.name = name;
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Gender: ");
                    let male_selected = creator_state.temp_character.gender == Gender::Male;
                    if ui.radio(male_selected, "Male").clicked() {
                        creator_state.temp_character.gender = Gender::Male;
                    }
                    if ui.radio(!male_selected, "Female").clicked() {
                        creator_state.temp_character.gender = Gender::Female;
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Age: ");
                    let mut age = creator_state.temp_character.age;
                    if ui.add(egui::DragValue::new(&mut age).clamp_range(18..=60)).changed() {
                        creator_state.temp_character.age = age;
                    }
                });
                
                ui.horizontal(|ui| {
                    ui.label("Culture: ");
                    egui::ComboBox::from_id_source("culture_select")
                        .selected_text(format!("{:?}", creator_state.temp_character.culture))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut creator_state.temp_character.culture, Culture::Vlandian, "Vlandian");
                            ui.selectable_value(&mut creator_state.temp_character.culture, Culture::Battanian, "Battanian");
                            ui.selectable_value(&mut creator_state.temp_character.culture, Culture::Sturgian, "Sturgian");
                            ui.selectable_value(&mut creator_state.temp_character.culture, Culture::Empire, "Empire");
                            ui.selectable_value(&mut creator_state.temp_character.culture, Culture::Aserai, "Aserai");
                            ui.selectable_value(&mut creator_state.temp_character.culture, Culture::Khuzait, "Khuzait");
                        });
                });
            },
            CreationStage::Attributes => {
                ui.heading("Attributes");
                ui.label(format!("Points remaining: {}", creator_state.attribute_points));
                
                ui.separator();
                
                let attributes = &mut creator_state.temp_character.attributes;
                
                attribute_selector(ui, "Vigor", &mut attributes.vigor, &mut creator_state.attribute_points, 
                                "Physical strength and fighting prowess.");
                
                attribute_selector(ui, "Control", &mut attributes.control, &mut creator_state.attribute_points,
                                "Precision, accuracy, and dexterity.");
                
                attribute_selector(ui, "Endurance", &mut attributes.endurance, &mut creator_state.attribute_points,
                                "Health, stamina, and physical resilience.");
                
                attribute_selector(ui, "Cunning", &mut attributes.cunning, &mut creator_state.attribute_points,
                                "Tactical thinking, stealth, and craftiness.");
                
                attribute_selector(ui, "Social", &mut attributes.social, &mut creator_state.attribute_points,
                                "Charisma, leadership, and interpersonal skills.");
                
                attribute_selector(ui, "Intelligence", &mut attributes.intelligence, &mut creator_state.attribute_points,
                                "Learning, problem-solving, and knowledge.");
            },
            CreationStage::Skills => {
                ui.heading("Skills");
                ui.label(format!("Focus points remaining: {}", creator_state.focus_points));
                
                ui.separator();
                ui.heading("Combat Skills");
                
                let skills = &mut creator_state.temp_character.skills;
                
                skill_selector(ui, "One-Handed", skills.get_mut(&SkillType::OneHanded).unwrap(), 
                            &mut creator_state.focus_points, "Proficiency with one-handed weapons.");
                
                skill_selector(ui, "Two-Handed", skills.get_mut(&SkillType::TwoHanded).unwrap(),
                            &mut creator_state.focus_points, "Proficiency with two-handed weapons.");
                
                skill_selector(ui, "Polearm", skills.get_mut(&SkillType::Polearm).unwrap(),
                            &mut creator_state.focus_points, "Proficiency with polearms and spears.");
                
                skill_selector(ui, "Bow", skills.get_mut(&SkillType::Bow).unwrap(),
                            &mut creator_state.focus_points, "Proficiency with bows and archery.");
                
                skill_selector(ui, "Crossbow", skills.get_mut(&SkillType::Crossbow).unwrap(),
                            &mut creator_state.focus_points, "Proficiency with crossbows.");
                
                skill_selector(ui, "Throwing", skills.get_mut(&SkillType::Throwing).unwrap(),
                            &mut creator_state.focus_points, "Proficiency with throwing weapons.");
                
                skill_selector(ui, "Riding", skills.get_mut(&SkillType::Riding).unwrap(),
                            &mut creator_state.focus_points, "Horse riding and mounted combat skills.");
                
                skill_selector(ui, "Athletics", skills.get_mut(&SkillType::Athletics).unwrap(),
                            &mut creator_state.focus_points, "Movement speed and stamina on foot.");
                
                ui.separator();
                ui.heading("Non-Combat Skills");
                
                skill_selector(ui, "Crafting", skills.get_mut(&SkillType::Crafting).unwrap(),
                            &mut creator_state.focus_points, "Ability to craft and improve weapons.");
                
                skill_selector(ui, "Tactics", skills.get_mut(&SkillType::Tactics).unwrap(),
                            &mut creator_state.focus_points, "Battle tactics and strategic planning.");
                
                skill_selector(ui, "Scouting", skills.get_mut(&SkillType::Scouting).unwrap(),
                            &mut creator_state.focus_points, "Tracking, spotting, and reconnaissance.");
                
                skill_selector(ui, "Roguery", skills.get_mut(&SkillType::Roguery).unwrap(),
                            &mut creator_state.focus_points, "Stealth, criminal activities, and subterfuge.");
                
                skill_selector(ui, "Charm", skills.get_mut(&SkillType::Charm).unwrap(),
                            &mut creator_state.focus_points, "Persuasion and social influence.");
                
                skill_selector(ui, "Leadership", skills.get_mut(&SkillType::Leadership).unwrap(),
                            &mut creator_state.focus_points, "Ability to lead troops and command respect.");
                
                skill_selector(ui, "Trade", skills.get_mut(&SkillType::Trade).unwrap(),
                            &mut creator_state.focus_points, "Commercial acumen and bargaining skills.");
                
                skill_selector(ui, "Steward", skills.get_mut(&SkillType::Steward).unwrap(),
                            &mut creator_state.focus_points, "Management of settlements and resources.");
                
                skill_selector(ui, "Medicine", skills.get_mut(&SkillType::Medicine).unwrap(),
                            &mut creator_state.focus_points, "Treatment of wounds and health management.");
                
                skill_selector(ui, "Engineering", skills.get_mut(&SkillType::Engineering).unwrap(),
                            &mut creator_state.focus_points, "Construction and siege capabilities.");
            },
            CreationStage::Equipment => {
                ui.heading("Starting Equipment");
                
                // In a real implementation, this would show selectable equipment
                ui.label("Equipment selection would go here");
                
                // For simplicity, we'll just add some default equipment
                let mut equipment = &mut creator_state.temp_character.equipment;
                
                if equipment.head.is_none() && ui.button("Add Iron Helmet").clicked() {
                    equipment.head = Some(Item {
                        id: "iron_helmet".to_string(),
                        name: "Iron Helmet".to_string(),
                        item_type: ItemType::Armor(ArmorType::Head),
                        tier: 3,
                        value: 550,
                        weight: 2.5,
                        stats: ItemStats {
                            armor: Some(20),
                            damage: None,
                            speed: None,
                            handling: None,
                            accuracy: None,
                            range: None,
                        },
                    });
                }
                
                if equipment.main_weapon.is_none() && ui.button("Add Iron Sword").clicked() {
                    equipment.main_weapon = Some(Item {
                        id: "iron_sword".to_string(),
                        name: "Iron Sword".to_string(),
                        item_type: ItemType::Weapon(WeaponType::OneHanded),
                        tier: 2,
                        value: 650,
                        weight: 1.2,
                        stats: ItemStats {
                            armor: None,
                            damage: Some(29),
                            speed: Some(93.0),
                            handling: Some(85.0),
                            accuracy: None,
                            range: Some(100),
                        },
                    });
                }
            },
            CreationStage::Traits => {
                ui.heading("Character Traits");
                
                // In a real implementation, this would be a proper trait selection system
                ui.label("Select character traits that define your personality");
                
                let traits = &mut creator_state.temp_character.traits;
                
                let has_brave = traits.iter().any(|t| t.id == "brave");
                if ui.checkbox(&mut has_brave.clone(), "Brave - Courageous in battle").changed() {
                    if has_brave {
                        traits.push(Trait {
                            id: "brave".to_string(),
                            name: "Brave".to_string(),
                            description: "This character is courageous in battle and faces danger without fear.".to_string(),
                            effects: vec![
                                TraitEffect {
                                    target: TraitEffectTarget::Skill(SkillType::Leadership),
                                    modifier: 0.1,
                                },
                                TraitEffect {
                                    target: TraitEffectTarget::TroopMorale,
                                    modifier: 0.05,
                                },
                            ],
                        });
                    } else {
                        traits.retain(|t| t.id != "brave");
                    }
                }
                
                let has_merciful = traits.iter().any(|t| t.id == "merciful");
                if ui.checkbox(&mut has_merciful.clone(), "Merciful - Shows mercy to defeated enemies").changed() {
                    if has_merciful {
                        traits.push(Trait {
                            id: "merciful".to_string(),
                            name: "Merciful".to_string(),
                            description: "This character shows mercy to defeated enemies and spares the innocent.".to_string(),
                            effects: vec![
                                TraitEffect {
                                    target: TraitEffectTarget::Relationship("Nobility".to_string()),
                                    modifier: 0.1,
                                },
                                TraitEffect {
                                    target: TraitEffectTarget::Skill(SkillType::Charm),
                                    modifier: 0.05,
                                },
                            ],
                        });
                    } else {
                        traits.retain(|t| t.id != "merciful");
                    }
                }
            },
            CreationStage::Review => {
                ui.heading("Character Review");
                
                let character = &creator_state.temp_character;
                
                ui.heading(&character.name);
                ui.label(format!("Gender: {:?}", character.gender));
                ui.label(format!("Culture: {:?}", character.culture));
                ui.label(format!("Age: {}", character.age));
                
                ui.separator();
                ui.heading("Attributes");
                
                ui.label(format!("Vigor: {}", character.attributes.vigor));
                ui.label(format!("Control: {}", character.attributes.control));
                ui.label(format!("Endurance: {}", character.attributes.endurance));
                ui.label(format!("Cunning: {}", character.attributes.cunning));
                ui.label(format!("Social: {}", character.attributes.social));
                ui.label(format!("Intelligence: {}", character.attributes.intelligence));
                
                ui.separator();
                ui.heading("Top Skills");
                
                let mut skills: Vec<_> = character.skills.iter().collect();
                skills.sort_by(|a, b| b.1.level.cmp(&a.1.level));
                
                for (skill_type, skill) in skills.iter().take(5) {
                    ui.label(format!("{:?}: {} (Focus: {})", skill_type, skill.level, skill.focus_points));
                }
                
                ui.separator();
                ui.heading("Traits");
                
                for trait_item in &character.traits {
                    ui.label(&trait_item.name);
                    ui.label(&trait_item.description);
                    ui.separator();
                }
            },
        }
    });
}