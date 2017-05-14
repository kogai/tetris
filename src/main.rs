extern crate rand;
extern crate chan;

mod world;
mod shape;
mod command;

use chan::async;
use world::{World, Block};
use command::{Command, Controller};

static COLMUN: u8 = 10;
static ROWS: u8 = 14;
static INTERVAL: u64 = 10;

fn main() {
    let (tx, rx) = async::<_>();
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
