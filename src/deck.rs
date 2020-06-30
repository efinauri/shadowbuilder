//! Implements the `Deck` structure.

use rand::Rng;

use crate::card::Card;
use crate::context::Context;
use crate::generics;
use crate::globals;
use std::cmp;

/// The chosen encoding uses a vectors of unique cards and a vector of their number of copies in
/// use.
pub struct Deck {
    /// Represents a card with its respective `Context.card_list` index.
    cards: Vec<usize>,
    /// Holds in its ith element the number of copies of the ith element of `cards`.
    /// This vector is usually consulted through the `copies()` method.
    copies_: Vec<usize>,
}

/// The [official site converts](https://shadowverse-portal.com) a card id to base 64
/// with custom radix.
fn id_to_sv_portal_encode(id: &u32) -> String {
    let mut id = *id;
    let radix: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_"
        .chars()
        .collect();
    let mut sv_portal_encode = String::new();
    while id > 0 {
        sv_portal_encode.push(radix[(id % 64) as usize]);
        id /= 64;
    }
    generics::invert(sv_portal_encode)
}
/// Returns the name of the card corresponding to the given `card_library` index.
fn name(card: usize, ctx: &Context) -> &String {
    &ctx.card_list[card]
}

/// Returns the [Card](../card/struct.Card.html) corresponding to the given `card_library` index.
fn attrs(card: usize, ctx: &Context) -> &Card {
    &ctx.card_library[name(card, ctx)]
}

impl Deck {
    /// Returns the number of copies of `card` in the deck.
    ///
    ///  # Note
    ///
    /// It is important that this task isn't made simpler by maintaining the deck as an ordered list
    /// of cards. In a GA, any ordering made on the chromosome invalidates the benefits of
    /// the crossover phase.
    fn copies(&self, card: usize) -> usize {
        match self.cards.iter().position(|&n| n == card) {
            Some(i) => self.copies_[i],
            None => 0,
        }
    }

    /// Returns the current deck size.
    fn current_size(&self) -> usize {
        self.copies_.iter().sum::<usize>() as usize
    }

    /// Returns an array whose ith element is the number of cards in the deck with a pp cost of i+1.
    ///
    /// pp costs lower than 1 or higher than 8 are grouped in the respective 0/1 and 8+ category
    fn get_pp_curve(&self, ctx: &Context) -> [usize; globals::PP_CURVE_SIZE] {
        let mut pp_curve = [0; globals::PP_CURVE_SIZE];
        for card in &self.cards {
            let mut pp_cost = attrs(*card, ctx).pp_ as usize;
            if pp_cost > globals::PP_CURVE_SIZE {
                pp_cost = globals::PP_CURVE_SIZE;
            }
            if pp_cost == 0 {
                pp_cost = 1;
            }
            pp_curve[pp_cost - 1] += self.copies(*card);
        }
        pp_curve
    }

    pub fn new() -> Deck {
        Deck {
            cards: Vec::new(),
            copies_: Vec::new(),
        }
    }

    /// Adds `card` to the deck and returns whether the deck size was increased or not.
    fn add(&mut self, card: usize) -> bool {
        const MCC_EXCLUSIVE: usize = globals::MAX_CARD_COPIES - 1;
        match self.copies(card) {
            0 => {
                self.cards.push(card);
                self.copies_.push(1);
                true
            }
            1..=MCC_EXCLUSIVE => {
                self.copies_[self.cards.iter().position(|&n| n == card).unwrap()] =
                    self.copies(card) + 1;
                true
            }
            _ => false,
        }
    }

    /// Fills the deck with random cards until the
    /// [legal deck size](../globals/constant.DECK_SIZE.html)is reached.
    fn random_fill(&mut self, ctx: &Context) {
        while self.current_size() < globals::DECK_SIZE {
            self.add(rand::thread_rng().gen_range(0, ctx.card_list.len()));
        }
    }

    /// Initializes a deck with random cards.
    pub fn from_random(ctx: &Context) -> Deck {
        let mut deck = Deck::new();
        deck.random_fill(&ctx);
        deck
    }

    /// Selects a random card index from the deck.
    ///
    /// This is done by weighing each card in respect to its number of copies in the deck.
    fn select_random_card_index(&self) -> usize {
        let roll = rand::thread_rng().gen_range(0, self.current_size());
        let mut count: usize = 0;
        for i in 0..self.copies_.len() {
            count += self.copies_[i];
            if count > roll {
                return i;
            }
        }
        self.copies_.len()
    }

