use cw_storage_plus::Item;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

//Trait structure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Trait {
    pub id: u32,
    pub trait_type: String,
    pub trait_value: String,
    pub trait_rarity: String,
}

//Premade Character structure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Character {
    pub id: u32,
    pub ears: Option<String>,
    pub eyes: Option<String>,
    pub mouth: Option<String>,
    pub fur_type: Option<String>,
    pub fur_color: Option<String>,
    pub tail_shape: Option<String>,
    pub rarity: String,
}

//Trait bundle structure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TraitBundle {
    pub id: u32,
    pub traits: Vec<Trait>,
    pub total_discount_percentage: u32,
}

//Character bundle structure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CharacterBundle {
    pub id: u32,
    pub characters: Vec<Character>,
    pub total_discount_percentage: u32,
}

//Trait Lootbox structure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TraitLootbox {
    pub id: u32,
    pub traits: Vec<Trait>,
    pub possibilities: Vec<u32>,
}

//Character Lootbox structure
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CharacterLootbox {
    pub id: u32,
    pub characters: Vec<Character>,
    pub possibilities: Vec<u32>,
}

pub const TRAITS: Item<Vec<Trait>> = Item::new("traits");
pub const CHARACTERS: Item<Vec<Character>> = Item::new("characters");
pub const TRAIT_BUNDLES: Item<Vec<TraitBundle>> = Item::new("trait_bundle");
pub const CHARACTER_BUNDLES: Item<Vec<CharacterBundle>> = Item::new("character_bundle");
pub const TRAIT_LOOTBOXES: Item<Vec<TraitLootbox>> = Item::new("trait_lootboxes");
pub const CHARACTER_LOOTBOXES: Item<Vec<CharacterLootbox>> = Item::new("character_lootboxes");
