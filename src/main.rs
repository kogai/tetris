extern crate rand;

mod world;
mod shape;
mod command;

use std::sync::mpsc::channel;
use world::{World, Block};
use command::{Command, Controller};

static COLUMNS: u8 = 10;
static ROWS: u8 = 14;
static INTERVAL: u64 = 10;
static MAX_LOOP: u8 = 4;

fn main() {
    let (tx, rx) = channel::<_>();
    let controller = Controller::new(tx);
    let mut game = World::new(COLUMNS, ROWS, rx);
    
    controller.fall_with_interval();

    let mut tick_count = 0;
    loop {
        if tick_count > MAX_LOOP {
            break;
        }
        tick_count = tick_count + 1;

        match tick_count {
            1 => controller.send(Command::Left),
            2 => controller.send(Command::Right),
            _ => controller.send(Command::Bottom),
        };
        
        game.tick();

        println!("{}", game.show());
        println!("---");
    }
}