    /// Removes one copy of the specified card index (that is, relative to `self.cards`
    /// and `self._copies`).
    fn cut(&mut self, card_index: usize) {
        self.copies_[card_index] -= 1;
        if self.copies_[card_index] == 0 {
            self.copies_.remove(card_index);
            self.cards.remove(card_index);
        }
    }
    /// Removes one copy of a random card and returns it.
    fn cut_random(&mut self) -> usize {
        let card_index = self.select_random_card_index();
        self.cut(card_index);
        card_index
    }

    /// Assigns a score between 0 and 1 to the deck.
    ///
    ///  # Notes
    /// * Just for sake of having a working fitness function, `rate` returns the weighed sum of
    /// partial scores determined by:
    /// 1) The euclidean distance between the pp curve and an arbitrary one;
    /// 2) The amount of cards in the deck with the ones given by the user.
    ///
    /// The domain of each of these scores is normalized to `[0, 1]`. It follows that for the total
    /// score to fall in that same range, the weighs have to sum to 1.
    /// * There's no reward specifically associated with having multiple copies of the same card,
    /// but thankfully the usefulness of such a feature is implicitly understood by the GA.
    /// # TODO
    /// * add some target curves (aggressive, midrange, control?)
    /// * think of some other parameters
    pub fn rate(
        &self,
        ctx: &Context,
        curve_weigh: f64,
        tags_weigh: f64,
        consistency_weigh: f64,
    ) -> f64 {
        // curve
        let target_curve = [4, 14, 6, 5, 4, 3, 2, 2];
        let pp_curve = self.get_pp_curve(&ctx);
        let max_curve_distance = (2.0 * (globals::DECK_SIZE as f64).powf(2.0)).sqrt();
        let mut curve_score = 0.0;
        for i in 0..globals::PP_CURVE_SIZE {
            curve_score += (target_curve[i] as f64 - pp_curve[i] as f64).powf(2.0);
        }
        let curve_score = 1.0 - (curve_score).sqrt() / max_curve_distance;
        //tags
        let mut tags_score = 0.0;
        for card in self.cards.iter() {
            for tag in &ctx.tags {
                if attrs(*card, &ctx).tags_.contains(tag) {
                    tags_score += self.copies(*card) as f64;
                    break;
                }
            }
        }
        tags_score = tags_score / globals::DECK_SIZE as f64;
        // consistency
        let consistency_score = (2.0 + globals::DECK_SIZE as f64)
            / (globals::MAX_CARD_COPIES as f64 * self.copies_.len() as f64);
        curve_score * curve_weigh + tags_score * tags_weigh + consistency_score * consistency_weigh
    }

    /// Randomly changes a card in the deck.
    ///
    /// # Arguments
    ///
    /// When the card `c` gets selected and discarded, its replacement is picked in the
    /// [card list](../context/struct.Context.html#structfield.card_list) slice `[c-x, c+x]`, where
    /// `x` is whichever comes first between the end of the array and the following expression:
    /// `max(t_cap, t - (t_annealing * generation))`, where the `t`s are shorthand for temperatures.
    ///
    /// * `temperature` - How far away from `c` in the card library a card is eligible to be picked.
    /// * `temperature_annealing` - The amount, per unit of time, by which the above `temperature`
    /// is progressively dampened.
    /// * `generation` - The unit of time mentioned above and defined
    /// [here](../population/struct.Population.html#structfield.generation).
    /// * `temperature_cap` - `temperature` can't be dampened below this value.
    ///
    /// This is done to have some form of control over the mutation, since the initial sorting done
    /// on `card_list` will have clumped similar cards in similar places. A high temperature will
    /// thus correspond to poking around for maxima by exploring a region of the solution space
    /// different enough from the one the deck is in. As the generations, pass the population
    /// gradually takes hold of a same maximum, and with a lower temperature mutations take up the
    /// role of a more systematic and thorough exploration of the local solution space (more
    /// closely matching the fine tuning made on a deck with a consolidated core/gameplan).
    pub fn mutate(
        &mut self,
        ctx: &Context,
        temperature: usize,
        temperature_annealing: usize,
        generation: u32,
        temperature_cap: usize,
    ) {
        let card_index = self.cut_random();
        let temperature =
            if temperature < (temperature_annealing * generation as usize + temperature_cap) {
                temperature_cap
            } else {
                temperature - (temperature_annealing * generation as usize)
            };
        let low = if card_index <= temperature {
            0
        } else {
            card_index - temperature
        };
        let high = cmp::min(ctx.card_list.len() - 1, card_index + temperature);
        loop {
            let roll = rand::thread_rng().gen_range(low, high);
            if self.add(roll) {
                return ();
            }
        }
    }

