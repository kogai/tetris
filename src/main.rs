extern crate rand;

mod world;
mod shape;
mod command;

use std::thread::{spawn, sleep};
use std::time::{Duration};
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};

use world::{World, Block};
use command::{Command, Controller};

static COLMUN: u8 = 10;
static ROWS: u8 = 14;

fn main() {
    let (tx, rx) = channel::<Command>();
    let controller = Controller::new(tx);
    let mut game = World::new(COLUMNS, ROWS, Arc::new(rx));

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
