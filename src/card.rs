//! Implements the `Card` structure.
use serde::Deserialize;

/// The struct holding all the card attributes found in the json files created by `tagger.py`
#[derive(Debug, Deserialize)]
pub struct Card {
    pub craft_: String,
    pub id_: u32,
    pub pp_: u8,
    pub trait_: String,
    pub type_: String,
    pub tags_: Vec<String>,
}
