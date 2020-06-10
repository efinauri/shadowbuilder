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
    let mut p = Population::new(&ctx);
    while p.cycle(&ctx) {}
    println!("\nEnter any key to exit.");
    std::io::stdin().read_line(&mut String::new());
}
