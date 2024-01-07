use std::collections::HashSet;
use std::io;

use fnv::FnvHashMap;

use crate::card::CardInfo;

fn clear_term() {
    print!("{}[2J", 27 as char);
}

fn dialogue<F, G>(prompt: G, selection_len: usize, max_range: usize, mut break_condition: F,
                  prompt_elements: &mut HashSet<String>, ) -> usize
    where F: FnMut(usize, &mut HashSet<String>) -> bool, G: Fn(&HashSet<String>) -> String {
    clear_term();
    let mut input = String::new();
    loop {
        input.clear();
        println!("{} [{}/{}]: ", prompt(prompt_elements), selection_len, max_range);
        io::stdin().read_line(&mut input).unwrap();
        if let Ok(num) = input.trim().parse() {
            if num <= max_range && break_condition(num, prompt_elements) { return num; }
        } else {
            clear_term();
            println!("Invalid input.");
        }
    }
}

fn simple_dialogue(prompt: &str, selection_len: usize, max_range: usize) -> usize {
    dialogue(
        |_| prompt.to_string(),
        selection_len,
        max_range,
        |_, _| true,
        &mut HashSet::new(),
    )
}

const CRAFTS: [&str; 8] = [
    "Forestcraft", "Swordcraft",
    "Runecraft", "Dragoncraft",
    "Shadowcraft", "Bloodcraft",
    "Havencraft", "Portalcraft"];

// Card ID to card info mapping.
// Fnv is preferred to std::collections::HashMap because the speed-for-safety tradeoff that the
// latter makes isn't needed.
pub struct CardsMap(pub FnvHashMap<i32, CardInfo>);

impl CardsMap {
    pub fn from_input(game_mode: usize, craft_n: usize) -> Result<CardsMap, io::Error> {
        let cards = include_str!("assets/cards.json");
        let mut cards: FnvHashMap<i32, CardInfo> = serde_json::from_str(&cards)?;
        //Filtering:
        cards.retain(|_id, card|
            //Filtering invalid crafts.
            (card.craft_ == CRAFTS[craft_n] || card.craft_ == "Neutral")
                // Filtering rotated cards if the game mode is Rotation.
                && ((game_mode == 1) || card.rotation_));
        Ok(CardsMap(cards))
    }
}

// Vector of card IDs.
pub struct CardsVec(pub Vec<i32>);

impl CardsVec {
    pub fn from_dict(data: &CardsMap) -> Self {
        let cards = data.0.iter().map(|(id, _)| *id).collect();
        let mut ret = CardsVec(cards);
        ret.sort(&data);
        ret
    }

    // This sorting phase encourages the creation of low-order schemata (in this case,
    // synergistic card packages) by clumping "similar" cards in the same genetic locus.

    fn sort(&mut self, data: &CardsMap) {
        self.0.sort_unstable_by_key(|c| data.0.get(c).unwrap().id_);
        self.0.sort_by_key(|c| data.0.get(c).unwrap().trait_.to_string());
        self.0.sort_by_key(|c| data.0.get(c).unwrap().craft_.to_string());
        self.0.sort_by_key(|c| data.0.get(c).unwrap().type_.to_string());
        self.0.sort_by_key(|c| &data.0.get(c).unwrap().tags_);
        self.0.sort_by_key(|c| data.0.get(c).unwrap().pp_);
    }
}

// A bundle of variables set by the user at runtime.
pub struct Context {
    pub cards_map: CardsMap,
    pub cards_vec: CardsVec,
    pub game_mode: usize,
    pub craft: usize,
    pub tags: Vec<String>,
}

fn get_archetype_tags(cm: &CardsMap) -> Vec<String> {
    let mut available_tags = HashSet::new();
    for (_, info) in &cm.0 {
        if &info.craft_ == "Neutral" { continue; }
        for tag in &info.tags_ {
            available_tags.insert(tag.clone());
        }
    }
    available_tags.into_iter().collect()
}

fn build_archetype_tags_prompt(avl_tags: &Vec<String>) -> String {
    let mut prompt = String::from("0 - done choosing\n");
    for (i, avl_tag) in avl_tags.iter().enumerate() {
        prompt += format!("{} - {}\n", i + 1, avl_tag).as_str();
    }
    prompt += "What archetypes would you like to build? (options above) ";
    prompt
}

impl Context {
    pub fn from_input() -> Self {
        let game_mode = simple_dialogue(
            "What game format would you like to build a deck for? \
                0 for Rotation, 1 for Unlimited",
            0, 1);

        let craft = simple_dialogue(
            "0 - Forestcraft\t4 - Shadowcraft\n\
                      1 - Swordcraft\t5 - Bloodcraft\n\
                      2 - Runecraft\t6 - Havencraft\n\
                      3 - Dragoncraft\t7 - Portalcraft\n\
                      Which class would you like to use? (options above)",
            0, 7);

        let cards_map = CardsMap::from_input(game_mode, craft).unwrap();
        let available_tags = get_archetype_tags(&cards_map);

        let prompt = build_archetype_tags_prompt(&available_tags);
        let mut selected_tags = HashSet::new();
        dialogue(|s_tags| format!("Chosen tags: {:?}\n Choose a tag again to remove it.\n", s_tags) + &prompt,
                 selected_tags.len(),
                 available_tags.len(),
                 |n, s_tags| {
                     if n == 0 { return true; }
                     if let Some(tag) = available_tags.get(n - 1) {
                         if s_tags.contains(tag) { s_tags.remove(tag); } else { s_tags.insert(tag.clone()); }
                     }
                     false
                 }, &mut selected_tags);
        let tags = selected_tags.into_iter().collect();
        let vec = CardsVec::from_dict(&cards_map);
        Context {
            cards_map,
            cards_vec: vec,
            game_mode,
            craft,
            tags,
        }
    }

    #[allow(dead_code)]
    pub fn from_debug() -> Self {
        let game_mode = 1;
        let craft = 1;
        let map = CardsMap::from_input(game_mode, craft).unwrap();
        let mut tags = HashSet::new();
        for (_, info) in &map.0 {
            for tag in &info.tags_ {
                tags.insert(tag);
            }
        }
        let tags: Vec<_> = tags.into_iter().map(|s| String::from(s)).collect();
        let vec = CardsVec::from_dict(&map);
        Context {
            cards_map: map,
            cards_vec: vec,
            game_mode,
            craft,
            tags,
        }
    }

    pub fn idx_to_card(&self, idx: usize) -> &CardInfo {
        self.cards_map.0.get(&self.cards_vec.0[idx]).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::context::Context;

    #[test]
    fn ctx() {
        // loading the unlimited forest cardpool.
        let ctx = Context::from_debug();
        // forest has a 0pp card in its pool, so it should be the first in the vector.
        let id = ctx.cards_vec.0[0];
        dbg!(&id);
        let card = ctx.cards_map.0.get(&id).unwrap();
        dbg!(&card);
        assert_eq!(card.pp_, 0);
    }
}
