use std::cmp::{max, min};
use std::collections::btree_map::BTreeMap;

use rand::Rng;

use crate::context::Context;

fn invert(s: String) -> String {
    s.chars().rev().collect()
}

fn hist(l: &[i32; PP_CURVE_SIZE], symbol: &str, gap: usize) -> String {
    fn blanks(n: usize) -> String {
        " ".repeat(n)
    }
    let mut result = String::from("\n");
    for height in 0..*l.iter().max().unwrap() {
        // the hist gets built rotated by 180°, it'll need to be inverted.
        for i in l.iter().rev() {
            if i > &height {
                result += &format!("{}{}", symbol, blanks(gap));
            } else {
                result += &format!("{}{}", blanks(symbol.len()), blanks(gap));
            }
        }
        let mut tmp = format!("\n{:>2} ", height + 1); // y axis
        tmp = invert(tmp); // this substring needs an even number of inversions
        result += &tmp;       // to maintain the left to right orientation of the digits.
    }
    result = invert(result);
    result += &format!("{}{}0/1", blanks(symbol.len()), blanks(gap)); // x axis
    for i in 2..l.len() + 1 {
        result += &format!("{}{}{}", blanks(gap), blanks(symbol.len() - 1), i)
    }
    result + "+\n"
}

const PP_CURVE_SIZE: usize = 8;
const DECK_SIZE: i8 = 40;
const MAX_QTY: i8 = 3;

// The behavior is separated from the underlying data structure in the interest of testing
// alternative deck encodings. Currently this doesn't result in a practical advantage, because
// map-type structures have no common type with which to generalize a trait like Deck.
// This separation is done nonetheless because there is still some small gain in modularity,
// and in the event that Rust introduces such a trait the refactoring would be straightforward.

pub trait Deck{
    type Card;
    fn new() -> Self;
    // basic methods for deck manipulation and monitoring
    fn len(&self) -> i8;
    fn add(&mut self, c: Self::Card);
    fn rand_idx(&self) -> usize;
    fn rand_fill(&mut self, ctx: &Context);
    fn from_rand(ctx: &Context) -> Self;
    fn cut(&mut self, c: Self::Card);
    fn pp_curve(&self, ctx: &Context) -> [i32; PP_CURVE_SIZE];
    // methods for displaying
    fn as_string(&self, ctx: &Context) -> String;
    fn url(&self, ctx: &Context) -> String;
    // methods for the GA
    fn rate(&self, ctx: &Context, w_curve: f64, w_length: f64, w_tags: f64) -> f64;
    fn mutate(&mut self, ctx: &Context, temp: isize, temp_min: isize, temp_annealing: f64, time: f64);
    fn mix(&self, other: &Self) -> Self;
}

// In the chosen deck encoding the keys are cards, represented by their indexes relative to
// CardList, and map to their number of copies.

#[derive(Clone)]
pub struct DeckBTree(pub BTreeMap<usize, i8>);


impl Deck for DeckBTree {
    type Card = usize;

    fn new() -> Self {
        DeckBTree(BTreeMap::new())
    }

    fn len(&self) -> i8 {
        // sum of the number of copies.
        self.0.iter().fold(0, |s, el| s + el.1)
    }

    // NOTE: silently fails if the card to add is already at its maximum amount of copies.
    // NOTE: assumes that the deck isn't full.
    fn add(&mut self, card: usize) {
        let qty = self.0.entry(card).or_insert(0);
        *qty = min(MAX_QTY, *qty + 1);
    }

    // NOTE: assumes that the deck is full.
    fn rand_idx(&self) -> usize {
        let roll = rand::thread_rng().gen_range(1, DECK_SIZE + 1);
        let mut chk = 0;
        for (idx, qty) in &self.0 {
            chk += *qty;
            if roll <= chk {
                return *idx;
            }
        }
        panic!("unreachable section of code");
    }

    fn rand_fill(&mut self, ctx: &Context) {
        while self.len() < DECK_SIZE {
            let card = rand::thread_rng().gen_range(0, ctx.vec.0.len());
            self.add(card)
        }
    }

    fn from_rand(ctx: &Context) -> Self {
        let mut ret = DeckBTree::new();
        ret.rand_fill(ctx);
        ret
    }

