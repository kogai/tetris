extern crate rand;

mod world;
mod shape;
mod command;

use world::{World, Block};

static COLMUN: u8 = 10;
static ROWS: u8 = 14;

fn main() {
    let mut game = World::new(COLMUN, ROWS);
    let mut tick_count = 0;

    loop {
        if tick_count > MAX_LOOP {
            break;
        }
        tick_count = tick_count + 1;
        game.tick();
        println!("{}", game.show());
        println!("---");
    }
}
