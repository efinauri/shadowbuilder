//! Implements the `Context` structure.

use serde_json;
use std::{collections::HashMap, path};
use crate::card::Card;
use crate::globals;

///Contains the "global" variables that depend on user input.
pub struct Context {
    pub game_mode: u8,
    /// Used as is, or for indexing `globals::CRAFTS`.
    pub craft: usize,
    /// Maps a card name to that card's attributes.
    pub card_library: HashMap<String, Card>,
    /// A list of card names. Although you can get such a list from `card_library`, `card_list` is
    /// useful to have as it undergoes sorting.
    pub card_list: Vec<String>,
    pub tags: Vec<String>,
}

/// Returns home/.../directory_of_the_executable/assets/craft_data.json
/// # Arguments
/// * `ctx` - (`game_mode`, `craft`) as used in `Context`.
fn get_path(ctx: (u8, usize)) -> path::PathBuf {
    use std::env;
    let mut path = env::current_dir()
        .unwrap();
    path.push(format!("assets/{}_{}.json", ctx.0, globals::CRAFTS[ctx.1]));
    path
}

/// Returns the `card_library` associated with the context passed.
/// # Note
///
/// This never amounts to the whole legal cardpool, as neutrals have their own context. While
/// treating neutrals as always part of the context would make sense, in this more general form you
/// can choose not to load neutrals, or to load a mixed class cardpool.
/// # Arguments
/// * `ctx` - (`game_mode`, `craft`) as used in `Context`.
fn get_card_library(ctx: (u8, usize)) -> HashMap<String, Card> {
    use std::fs;
    let f = fs::File::open(get_path(ctx))
        .expect(format!("{:?} not found. \nUse `cargo run` from the directory containing /assets.",
                        get_path(ctx)).as_str());
    // TODO: call tagger.py instead of failing
    let data: serde_json::Value = serde_json::from_reader(f).unwrap();
    let data = data.as_object().unwrap();
    let mut ctx_card_library = HashMap::new();
    for card in data.keys() {
        ctx_card_library.insert(String::from(card), Card::from_value(&data[card]));
    }
    ctx_card_library
}

/// Sorts `card_library` by the specified card attributes.
/// # Note
///
/// * This sorting phase encourages the creation of low-order schemata (in this case,
/// synergistic card packages) by clumping similar cards in the same genetic locus.
fn deep_sort(lib: &HashMap<String, Card>) -> Vec<String> {
    // TODO: pass tags as arguments and add (*) to doc.
    let mut result = vec![];
    for card in lib.iter() {
        result.push(String::from(card.0));
    }
    // (*) The sorts go from least important to most important.
    result.sort_by_key(|k| &lib[k].tags);
    result.sort_by_key(|k| &lib[k]._type);
    result.sort_by_key(|k| &lib[k].mana_cost);
    result
}

impl Context {
    /// Loads the Unlimited Forestcraft cardpool, used to skip user input during tests.
    pub fn from_debug() -> Context {
        let mut card_library = HashMap::new();
        card_library.extend(get_card_library((3, 0))); // Neutrals
        card_library.extend(get_card_library((3, 1)));
        let card_list = deep_sort(&card_library);
        Context {
            game_mode: 3,
            craft: 1,
            card_library,
            card_list,
            tags: Vec::new(),
        }
    }
    /// Builds the context from the game mode and craft specified by the user.
    pub fn from_input() -> Context {
        use crate::generics;
        // Following shadowverse-portal's format: rot = 3, ul = 1 (hence the 3/ below).
        let game_mode = 3 / generics::dialogue(
            "What game format would you like to build a deck for? \
                1 for Rotation, 2 for Unlimited (1/2):",
            (1, 2)) as u8;
        let craft = generics::dialogue(
            "1 - Forestcraft\t5 - Shadowcraft\n\
                              2 - Swordcraft\t6 - Bloodcraft\n\
                              3 - Runecraft\t7 - Havencraft\n\
                              4 - Dragoncraft\t8 - Portalcraft\n\
                              Which class would you like to use? (options above) [1...8]: ",
            (1, 8));
        let mut card_library: HashMap<String, Card> = HashMap::new();
        card_library.extend(get_card_library((game_mode, 0))); //Neutrals
        card_library.extend(get_card_library((game_mode, craft)));
        let card_list = deep_sort(&card_library);
        let mut available_tags = Vec::new();
        let mut prompt = String::new();
        prompt += "0 - done chosing\n";
        for card in &card_list {
            for tag in &card_library[card].tags {
                if !available_tags.contains(&tag){
                    available_tags.push(tag);
                    prompt += format!("{} - {}\n", available_tags.len(), tag).as_str();
                }
            }
        }
        prompt += "What archetypes would you like to build? (options above): ";
        let prompt = prompt.as_str();
        let mut tags = Vec::new();
        loop {
            let input = generics::dialogue(prompt,(0, available_tags.len() as u8));
            match input {
                0 => break,
                _=> if tags.contains(available_tags[input - 1]) {
                    tags.remove(tags.iter().position(|t|t.eq(available_tags[input - 1])).unwrap());
                }
                else {
                    tags.push(available_tags[input - 1].to_string());
                },
            }
            println!("Chosen tags: {:?}\n Choose a tag again to remove it.", tags);
        }
        Context {
            game_mode,
            craft,
            card_library,
            card_list,
            tags,
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    let ctx = Context::from_debug();
    assert_eq!(globals::CRAFTS[ctx.craft], "Forestcraft");
    assert_eq!(ctx.card_library["Robogoblin"].faction, "Neutral");
    assert!(ctx.card_library[&ctx.card_list[0]].mana_cost <
        ctx.card_library[&ctx.card_list[50]].mana_cost);
}