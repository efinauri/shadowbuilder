//! Implements the `Card` structure.

use serde_json;

/// Card attributes that change before and after evolving.
#[derive(Debug)]
pub struct _FormData {
    pub description: String,
    pub attack: u8,
    pub defense: u8,
}

/// The struct holding all the card attributes found in the json files created by `tagger.py`
#[derive(Debug)]
pub struct Card {
    pub base_data: _FormData,
    pub evo_data: _FormData,
    pub expansion: String,
    /// e.g. `Neutral`
    pub faction: String,
    pub id: u32,
    pub mana_cost: u8,
    /// e.g. `Machina`
    pub race: String,
    pub rot: String,
    /// e.g. `Follower`
    pub _type: String,
    /// e.g. `['mc']`
    pub tags: Vec<String>,
}

impl Card {
    /// # Arguments
    /// * `card` - When a json created by `tagger.py` gets parsed into the default enum Value
    /// `cards_data`, by calling `cards_data.as_object()` one can get a Map binding a card's name
    /// to a `Value` storing that card's attributes, which is what this function is taking in.
    pub fn from_value(card: &serde_json::Value) -> Card {
        let data_tags = card["tags"].as_array().unwrap();
        let mut card_library_tags: Vec<String> = vec![];
        for tag in data_tags.iter() {
            card_library_tags.push(String::from(tag.as_str().unwrap()));
        }
        Card {
            base_data: _FormData {
                description: String::from(card["baseData"]["description"].as_str()
                    .unwrap()),
                attack: card["baseData"]["attack"].as_u64().unwrap() as u8,
                defense: card["baseData"]["defense"].as_u64().unwrap() as u8,
            },
            evo_data: _FormData {
                description: String::from(card["evoData"]["description"].as_str()
                    .unwrap()),
                attack: card["evoData"]["attack"].as_u64().unwrap() as u8,
                defense: card["evoData"]["defense"].as_u64().unwrap() as u8,
            },
            expansion: String::from(card["expansion"].as_str().unwrap()),
            faction: String::from(card["faction"].as_str().unwrap()),
            id: String::from(card["id"].as_str().unwrap()).trim().parse()
                .unwrap(),
            mana_cost: card["manaCost"].as_u64().unwrap() as u8,
            race: String::from(card["race"].as_str().unwrap()),
            rot: String::from(card["rot"].as_str().unwrap()),
            _type: String::from(card["type"].as_str().unwrap()),
            tags: card_library_tags,
        }
    }
}
