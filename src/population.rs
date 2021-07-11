use rand::Rng;
use crate::context::Context;
use crate::deck::Deck;

pub const POPULATION_SIZE: usize = 2_048;

pub struct GA<D>
    where D: Deck
{
    pub ctx: Context,
    // An array would've been more explicit, but arrays of trait objects have a lot of
    // technicalities, ultimately meaning that using a vector is more straightforward.
    // Nonetheless, this field will be treated as though it had the same restrictions of an array.
    pub population: Vec<(D, f64)>,
    pub time: f64,
    pub min_scores: Vec<f64>,
    pub avg_scores: Vec<f64>,
    pub max_scores: Vec<f64>,
    pub rate_args: (f64, f64, f64),
    pub mutation_args: (isize, isize, f64),
    pub cull_args: (f64, f64, f64),
    pub target_fitness: f64,
}

impl<D: Clone + Deck> GA<D> {
    pub fn from_rand(ctx: Context) -> GA<D> {
        let mut population = vec![(Deck::new(), 0.0); POPULATION_SIZE];
        for i in 0..POPULATION_SIZE {
            population[i].0 = D::from_rand(&ctx);
        }
        GA {
            ctx,
            population,
            time: 0.0,
            target_fitness: 1.0,
            min_scores: vec![],
            avg_scores: vec![],
            max_scores: vec![],
            rate_args: (0.0, 0.0, 0.0),
            mutation_args: (0, 0, 0.0),
            cull_args: (0.0, 0.0, 0.0),
        }
    }
    pub fn set_rate_args(&mut self, w_curve: f64, w_tags: f64, w_length: f64) {
        self.rate_args = (w_curve, w_tags, w_length);
    }

    pub fn set_mutation_args(&mut self, temp: isize, starting_temp: isize, temp_annealing: f64) {
        self.mutation_args = (temp, starting_temp, temp_annealing);
    }

    pub(crate) fn set_cull_args(&mut self, threshold: f64, cap: f64, annealing: f64) {
        self.cull_args = (threshold, cap, annealing);
    }

    pub(crate) fn set_stop_condition(&mut self, target_fitness: f64) {
        self.target_fitness = target_fitness;
    }
    // Culls individuals whose fitness is below a threshold that's proportional to time.
    // Culled decks are placed at the end of the list. The index of the last survivor is returned.
    // NOTE: assumes that the population is sorted by score.
    fn cull(&mut self) -> usize {
        let (threshold, cap, annealing) = self.cull_args;
        let threshold = (threshold + (annealing * self.time)).min(cap);
        println!("Culling decks below {}...", threshold);
        let mut lo = 0;
        let mut hi = POPULATION_SIZE - 1;
        loop {
            if hi <= lo { return hi; };
            let ret = lo + (hi - lo) / 2;
            if self.population[ret].1 >= threshold {
                lo = ret + 1;
            } else {
                hi = ret - 1;
            }
        }
    }

    // Using stochastic acceptance.
    fn select(&self, selectable: usize) -> usize {
        // selectable: the production of a new generation is done in place: candidate parents will
        // find themselves on the first half of the array, in the section [0..selectable].
        loop {
            let candidate = rand::thread_rng().gen_range(0, selectable);
            if rand::thread_rng().gen::<f64>() < self.population[candidate].1 / self.max_scores.last().unwrap() {
                return candidate;
            }
        }
    }

    fn update_params(&mut self) {
        self.time += 1.0;
        let (w_curve, w_tags, w_length) = self.rate_args;
        let mut scores = [0.0; POPULATION_SIZE];
        for i in 0..POPULATION_SIZE {
            let score = self.population[i].0.rate(&self.ctx, w_curve, w_length, w_tags);
            self.population[i].1 = score;
            scores[i] = score
        }
        self.population.sort_unstable_by(|(_, a), (_, b)| a.partial_cmp(b)
            .unwrap().reverse());
        let min = self.population[POPULATION_SIZE - 1].1;
        let avg = scores.iter().fold(0.0, |s, el| s + el) / POPULATION_SIZE as f64;
        let max = self.population[0].1;
        self.min_scores.push(min);
        self.avg_scores.push(avg);
        self.max_scores.push(max);
        println!("\nGENERATION {}", self.time);
        println!(
            "\tMin fitness: {}\n\
            \tAvg fitness: {}\n\
            \tMax fitness: {}",
            self.min_scores.last().unwrap(),
            self.avg_scores.last().unwrap(),
            self.max_scores.last().unwrap());
    }