    /// Recombines with another deck into a new deck.
    ///
    /// At the moment only single point crossover is implemented.
    pub fn mix(&mut self, other: &mut Deck) -> Deck {
        let crossover = rand::thread_rng().gen_range(0, globals::DECK_SIZE - 1);
        let mut deck = Deck::new();
        for _ in 0..crossover {
            deck.add(self.cards[0]);
            self.cut(0);
            other.cut(0);
        }
        for _ in 0..globals::DECK_SIZE - crossover - 1 {
            deck.add(other.cards[0]);
            other.cut(0);
        }
        while deck.current_size() != globals::DECK_SIZE {
            if rand::thread_rng().gen_bool(0.5) {
                deck.add(self.cards[self.select_random_card_index()]);
            } else {
                deck.add(other.cards[other.select_random_card_index()]);
            }
        }
        deck
    }

    /// Displays the deck.
    pub fn print(&self, ctx: &Context) {
        let mut cards = self.cards.clone();
        cards.sort();
        // printing the card list.
        for card in cards.iter() {
            println!(
                "[{:<3}]  {}x {}",
                card,
                self.copies(*card),
                name(*card, &ctx)
            );
        }

        let pp_curve = self.get_pp_curve(&ctx);
        println!("{}", generics::hist(&pp_curve, "#", 1));

        let mut sv_portal_link = String::from(format!(
            "https://shadowverse-portal.com/deck/{}.{}",
            ctx.game_mode, ctx.craft
        ));
        for card in self.cards.iter() {
            for _ in 0..self.copies(*card) {
                sv_portal_link.push('.');
                sv_portal_link.push_str(&id_to_sv_portal_encode(&attrs(*card, ctx).id_));
            }
        }
        println!("{}?lang=en", sv_portal_link);
    }
}

impl Clone for Deck {
    fn clone(&self) -> Deck {
        Deck {
            cards: self.cards.clone(),
            copies_: self.copies_.clone(),
        }
    }
}

#[cfg(test)]
#[test]
fn fromrandom_randomfill_name_attrs() {
    let c = Context::from_debug();
    let d = Deck::from_random(&c);
    assert_eq!(d.current_size(), globals::DECK_SIZE);
}

#[test]
fn new_add_copies_size_ppcurve() {
    let c = Context::from_debug();
    let mut d = Deck::new();
    for _ in 0..5 {
        d.add(0);
    }
    d.add(1);
    assert_eq!(d.copies(0), 3);
    assert_eq!(d.current_size(), 4);
    assert_eq!(d.get_pp_curve(&c), [4, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn select_random_index() {
    let mut d = Deck::new();
    d.add(1);
    d.add(1);
    d.add(1);
    d.add(2);
    let mut count = 0.0;
    const N_TESTS: usize = 100_000;
    for _ in 0..N_TESTS {
        if d.select_random_card_index() == 0 {
            count += 1.0
        }
    }
    println!("{}", count / N_TESTS as f64);

    fn abs(n: f64) -> f64 {
        if n < 0.0 {
            -1.0 * n
        } else {
            n
        }
    }
    assert!(abs(0.75 - (count / N_TESTS as f64)) < 0.01);
}

#[test]
fn cut_random() {
    let c = Context::from_debug();
    let mut d = Deck::from_random(&c);
    for _ in 0..20 {
        d.cut_random();
    }
    assert_eq!(d.current_size(), globals::DECK_SIZE - 20);
}
#[test]
fn rate() {
    let ctx = Context::from_debug();
    let mut d = Deck::new();
    for card in 0..13 {
        for _ in 0..6 {
            d.add(card);
        }
    }
    d.add(14);
    assert_eq!(d.get_pp_curve(&ctx), [40, 0, 0, 0, 0, 0, 0, 0]);
}

#[test]
fn mutation() {
    let ctx = Context::from_debug();
    let mut d = Deck::from_random(&ctx);
    for i in 0..ctx.card_library.len() {
        d.mutate(&ctx, ctx.card_library.len() / 2, 2, i as u32, 3);
    }
}
