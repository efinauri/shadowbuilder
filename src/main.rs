mod globals;
mod generics;
mod card;
mod context;
mod deck;
mod population;

use crate::population::Population;
use crate::context::Context;

fn main() {
    // TODO: find a small library for plotting fitness over generation.
    let ctx = Context::from_input();
    let mut p = Population::new(&ctx);
    while p.cycle(&ctx) {}
}