    // selects two parents and inserts the resulting children in the new generation. note that this
    // is done in place, dividing the original array into zones.
    fn replace_individual(&mut self, selectable: usize, new_population_idx: usize) {
        // selectable: the index of the last deck selectable as a parent.
        // new_population_idx: the index after which the new generation of decks begins.
        let parent = &self.population[self.select(selectable)].0;
        let mut child = parent.mix(&self.population[self.select(selectable)].0);
        child.mutate(&self.ctx,
                     self.mutation_args.0,
                     self.mutation_args.1,
                     self.mutation_args.2,
                     self.time);
        self.population[new_population_idx] = (child, 0.0);
        // REMINDER: now new_population_idx points to the first new gen deck.
    }

    // Computes the passage of a unit of time for the population:
    // The existing population is rated, sorted and culled.

    // [0, 1, ..., cull_idx, cull_idx + 1, ..., POPULATION_SIZE]
    // on the left of cull_idx are the survivors, on the right culled individuals. if we sort
    //the left part of the array by score (decreasing), and we put the new generation individs
    // on the right, keeping track of when they start with an index similar to what done on
    // cull, we will eventually progressively overwrite actual parents, so if we sort the fitter
    // reproduces an higher amount of times on average, plus champion elitism is implemented
    // because on the last couple only 1 of the two parents, fittest deck is never overwritten.

    pub fn tick(&mut self) -> bool {
        self.update_params();
        if self.max_scores.last().unwrap() >= &self.target_fitness { return false }
        let selectable = self.cull();
        if selectable < 2 {
            println!(
                "Too many individuals were culled \
            for the program to continue.\n"
            );
            return false;
        }
        println!("...{:.2}% of the population left", 100.0 * ((selectable + 1) as f64 / POPULATION_SIZE as f64));
        for i in (1..POPULATION_SIZE).rev() {
            self.replace_individual(selectable.min(i), i);
        }
        return true;
    }
}


 #[cfg(test)]
 mod tests {
     use rand::Rng;

     use crate::context::Context;
     use crate::deck::{DeckBTree, Deck};
     use crate::population::{GA, POPULATION_SIZE};

     #[test]
     fn  cull_and_params() {
         let ctx = Context::from_debug();
         let mut ga: GA<DeckBTree> = GA::from_rand(ctx);
         ga.set_rate_args(0.5, 0.0, 0.5);
         ga.set_cull_args(0.0, 0.9, 0.05);
         ga.update_params();
         assert!(ga.population[0].1 >= ga.population[1].1);
         assert_eq!(ga.cull(), POPULATION_SIZE - 1);
         ga.set_cull_args(1.0, 0.9, 0.05);
         ga.update_params();
         assert_eq!(ga.cull(), 0);
        }

    #[test]
    fn select() {
        let ctx = Context::from_debug();
        let mut ga: GA<DeckBTree> = GA::from_rand(ctx);
        ga.population[0].1 = 0.5;
        ga.population[1].1 = 0.125;
        ga.population[2].1 = 0.125;
        ga.population[3].1 = 0.25;
        ga.max_scores.push(1.0);
        let tries = 100_000;
        let mut results = [0.0; 4];
        for _ in 0..tries {
            results[ga.select(4)] += 1.0;
        }
        for i in 0..4 {
            assert!(f64::abs(ga.population[i].1 - results[i] / tries as f64) < 0.01);
        }
    }

    #[test]
    fn replace() {
        let ctx = Context::from_debug();
        let mut ga: GA<DeckBTree> = GA::from_rand(ctx);
        ga.set_rate_args(0.5, 0.0, 0.5);
        ga.set_mutation_args(20, 1, 0.05);
        ga.update_params();
        let d2 = ga.population[0].0.clone();
        // this should correspond to a mutation of the first individual, the test is the same
        // one made for mutation
        ga.replace_individual(1, 0);
        let d = &ga.population[0].0;
        let mut amount_of_differences = 0;
        for (key, _) in &d.0 {
            if d2.0.contains_key(key) {
                // the new card is a new copy of one already in the deck
                if d.0[key] != d2.0[key] {
                    amount_of_differences += 1;
                }
                // the mutation generated a new card
            } else { amount_of_differences += 1; }
        }
        println!("{}\n{}", d.url(&ga.ctx), d2.url(&ga.ctx));
        assert!(amount_of_differences <= 2);
    }

    #[test]
    fn tick() {
        let ctx = Context::from_debug();
        let mut p: GA<DeckBTree> = GA::from_rand(ctx);
        p.set_rate_args(0.5, 0.0, 0.5);
        p.set_mutation_args(10, 3, 0.05);
        // should terminate immediately, returning false
        p.set_cull_args(1.0, 1.0, 0.005);
        assert!(!p.tick());
        // this time there should be no culls
        p.set_cull_args(0.0, 0.0, 0.005);
        assert!(p.tick());
    }
}











