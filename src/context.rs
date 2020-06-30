//! Implements the `Context` structure.

use std::collections::HashMap;

use serde_json;

use crate::card::Card;
use crate::globals;

///Contains the "global" variables that depend on user input.
pub struct Context {
    /// Following [shadowverse-portal](https://shadowverse-portal.com)'s format:
    /// * 1 = Unlimited
    /// * 3 = Rotation
    pub game_mode: u8,
    /// Used as is, or for indexing [CRAFTS](../globals/constant.CRAFTS.html).
    pub craft: usize,
    /// Maps a card name to the card's attributes.
    pub card_library: HashMap<String, Card>,
    /// A list of card names. Although you can get such a list from `card_library`, `card_list` is
    /// important to have as it undergoes sorting.
    pub card_list: Vec<String>,
    /// The list of tags to be considered for the [fitness function](../deck/struct.Deck.html#method.rate).
    pub tags: Vec<String>,
}

/// Returns the `card_library` associated with the context passed.
/// # Note
///
/// This never amounts to the whole legal cardpool, as neutrals have their own context. While
/// treating neutrals as always part of the context would make sense, in this more general form one
/// can choose not to load neutrals, or to load a mixed class cardpool.
/// # Arguments
/// * `ctx` - (`game_mode`, `craft`) as used in [Context](struct.Context.html).
fn get_card_library(ctx: (u8, usize)) -> HashMap<String, Card> {
    let f = globals::ASSETS[9 * (ctx.0 as usize % 3) + ctx.1];
    serde_json::from_str(&f).unwrap()
}

/// Sorts `card_library` by the specified card attributes.
/// # Note
///
/// * This sorting phase encourages the creation of low-order schemata (in this case,
/// synergistic card packages) by clumping similar cards in the same genetic locus.
/// * It also has to do with the usefulness of establishing a temperature parameter in the
/// [mutation phase](../deck/struct.Deck.html#method.mutate).
// TODO: add more sorts relevant to correlating similarity and proximity.
fn deep_sort(lib: &HashMap<String, Card>) -> Vec<String> {
    let mut result = vec![];
    for card in lib.iter() {
        result.push(String::from(card.0));
    }
    result.sort_by_key(|k| &lib[k].id_);
    result.sort_by_key(|k| &lib[k].trait_);
    result.sort_by_key(|k| &lib[k].craft_);
    result.sort_by_key(|k| &lib[k].type_);
    result.sort_by_key(|k| &lib[k].tags_);
    result.sort_by_key(|k| &lib[k].pp_);
    result
}

impl Context {
    /// Loads the Unlimited Forestcraft cardpool, used to skip user input during tests.
    #[allow(dead_code)]
    pub fn from_debug() -> Context {
        let mut card_library = HashMap::new();
        card_library.extend(get_card_library((1, 0))); // Neutrals
        card_library.extend(get_card_library((1, 1)));
        let card_list = deep_sort(&card_library);
        Context {
            game_mode: 1,
            craft: 1,
            card_library,
            card_list,
            tags: Vec::new(),
        }
    }
    /// Builds the context from the game mode and craft specified by the user.
    pub fn from_input() -> Context {
        use crate::generics;
        let game_mode = 3 / generics::dialogue(
            "What game format would you like to build a deck for? \
                1 for Rotation, 2 for Unlimited (1/2):",
            (1, 2),
        ) as u8;
        let craft = generics::dialogue(
            "1 - Forestcraft\t5 - Shadowcraft\n\
                              2 - Swordcraft\t6 - Bloodcraft\n\
                              3 - Runecraft\t7 - Havencraft\n\
                              4 - Dragoncraft\t8 - Portalcraft\n\
                              Which class would you like to use? (options above) [1...8]: ",
            (1, 8),
        );
        let mut card_library: HashMap<String, Card> = HashMap::new();
        card_library.extend(get_card_library((game_mode, 0))); //Neutrals
        card_library.extend(get_card_library((game_mode, craft)));
        let card_list = deep_sort(&card_library);

        let mut available_tags = Vec::new();
        let mut prompt = String::new();
        prompt += "0 - done choosing\n";
        for card in &card_list {
            for tag in &card_library[card].tags_ {
                if !available_tags.contains(&tag) {
                    available_tags.push(tag);
                    prompt += format!("{} - {}\n", available_tags.len(), tag).as_str();
                }
            }
        }
        prompt += "What archetypes would you like to build? (options above): ";
        let prompt = prompt.as_str();
        let mut tags = Vec::new();
        loop {
            let input = generics::dialogue(prompt, (0, available_tags.len() as u8));
            if input == 0 {
                break;
            } else if tags.contains(available_tags[input - 1]) {
                tags.remove(
                    tags.iter()
                        .position(|t| t.eq(available_tags[input - 1]))
                        .unwrap(),
                );
            } else {
                tags.push(available_tags[input - 1].to_string());
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
    println!("{:?}", ctx.card_list);
    assert_eq!(ctx.card_library["Robogoblin"].pp_, 2);
    assert!(ctx.card_library[&ctx.card_list[0]].pp_ < ctx.card_library[&ctx.card_list[50]].pp_);
}
