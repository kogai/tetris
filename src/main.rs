extern crate rand;

mod world;
mod shape;
mod command;

use world::{World, Block};
use command::Command;

static COLMUN: u8 = 10;
static ROWS: u8 = 14;

fn main() {
    let mut game = World::new(COLUMNS, ROWS);
    let mut tick_count = 0;

    loop {
        if tick_count > MAX_LOOP {
            break;
        }
        tick_count = tick_count + 1;

        match tick_count {
            1 => game.send(Command::Left),
            2 => game.send(Command::Right),
            _ => game.send(Command::Bottom),
        };
        game.tick();

        println!("{}", game.show());
        println!("---");
    }
}
