use serde::{Deserialize, Serialize};
use ts_rs::TS;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Manga {
    pub abilities: Vec<Ability>,
    pub base_experience: i64,
    pub forms: Vec<Species>,
    pub game_indices: Vec<GameIndex>,
    pub height: i64,
    pub held_items: Vec<HeldItem>,
    pub id: i64,
    pub is_default: bool,
    pub location_area_encounters: String,
    pub moves: Vec<Move>,
    pub name: String,
    pub order: i64,
    #[ts(type = "string[]")]
    pub past_types: Vec<Option<serde_json::Value>>,
    pub species: Species,
    pub sprites: Box<Sprites>,
    pub stats: Vec<Stat>,
    pub types: Vec<Type>,
    pub weight: i64,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Ability {
    pub ability: Species,
    pub is_hidden: bool,
    pub slot: i64,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Species {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GameIndex {
    pub game_index: i64,
    pub version: Species,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HeldItem {
    pub item: Species,
    pub version_details: Vec<VersionDetail>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct VersionDetail {
    pub rarity: i64,
    pub version: Species,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Move {
    #[serde(rename = "move")]
    pub move_move: Species,
    pub version_group_details: Vec<VersionGroupDetail>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct VersionGroupDetail {
    pub level_learned_at: i64,
    pub move_learn_method: Species,
    pub version_group: Species,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "kebab-case")]
pub struct GenerationV {
    pub black_white: Box<Sprites>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "kebab-case")]
pub struct GenerationIv {
    pub diamond_pearl: Box<Sprites>,
    pub heartgold_soulsilver: Box<Sprites>,
    pub platinum: Box<Sprites>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "kebab-case")]
pub struct Versions {
    pub generation_i: GenerationI,
    pub generation_ii: GenerationIi,
    pub generation_iii: GenerationIii,
    pub generation_iv: GenerationIv,
    pub generation_v: GenerationV,
    pub generation_vi: HashMap<String, Home>,
    pub generation_vii: GenerationVii,
    pub generation_viii: GenerationViii,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Sprites {
    pub back_default: String,
    #[ts(type = "string")]
    pub back_female: Option<serde_json::Value>,
    pub back_shiny: String,
    #[ts(type = "string")]
    pub back_shiny_female: Option<serde_json::Value>,
    pub front_default: String,
    #[ts(type = "string")]
    pub front_female: Option<serde_json::Value>,
    pub front_shiny: String,
    #[ts(type = "string")]
    pub front_shiny_female: Option<serde_json::Value>,
    pub other: Option<Other>,
    pub versions: Option<Versions>,
    pub animated: Option<Box<Sprites>>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "kebab-case")]
pub struct GenerationI {
    pub red_blue: RedBlue,
    pub yellow: RedBlue,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RedBlue {
    pub back_default: String,
    pub back_gray: String,
    pub back_transparent: String,
    pub front_default: String,
    pub front_gray: String,
    pub front_transparent: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GenerationIi {
    pub crystal: Crystal,
    pub gold: Gold,
    pub silver: Gold,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Crystal {
    pub back_default: String,
    pub back_shiny: String,
    pub back_shiny_transparent: String,
    pub back_transparent: String,
    pub front_default: String,
    pub front_shiny: String,
    pub front_shiny_transparent: String,
    pub front_transparent: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Gold {
    pub back_default: String,
    pub back_shiny: String,
    pub front_default: String,
    pub front_shiny: String,
    pub front_transparent: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "kebab-case")]
pub struct GenerationIii {
    pub emerald: OfficialArtwork,
    pub firered_leafgreen: Gold,
    pub ruby_sapphire: Gold,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct OfficialArtwork {
    pub front_default: String,
    pub front_shiny: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Home {
    pub front_default: String,
    #[ts(type = "string")]
    pub front_female: Option<serde_json::Value>,
    pub front_shiny: String,
    #[ts(type = "string")]
    pub front_shiny_female: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "kebab-case")]
pub struct GenerationVii {
    pub icons: DreamWorld,
    pub ultra_sun_ultra_moon: Home,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DreamWorld {
    pub front_default: String,
    #[ts(type = "string")]
    pub front_female: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GenerationViii {
    pub icons: DreamWorld,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Other {
    pub dream_world: DreamWorld,
    pub home: Home,
    #[serde(rename = "official-artwork")]
    pub official_artwork: OfficialArtwork,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Stat {
    pub base_stat: i64,
    pub effort: i64,
    pub stat: Species,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Type {
    pub slot: i64,
    #[serde(rename = "type")]
    pub type_type: Species,
}
