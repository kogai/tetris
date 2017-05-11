mod world;
mod shape;

use world::{World, Block};

static COLMUN: u8 = 10;
static ROWS: u8 = 14;

fn main() {
    let game = World::new(COLMUN, ROWS);

    // loop
    game.tick();
    println!("{}", game.show());
}

