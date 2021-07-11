use crate::population::GA;
use crate::context::Context;
use crate::deck::{Deck, DeckBTree};

mod card;
mod context;
mod deck;
mod population;

fn main() {
    let mut g = GA::<DeckBTree>::from_rand(Context::from_input());
    g.set_rate_args(0.4, 0.4, 0.2);
    g.set_mutation_args(20, 3, 0.05);
    g.set_cull_args(0.3, 1.0, 0.005);
    g.set_stop_condition(1.0);
    while g.tick() {}
    println!("{}", g.population[0].0.as_string(&g.ctx));
    println!("{}", g.population[0].0.url(&g.ctx));
    println!("\nEnter any key to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();

}