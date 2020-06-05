//! Implements the `Deck` structure.

use rand::Rng;

use crate::card::Card;
use crate::context::Context;
use crate::generics;
use crate::globals;

/// The chosen encoding uses a vectors of unique cards and a vector of their number of copies in
/// use.
pub struct Deck {
    /// Represents a card with its respective `Context.card_list` index.
    cards: Vec<usize>,
    /// Holds in its ith element the number of copies of the ith element of `cards`.
    /// This vector is usually consulted through the `copies()` method.
    _copies: Vec<usize>,
}

/// The official site converts a card id to base 64 with custom radix.
fn id_to_sv_portal_encode(id: &u32) -> String {
    let mut id = *id;
    let radix: Vec<char> =
        "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_".chars().collect();
    let mut sv_portal_encode = String::new();
    while id > 0 {
        sv_portal_encode.push(radix[(id % 64) as usize]);
        id /= 64;
    }
    generics::invert(sv_portal_encode)
}

fn name(card: usize, ctx: &Context) -> &String {
    &ctx.card_list[card]
}

fn attrs(card: usize, ctx: &Context) -> &Card { &ctx.card_library[name(card, ctx)] }

impl Deck {
    fn copies(&self, card: usize) -> usize {
        match self.cards.iter().position(|&n| n == card) {
            Some(i) => self._copies[i],
            None => 0,
        }
    }

    fn current_size(&self) -> usize { self._copies.iter().sum::<usize>() as usize }

    fn get_pp_curve(&self, ctx: &Context) -> [usize; globals::PP_CURVE_SIZE] {
        let mut pp_curve = [0; globals::PP_CURVE_SIZE];
        for card in &self.cards {
            let mut pp_cost = attrs(*card, ctx).mana_cost as usize;
            if pp_cost > globals::PP_CURVE_SIZE { pp_cost = globals::PP_CURVE_SIZE; }
            if pp_cost == 0 { pp_cost = 1; }
            pp_curve[pp_cost - 1] += self.copies(*card);
        }
        pp_curve
    }

    pub fn new() -> Deck {
        Deck { cards: Vec::new(), _copies: Vec::new() }
    }

    fn random_fill(&mut self, ctx: &Context) {
        while self.current_size() < globals::DECK_SIZE {
            self.add(rand::thread_rng().gen_range(0, ctx.card_list.len()));
        }
    }

    pub fn from_random(ctx: &Context) -> Deck {
        let mut deck = Deck::new();
        deck.random_fill(&ctx);
        deck
    }

    fn add(&mut self, card: usize) {
        // the check of if adding a card brings the deck over 40 cards is left to the methods
        // calling add(), for speed.
        fn min(n: usize, m: usize) -> usize { if n < m { n } else { m } }
        match self.copies(card) {
            0 => {
                self.cards.push(card);
                self._copies.push(1);
            }
            _ => self._copies[self.cards.iter().position(|&n| n == card).unwrap()] = min(globals::MAX_CARD_COPIES, self.copies(card) + 1),
        }
    }

    fn remove_random(&mut self) {
        let roll = rand::thread_rng().gen_range(1, self.cards.len() - 1);
        self._copies[roll] -= 1;
        if self._copies[roll] == 0 {
            self._copies.remove(roll);
            self.cards.remove(roll);
        }
    }

    pub fn rate(&self, ctx: &Context) -> f64 {
        let least_unique_cards = (globals::DECK_SIZE / globals::MAX_CARD_COPIES) as f64 + 1.0;
        let consistency_score = least_unique_cards / self.cards.len() as f64;
        let target_curve = [4, 14, 6, 5, 4, 3, 2, 2];
        let pp_curve = self.get_pp_curve(&ctx);
        let max_curve_distance = (2.0 * (globals::DECK_SIZE as f64).powf(2.0)).sqrt();
        let mut curve_score = 0.0;
        for i in 0..globals::PP_CURVE_SIZE {
            curve_score += (target_curve[i] as f64 - pp_curve[i] as f64).powf(2.0);
        }
        let curve_score = 1.0 - (curve_score).sqrt() / max_curve_distance;
        let mut tags_score = 0.0;
        for card in self.cards.iter() {
            for tag in &ctx.tags {
                if attrs(*card, &ctx).tags.contains(tag) {
                    tags_score += self.copies(*card) as f64;
                    break;
                }
            }
        }
        tags_score = tags_score / globals::DECK_SIZE as f64;
        consistency_score * 0.2 + curve_score * 0.4 + tags_score * 0.4
    }

    pub fn mutate(&mut self, ctx: &Context) {
        self.remove_random();
        self.random_fill(ctx);
    }

    pub fn mix(&self, other: &Deck, ctx: &Context) -> Deck {
        let crossover = rand::thread_rng().gen_range(0, globals::DECK_SIZE - 1);
        let mut deck = Deck::new();
        let mut copies_count = 0;
        let mut current_card = 0;
        for _ in 0..crossover {
            deck.add(self.cards[current_card]);
            copies_count += 1;
            if copies_count == self._copies[current_card] {
                current_card += 1;
                copies_count = 0;
            }
        }
        copies_count = 0;
        current_card = 0;
        for _ in crossover..globals::DECK_SIZE - 1 {
            deck.add(other.cards[current_card]);
            copies_count += 1;
            if copies_count == other._copies[current_card] {
                current_card += 1;
                copies_count = 0;
            }
        }
        deck.random_fill(&ctx);
        deck
    }

    pub fn print(&self, ctx: &Context) {
        let mut cards = self.cards.clone();
        cards.sort();
        // printing the card list.
        for card in cards.iter() {
            println!("[{:<3}]  {}x {}", card,
                     self.copies(*card),
                     name(*card, &ctx));
        }
        // printing the deck's curve.
        let pp_curve = self.get_pp_curve(&ctx);
        println!("{}", generics::hist(&pp_curve, "#", 1));
        // printing a shadowverse-portal link.
        let mut sv_portal_link = String::from(
            format!("https://shadowverse-portal.com/deck/{}.{}", ctx.game_mode, ctx.craft));
        for card in self.cards.iter() {
            for _ in 0..self.copies(*card) {
                sv_portal_link.push('.');
                sv_portal_link.push_str(&id_to_sv_portal_encode(&attrs(*card, ctx).id));
            }
        }
        println!("{}?lang=en", sv_portal_link);
    }
}

impl Clone for Deck {
    fn clone(&self) -> Deck {
        Deck {
            cards: self.cards.clone(),
            _copies: self._copies.clone(),
        }
    }
}

#[cfg(test)]
#[test]
fn test() {
    let ctx = Context::from_debug();
    let mut d = Deck::new();
    d.random_fill(&ctx);
    assert_eq!(d.current_size(), globals::DECK_SIZE);
    assert_eq!(attrs(d.cards[0], &ctx).base_data.description,
               ctx.card_library[name(d.cards[0], &ctx)].base_data.description);
    for _ in 0..20 {
        print!("from {}", d.current_size());
        d.remove_random();
        println!(" to {}", d.current_size());
    }
    d = Deck::new();
    for card in 0..13 {
        for _ in 0..6 {
            d.add(card);
        }
    }
    d.add(14);
    assert_eq!(d.get_pp_curve(&ctx), [40, 0, 0, 0, 0, 0, 0, 0]);
    assert!(d.rate(&ctx) > 0.1);
    assert!(d.rate(&ctx) < 0.9);
}
