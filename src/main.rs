mod card;
mod context;
mod deck;
mod generics;
mod globals;
mod population;

use crate::context::Context;
use crate::population::Population;

fn main() {
    // TODO: find a small library for plotting fitness over generation.
    let ctx = Context::from_input();
    println!("{}", ctx.card_list.len());
    let mut p = Population::from_random(&ctx);
    p.set_fitness_args(0.35, 0.40, 0.25);
    p.set_mutation_args(
        ctx.card_library.len(),
        1,
        (ctx.card_library.len() as f64).sqrt().round() as usize,
    );
    p.set_cull_args(0.3, 0.001, 1.0);
    while p.cycle(&ctx) {}
    println!("\nEnter any key to exit.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}

/* TODO: feedback annealing:
    problem: annealing is linearly proportional to the overall speed and inversely to quality,
    so a big annealing makes the population converge quicker but causes an early termination, and a small annealing
    eventually yields a higher quality solution but it's slower
    solution: annealing changes to the percentage of the population that was culled in the generation before
    (no culls -> the annealing can be higher, a lot of culls -> maintain the same value)
*/