    // NOTE: assumes that the card to remove is in the deck.
    fn cut(&mut self, card: usize) {
        let decrement = match self.0.get(&card) {
            Some(qty) => qty - 1,
            None => 0,
        };
        if decrement == 0 {
            self.0.remove_entry(&card);
        } else {
            self.0.insert(card, decrement);
        }
    }

    // Returns a list where the ith element is the number of cards in the deck costing i+1 pps.
    // The first and last bucket also contain all the cards costing, respectively, less than or more
    // than the pp cost of the bucket.
    fn pp_curve(&self, ctx: &Context) -> [i32; PP_CURVE_SIZE] {
        let mut ret = [0; PP_CURVE_SIZE];
        for (idx, qty) in &self.0 {
            let mut pp_idx = ctx.idx_to_card(*idx).pp_ as usize;
            pp_idx = min(pp_idx, PP_CURVE_SIZE);
            pp_idx = max(pp_idx, 1);
            ret[pp_idx - 1] += *qty as i32;
        }
        ret
    }

    fn as_string(&self, ctx: &Context) -> String {
        let mut ret = hist(&self.pp_curve(&ctx), "#", 1);
        for (idx, qty) in &self.0 {
            ret.push_str(&*format!("[{:<3}] {}x {}\n",
                                   idx, qty, ctx.idx_to_card(*idx).name_));
        }
        ret
    }

    // The official site encodes a card ID to radix-64.
    fn url(&self, ctx: &Context) -> String {
        let radix: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz-_"
            .chars().collect();
        let mut deck_hash = String::new();
        for (idx, qty) in &self.0 {
            let mut card_hash = String::new();
            let mut id = ctx.vec.0[*idx];
            while id > 0 {
                card_hash.push(radix[(id % 64) as usize]);
                id /= 64;
            }
            card_hash = card_hash.chars().rev().collect();
            for _ in 0..*qty {
                deck_hash.push_str(&*card_hash);
                deck_hash.push('.');
            }
        }
        // 3 is rotation, 1 is unlimited
        format!("https://shadowverse-portal.com/deck/{}.{}.{}",
                match ctx.game_mode {
                    0 => 3,
                    1 => 1,
                    _ => panic!("invalid game mode (not 0 or 1)")
                },
                ctx.craft,
                deck_hash)
    }

    // The fitness score is a float between 0 and 1, the higher the better the deck is.
    // Its value is a weighed sum of sub-scores.
    fn rate(&self, ctx: &Context, w_curve: f64, w_length: f64, w_tags: f64) -> f64 {
        // FIRST SCORE: how similar the deck curve is to an arbitrary one.
        // The arbitrary curve that the deck should approach.
        let curve_ideal = [4, 14, 6, 5, 4, 3, 2, 2];
        // The euclidean norm of the above.
        let curve_ideal_module = 17.4928556845359;
        // The minimum cosine similarity between the pp curve above and the possible pp curves of a
        // 40 cards deck.
        let curve_min_score = 0.1143323900950059;
        let curve_max_score = 1.0 - curve_min_score;
        let curve_self = self.pp_curve(&ctx);
        let mut curve_score = 0.0;
        let mut module_self = 0.0;
        // Calculating the cosine similarity.
        for i in 0..PP_CURVE_SIZE {
            curve_score += (curve_ideal[i] * curve_self[i]).abs() as f64;
            module_self += curve_self[i].pow(2) as f64;
        }
        curve_score /= curve_ideal_module * module_self.sqrt();
        // Normalizing the score between 0 and 1.
        curve_score = (curve_score - curve_min_score) / curve_max_score;
        // SECOND SCORE: the number of cards with different names in the deck, taken as a measure
        // of consistency.
        let length_offset = 0.35;
        let length_ret_max = 0.65;
        let mut length_ret = self.0.len() as f64 / DECK_SIZE as f64;
        length_ret = 1.0 - (length_ret - length_offset) / length_ret_max;
        // THIRD SCORE: the number of cards that match the given tags.
        let mut tags_ret = 0.0;
        for (idx, qty) in &self.0 {
            let card = Context::idx_to_card(ctx, *idx);
            for tag in &ctx.tags {
                if card.tags_.contains(tag) {
                    tags_ret += *qty as f64;
                    break;
                }
            }
        }
        tags_ret /= DECK_SIZE as f64;
        curve_score * w_curve + length_ret * w_length + tags_ret * w_tags
    }

