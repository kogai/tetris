extern crate rand;

mod world;
mod shape;
mod command;

use std::sync::mpsc::channel;
use world::{World, Block};
use command::{Command, Controller};

static COLUMNS: u8 = 10;
static ROWS: u8 = 14;
static INTERVAL: u64 = 1000;

fn main() {
    let (tx, rx) = channel::<_>();
    let controller = Controller::new(tx);
    let mut game = World::new(COLUMNS, ROWS, rx);

    controller.fall_with_interval();

    loop {
        game.tick();

        println!("{}", game.show());
        println!("---");
    }
}
