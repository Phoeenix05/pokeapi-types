use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[cfg(test)]
use ts_rs::TS;

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Pokemon {
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
    #[cfg_attr(test, ts(type = "string[]"))]
    pub past_types: Vec<Option<serde_json::Value>>,
    pub species: Species,
    pub sprites: Box<Sprites>,
    pub stats: Vec<Stat>,
    pub types: Vec<Type>,
    pub weight: i64,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Ability {
    pub ability: Species,
    pub is_hidden: bool,
    pub slot: i64,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Species {
    pub name: String,
    pub url: String,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct GameIndex {
    pub game_index: i64,
    pub version: Species,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct HeldItem {
    pub item: Species,
    pub version_details: Vec<VersionDetail>,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionDetail {
    pub rarity: i64,
    pub version: Species,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Move {
    #[serde(rename = "move")]
    pub move_move: Species,
    pub version_group_details: Vec<VersionGroupDetail>,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionGroupDetail {
    pub level_learned_at: i64,
    pub move_learn_method: Species,
    pub version_group: Species,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GenerationV {
    pub black_white: Box<Sprites>,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GenerationIv {
    pub diamond_pearl: Box<Sprites>,
    pub heartgold_soulsilver: Box<Sprites>,
    pub platinum: Box<Sprites>,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
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

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Sprites {
    pub back_default: String,
    #[cfg_attr(test, ts(type = "string"))]
    pub back_female: Option<serde_json::Value>,
    pub back_shiny: String,
    #[cfg_attr(test, ts(type = "string"))]
    pub back_shiny_female: Option<serde_json::Value>,
    pub front_default: String,
    #[cfg_attr(test, ts(type = "string"))]
    pub front_female: Option<serde_json::Value>,
    pub front_shiny: String,
    #[cfg_attr(test, ts(type = "string"))]
    pub front_shiny_female: Option<serde_json::Value>,
    pub other: Option<Other>,
    pub versions: Option<Versions>,
    pub animated: Option<Box<Sprites>>,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GenerationI {
    pub red_blue: RedBlue,
    pub yellow: RedBlue,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct RedBlue {
    pub back_default: String,
    pub back_gray: String,
    pub back_transparent: String,
    pub front_default: String,
    pub front_gray: String,
    pub front_transparent: String,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationIi {
    pub crystal: Crystal,
    pub gold: Gold,
    pub silver: Gold,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
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

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Gold {
    pub back_default: String,
    pub back_shiny: String,
    pub front_default: String,
    pub front_shiny: String,
    pub front_transparent: Option<String>,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GenerationIii {
    pub emerald: OfficialArtwork,
    pub firered_leafgreen: Gold,
    pub ruby_sapphire: Gold,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct OfficialArtwork {
    pub front_default: String,
    pub front_shiny: String,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Home {
    pub front_default: String,
    #[cfg_attr(test, ts(type = "string"))]
    pub front_female: Option<serde_json::Value>,
    pub front_shiny: String,
    #[cfg_attr(test, ts(type = "string"))]
    pub front_shiny_female: Option<serde_json::Value>,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GenerationVii {
    pub icons: DreamWorld,
    pub ultra_sun_ultra_moon: Home,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct DreamWorld {
    pub front_default: String,
    #[cfg_attr(test, ts(type = "string"))]
    pub front_female: Option<serde_json::Value>,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct GenerationViii {
    pub icons: DreamWorld,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Other {
    pub dream_world: DreamWorld,
    pub home: Home,
    #[serde(rename = "official-artwork")]
    pub official_artwork: OfficialArtwork,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    pub base_stat: i64,
    pub effort: i64,
    pub stat: Species,
}

#[cfg_attr(test, derive(TS))]
#[cfg_attr(test, ts(export))]
#[derive(Debug, Serialize, Deserialize)]
pub struct Type {
    pub slot: i64,
    #[serde(rename = "type")]
    pub type_type: Species,
}