    // A copy of random card (with CardList index i) is replaced by another extracted from the slice
    // [i-t, i+t]. The slice radius t is initially equal to temp, and is progressively dampened
    // by the factor annealing as time passes, until it reaches its minimum value temp_min.
    fn mutate(&mut self, ctx: &Context, temp: isize, temp_min: isize, temp_annealing: f64, time: f64) {
        let idx = self.rand_idx();
        self.cut(idx);
        let idx = idx as isize;
        let damp = (time * temp_annealing).round() as isize;
        let temp = max(temp_min, temp - damp);
        // out of bounds range are shifted to respect bounds
        let center = max(temp, idx);
        let center = min(center, ctx.vec.0.len() as isize - temp);
        let range = (center - temp, center + temp);
        while self.len() < DECK_SIZE {
            self.add(rand::thread_rng().gen_range(range.0 as usize,
                                                  range.1 as usize));
        }
    }

    // The crossover is single-point.
    //  TODO: double-point crossover should be better.
    fn mix(&self, other: &DeckBTree) -> DeckBTree {
        let cross = rand::thread_rng().gen_range(1, DECK_SIZE - MAX_QTY);
        let mut ret = DeckBTree::new();
        for (idx, qty) in &self.0 {
            for _ in 0..*qty {
                ret.add(*idx);
            }
            if ret.len() >= cross {
                break;
            }
        }
        for (idx, qty) in other.0.iter().rev() {
            for _ in 0..*qty {
                ret.add(*idx);
                if ret.len() == DECK_SIZE {
                    return ret;
                }
            }
        }
        panic!("Failed to reach deck size.")
    }
}

#[cfg(test)]
mod tests {
    use crate::context::Context;
    use crate::deck::{Deck, DECK_SIZE, MAX_QTY, DeckBTree};

    #[test]
    fn add_and_len() {
        let mut d = DeckBTree::new();
        // trying to overadd, it shouldn't go over max_qty
        for _ in 0..2 * MAX_QTY { d.add(0); }
        assert_eq!(d.len(), MAX_QTY);
    }

    #[test]
    fn t_rand_idx() {
        let ctx = Context::from_debug();
        let mut d = DeckBTree::new();
        // adding 3of of 6 cards (partial: 18/40, unique cards: 6)
        // 2of of 6 cards (partial: 30/40, unique cards: 12)
        // filled with 1ofs (unique cards: 22)
        for i in 0..6 {
            for _ in 0..3 {
                d.add(i);
            }
        }
        for j in 6..12 {
            d.add(j);
            d.add(j);
        }
        for k in 12..22 {
            d.add(k);
        }
        println!("{}", d.len());
        println!("{}", d.url(&ctx));
        let tries = 1_000_000;
        let mut ret = [0.0; 22];
        for _ in 0..tries {
            ret[d.rand_idx()] += 1.0;
        }
        for i in 0..22 {
            let expected_probability = {
                if i < 6 { 7.5 } else if i < 12 { 5.0 } else { 2.5 }
            };
            // Expecting first 6 at ~7.5%, second ~5%, rest ~2.5%
            assert!(f64::abs(expected_probability - 100.0 * ret[i] / tries as f64) < 0.1);
        }
    }


    #[test]
    fn rand_and_url() {
        let mut d = DeckBTree::new();
        let ctx = Context::from_debug();
        d.rand_fill(&ctx);
        println!("{}", d.url(&ctx));
    }

    #[test]
    fn cut() {
        let mut d = DeckBTree::new();
        for _ in 0..MAX_QTY {
            d.add(0);
        }
        d.add(1);
        //trying to overremove, it should fail silently
        for _ in 0..2 * MAX_QTY {
            d.cut(0);
        }
        assert_eq!(d.len(), 1)
    }

    #[test]
    fn deck_info() {
        // Rune has both 0pps and >10pps to test index errors.
        let ctx = Context::from_debug();
        let mut d = DeckBTree::new();
        d.add(0);
        d.add(0);
        d.add(0);
        d.add(ctx.vec.0.len() - 1);
        d.add(ctx.vec.0.len() - 1);
        d.add(ctx.vec.0.len() - 1);
        d.rand_fill(&ctx);
        println!("{}", d.as_string(&ctx));
        println!("{:?}", d.pp_curve(&ctx));
        println!("{}", d.url(&ctx));
    }

