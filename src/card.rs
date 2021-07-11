use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CardInfo {
    pub craft_: String,
    pub id_: i32,
    pub pp_: u8,
    pub trait_: String,
    pub type_: String,
    pub rotation_: bool,
    pub name_: String,
    pub tags_: Vec<String>,
}