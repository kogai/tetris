use std::sync::mpsc::{channel, Sender};

#[derive(Debug)]
pub struct Controller {
    tx: Sender<Command>,
}

impl Controller {
    pub fn new(tx: Sender<Command>) -> Self {
        Controller {
            tx,
        }
    }
    
    pub fn send(&self, command: Command) {
        match self.tx.send(command) {
            Ok(_) => (),
            Err(error) => println!("{}", error),
        }
    }
}

#[derive(Debug)]
pub enum Command {
    Left,
    Right,
    Bottom,
}