    #[test]
    fn rate_curve() {
        // testing with dragon, which has 40 10+pp cards
        let ctx = Context::from_debug();
        let mut d = DeckBTree::new();
        // all 10+pp
        for i in ctx.vec.0.len() - 13..ctx.vec.0.len() {
            for _ in 0..3 {
                d.add(i);
            }
        }
        d.add(ctx.vec.0.len() - 14);
        assert!(d.rate(&ctx, 1.0, 0.0, 0.0) < 0.0001);
        let mut d = DeckBTree::new();
        let cmp_arr: [usize; 8] = [4, 14, 6, 5, 4, 3, 2, 2];
        for pp in 0..8 {
            let mut curr: Vec<usize> = Vec::new();
            for i in 0..ctx.vec.0.len() {
                if ctx.map.0.get(&ctx.vec.0[i]).unwrap().pp_ == pp + 1 {
                    curr.push(i)
                }
            }
            for i in 0..cmp_arr[pp as usize] {
                d.add(curr[i as usize]);
            }
        }
        assert!(f64::abs(1.0 - d.rate(&ctx, 1.0, 0.0, 0.0)) < 0.0001);
    }

    #[test]
    fn rate_len() {
        let ctx = Context::from_debug();
        let mut d = DeckBTree::new();
        for i in 0..DECK_SIZE {
            d.add(i as usize);
        }
        assert_eq!(0.0, d.rate(&ctx, 0.0, 1.0, 0.0));
        let mut d = DeckBTree::new();
        for i in 0..13 {
            for _ in 0..3 {
                d.add(i);
            }
        }
        d.add(13);
        assert_eq!(1.0, d.rate(&ctx, 0.0, 1.0, 0.0));
    }

    #[test]
    fn rate_tags() {
        let ctx = Context::from_debug();
        let mut d = DeckBTree::new();
        for i in 0..13 {
            d.add(i);
            d.add(i);
            d.add(i);
        }
        d.add(13);
        // sorting by tags puts tagless cards first, and there's enough of them for a deck,
        // so the score should be zero
        assert_eq!(0.0, d.rate(&ctx, 0.0, 0.0, 1.0));
        d = DeckBTree::new();
        for i in ctx.vec.0.len()-14..ctx.vec.0.len()-1 {
            d.add(i);
            d.add(i);
            d.add(i);
        }
        d.add(ctx.vec.0.len()-1);
        // on the other hand near the end of the list, both because higher pp cards have more effect
        // and because of tagless cards came first, we expect a high concentration of tagged cards.
        assert!( d.rate(&ctx, 0.0, 0.0, 1.0) > 0.6);
        println!("{}", d.url(&ctx));
    }

    #[test]
    fn mutate() {
        let ctx = Context::from_debug();
        let mut d = DeckBTree::from_rand(&ctx);
        let d2 = d.clone();
        d.mutate(&ctx,
                 3,
                 3,
                 0.5,
                 1.0);
        let mut amount_of_differences = 0;
        for (key, _) in &d.0 {
            if d2.0.contains_key(key) {
                if d.0[key] != d2.0[key] {
                    amount_of_differences += 1;
                }
            } else { amount_of_differences += 1; }
        }
            println!("{}\n{}", d.url(&ctx), d2.url(&ctx));
            // it can rarely happen that a card mutates in itself, resulting in no changes
            // the most that can happen is that a card mutates onto an another card in the deck
            // (1 from the contains check + 1 from the card copies comparison)
            assert!(amount_of_differences <= 2);
    }

    #[test]
    fn mix() {
        let mut d1 = DeckBTree::new();
        for i in 0..13 {
            for _ in 0..3 {
                d1.add(i);
            }
        }
        d1.add(13);
        let mut d2 = DeckBTree::new();
        for j in 100..113 {
            for _ in 0..3 {
                d2.add(j);
            }
        }
        d2.add(113);
        let d3 = d1.mix(&d2);
        for key in d3.0.keys() {
            assert!(d1.0.contains_key(key) | d2.0.contains_key(key));
        }
    }
}