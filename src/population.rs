use crate::globals;
use crate::generics;
use crate::context::Context;
use crate::deck::Deck;
use rand::Rng;

pub struct Population {
    decks: Vec<Deck>,
    ratings: Vec<f64>,
    pub generation: u32,
    pub min_rating: f64,
    pub avg_rating: f64,
    pub max_rating: f64,
    best_rating: f64,
    pub best_individual: Deck,
}

impl Population {
    pub fn new(ctx: &Context) -> Population {
        let mut decks = Vec::new();
        let mut ratings = Vec::new();
        let mut max_rating = 0.0;
        let mut best_individual = Deck::new();
        for i in 0..globals::POPULATION_SIZE {
            decks.push(Deck::from_random(&ctx));
            ratings.push(decks[i].rate(&ctx));
            if ratings[i] > max_rating {
                max_rating = ratings[i];
                best_individual = decks[i].clone();
            }
        }
        Population {
            decks,
            min_rating: generics::min(&ratings),
            avg_rating: generics::avg(&ratings),
            ratings,
            generation: 1,
            max_rating,
            best_rating: max_rating,
            best_individual,
        }
    }

    fn update_history(&mut self) {
        self.generation += 1;
        self.min_rating = generics::min(&self.ratings);
        self.avg_rating = generics::avg(&self.ratings);
        self.max_rating = generics::max(&self.ratings);
        if self.max_rating > self.best_rating {
            self.best_rating = self.max_rating;
            self.best_individual =
                self.decks[self.ratings.iter().position(|&n| n == self.max_rating).unwrap()].clone();
        }
    }

    fn cull(&mut self, threshold: f64, annealing: f64, threshold_cap: f64) -> f64{
        let mut threshold = threshold + annealing * self.generation as f64;
        if threshold > threshold_cap { threshold = threshold_cap; }
        let mut current_idx = 0;
        while current_idx < self.decks.len() {
            if self.ratings[current_idx] < threshold {
                self.ratings.remove(current_idx);
                self.decks.remove(current_idx);
            } else { current_idx += 1; }
        }
        threshold
    }

    fn select(&self) -> Deck {
        loop {
            let individual = rand::thread_rng().gen_range(0, self.decks.len());
            if rand::thread_rng().gen::<f64>() < self.ratings[individual] / self.max_rating {
                return self.decks[individual].clone();
            }
        }
    }

    pub fn cycle(&mut self, ctx: &Context) -> bool{
        println!("\n Generation: {}\n", self.generation);
        let cull_threshold = self.cull(0.3, 0.01, 1.0);
        if self.decks.len() < 2 {
            println!("Too many individuals were culled \
            for the program to continue.\n");
            self.best_individual.print(&ctx);
            return false;
        }
        println!("Threshold: {:.3}", cull_threshold);
        println!("Remaining individuals: {} [{:.3}%]", self.decks.len(),
                 100 * self.decks.len()/globals::POPULATION_SIZE);
        let mut next_decks = Vec::new();
        let mut next_ratings = Vec::new();
        while next_decks.len() < globals::POPULATION_SIZE {
            let mother = self.select();
            let father = &self.select();
            let child = mother.mix(father, &ctx);
            next_ratings.push(child.rate(&ctx));
            next_decks.push(child);
        }
        self.decks = next_decks;
        self.ratings = next_ratings;
        self.update_history();
        println!("Fitness profile of generation {}:\n\
        \tMinimum fitness: {:<28}\n\
        \tAverage fitness: {:<28}\n\
        \tMaximum fitness: {:<28}\n\
        Overall fitness profile:\n\
        \tMaximum fitness: {:<28}\n\
        ", self.generation, self.min_rating, self.avg_rating, self.max_rating, self.best_rating);
        true
    }
}

#[cfg(test)]
#[test]
fn test() {
    let ctx = Context::from_debug();
    let mut p = Population::new(&ctx);
    assert!(p.max_rating > p.avg_rating && p.avg_rating > p.min_rating);
    p.best_individual.print(&ctx);
    for _ in 0..10 {
        p.cycle(&ctx)
    }
    while p.best_rating < 0.7 {
        p.cycle(&ctx);
    }
    p.best_individual.print(&ctx);
}