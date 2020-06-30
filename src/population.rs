//! Implements the `Population` structure.

use crate::context::Context;
use crate::deck::Deck;
use crate::generics;
use crate::globals;
use rand::Rng;

pub struct Population {
    decks: Vec<Deck>,
    ratings: Vec<f64>,
    pub generation: u32,
    pub min_rating: f64,
    pub avg_rating: f64,
    ///Maximum fitness in the current generation.
    pub max_rating: f64,
    ///Maximum fitness over the generations.
    best_rating: f64,
    pub best_individual: Deck,
    pub fitness_args: Vec<f64>,
    pub cull_args: Vec<f64>,
    pub selection_args: Vec<usize>,
    pub mix_args: Vec<usize>,
    pub mutation_args: Vec<usize>,
}

impl Population {
    pub fn from_random(ctx: &Context) -> Population {
        let mut decks = Vec::new();
        for _ in 0..globals::POPULATION_SIZE {
            decks.push(Deck::from_random(&ctx));
        }
        Population {
            decks,
            ratings: vec![],
            generation: 0,
            min_rating: 0.0,
            avg_rating: 0.0,
            max_rating: 0.0,
            best_rating: 0.0,
            best_individual: Deck::new(),
            fitness_args: vec![],
            cull_args: vec![],
            selection_args: vec![],
            mix_args: vec![],
            mutation_args: vec![],
        }
    }

    pub fn set_fitness_args(&mut self, curve_weigh: f64, tags_weigh: f64, consistency_weigh: f64) {
        self.fitness_args = vec![curve_weigh, tags_weigh, consistency_weigh];
    }

    pub fn set_cull_args(&mut self, threshold: f64, annealing: f64, threshold_cap: f64) {
        self.cull_args = vec![threshold, annealing, threshold_cap];
    }

    #[allow(dead_code)]
    pub fn set_selection_args(&mut self) {}

    #[allow(dead_code)]
    pub fn set_mix_args(&mut self) {}

    pub fn set_mutation_args(
        &mut self,
        temperature: usize,
        temperature_annealing: usize,
        temperature_cap: usize,
    ) {
        self.mutation_args = vec![temperature, temperature_annealing, temperature_cap];
    }

    fn update_history(&mut self) {
        self.generation += 1;
        self.min_rating = generics::min(&self.ratings);
        self.avg_rating = generics::avg(&self.ratings);
        self.max_rating = generics::max(&self.ratings);
        if self.max_rating > self.best_rating {
            self.best_rating = self.max_rating;
            self.best_individual = self.decks[self
                .ratings
                .iter()
                .position(|&n| n == self.max_rating)
                .unwrap()]
            .clone();
        }
    }

    fn cull(&mut self, threshold: f64, annealing: f64, threshold_cap: f64) -> f64 {
        let mut threshold = threshold + (annealing * self.generation as f64);
        if threshold > threshold_cap {
            threshold = threshold_cap;
        }
        let mut current_idx = 0;
        while current_idx < self.decks.len() {
            if self.ratings[current_idx] < threshold {
                self.ratings.remove(current_idx);
                self.decks.remove(current_idx);
            } else {
                current_idx += 1;
            }
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

    pub fn cycle(&mut self, ctx: &Context) -> bool {
        self.ratings = vec![];
        for individual in &self.decks {
            self.ratings.push(individual.rate(
                &ctx,
                self.fitness_args[0],
                self.fitness_args[1],
                self.fitness_args[2],
            ));
        }
        self.update_history();
        println!("\n Generation: {}\n", self.generation);
        println!(
            "Fitness profile of generation {}:\n\
        \tMinimum fitness: {:<28}\n\
        \tAverage fitness: {:<28}\n\
        \tMaximum fitness: {:<28}\n\
        Overall fitness profile:\n\
        \tMaximum fitness: {:<28}\n\
        ",
            self.generation, self.min_rating, self.avg_rating, self.max_rating, self.best_rating
        );
        let cull_threshold = self.cull(self.cull_args[0], self.cull_args[1], self.cull_args[2]);
        if self.decks.len() < 2 {
            println!(
                "Too many individuals were culled \
            for the program to continue.\n"
            );
            self.best_individual.print(&ctx);
            return false;
        }
        println!("Threshold: {:.3}", cull_threshold);
        println!(
            "Remaining individuals: {} [{:.2}%]",
            self.decks.len(),
            100.0 * self.decks.len() as f64 / globals::POPULATION_SIZE as f64
        );
        let mut next_decks = Vec::new();
        while next_decks.len() < globals::POPULATION_SIZE {
            let mut parent = self.select();
            let mut child = parent.mix(&mut Deck::from_random(&ctx));
            child.mutate(
                &ctx,
                ctx.card_library.len(),
                1,
                self.generation,
                (ctx.card_library.len() as f64).sqrt().round() as usize,
            );

            next_decks.push(child);
        }
        self.decks = next_decks;
        true
    }
}

// #[cfg(test)]
// #[test]
// fn test() {
//     let ctx = Context::from_debug();
//     let mut p = Population::new(&ctx);
//     assert!(p.max_rating > p.avg_rating && p.avg_rating > p.min_rating);
//     p.best_individual.print(&ctx);
//     for _ in 0..10 {
//         p.cycle(&ctx);
//     }
//     while p.best_rating < 0.7 {
//         p.cycle(&ctx);
//     }
//     p.best_individual.print(&ctx);
// }
