
use std::io;
use fnv::FnvHashMap;
use crate::card::CardInfo;
use std::collections::HashSet;

fn dialogue(prompt: &str, range: (u8, u8)) -> u8 {
    loop {
        println!("{} [{}/{}]: ", prompt, range.0, range.1);
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        match input.trim().parse() {
            Ok(num) => {
                if num as u8 >= range.0 && num as u8 <= range.1 { return num; }
            }
            Err(_) => (),
        }
        println!("Invalid input.");
    }
}


fn get_craft(n: u8) -> &'static str {
    match n {
        1 => "Forestcraft",
        2 => "Swordcraft",
        3 => "Runecraft",
        4 => "Dragoncraft",
        5 => "Shadowcraft",
        6 => "Bloodcraft",
        7 => "Havencraft",
        8 => "Portalcraft",
        _ => ""
    }
}

// Card ID to card info mapping.
// Fnv is preferred to std::collections::HashMap because the speed-for-safety tradeoff that the
// latter makes isn't needed.
pub struct CardsMap(pub FnvHashMap<i32, CardInfo>);

impl CardsMap {
    pub fn from_input(game_mode: u8, craft: u8) -> Result<CardsMap, io::Error> {
       let cards= include_str!("assets/cards.json");
        let mut cards: FnvHashMap<i32, CardInfo> = serde_json::from_str(&cards)?;
        //Filtering:
        cards.retain(|_id, card|
            //Filtering invalid crafts.
            (card.craft_ == get_craft(craft) || card.craft_ == "Neutral")
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
    pub map: CardsMap,
    pub vec: CardsVec,
    pub game_mode: u8,
    pub craft: u8,
    pub tags: Vec<String>,
}

impl Context {
    pub fn from_input() -> Self {
        let game_mode = dialogue(
            "What game format would you like to build a deck for? \
                0 for Rotation, 1 for Unlimited",
            (0, 1)) as u8;
        let craft = dialogue(
            "1 - Forestcraft\t5 - Shadowcraft\n\
                              2 - Swordcraft\t6 - Bloodcraft\n\
                              3 - Runecraft\t7 - Havencraft\n\
                              4 - Dragoncraft\t8 - Portalcraft\n\
                              Which class would you like to use? (options above)",
            (1, 8));
        let map = CardsMap::from_input(game_mode, craft).unwrap();
        let mut available_tags = HashSet::new();
        let mut prompt = String::from("0 - done choosing\n");
        for (_, info) in &map.0 {
            if &info.craft_ == "Neutral" {continue;}
            for tag in &info.tags_ {
                available_tags.insert(tag);
            }
        }
        let available_tags: Vec<_> = available_tags.into_iter().collect();
        for (i, avl_tag) in available_tags.iter().enumerate() {
            prompt += format!("{} - {}\n", i+1, avl_tag).as_str();
        }
        prompt += "What archetypes would you like to build? (options above): ";
        let prompt = prompt.as_str();
        let mut tags = Vec::new();
        loop {
             let input = dialogue(prompt, (0, available_tags.len() as u8)) as usize;
             if input == 0 { break; }
             else if tags.contains(&available_tags[input - 1]) {
                tags.remove(
                    tags.iter()
                        .position(|&t| t.eq(available_tags[input - 1]))
                        .unwrap(),
                );
            } else {
                tags.push(available_tags[input - 1]);
            }
            println!("Chosen tags: {:?}\n Choose a tag again to remove it.", &tags);
        }
        let tags = tags.into_iter().map(|s| String::from(s)).collect();
        let vec = CardsVec::from_dict(&map);
        Context {
            map,
            vec,
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
            map,
            vec,
            game_mode,
            craft,
            tags,
        }
    }

    pub fn idx_to_card(&self, idx: usize) -> &CardInfo {
        self.map.0.get(&self.vec.0[idx]).unwrap()
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
        let id = ctx.vec.0[0];
        let card = ctx.map.0.get(&id).unwrap();
        assert!(vec!["Forestcraft", "Neutral"].contains(&&*card.craft_));
        assert_eq!(card.pp_, 0);
    }
}
