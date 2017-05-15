use std::thread::{spawn, sleep};
use std::time::Duration;
use std::sync::mpsc::Sender;
use INTERVAL;

#[derive(Debug, Clone)]
pub struct Controller {
    tx: Sender<Command>,
}

impl Controller {
    pub fn new(tx: Sender<Command>) -> Self {
        Controller { tx: tx }
    }

    pub fn send(&self, command: Command) {
        self.tx.send(command).expect("Fail at Command");
    }

    pub fn fall_with_interval(&self) {
        let tx = self.tx.clone();
        spawn(move || loop {
            match tx.send(Command::Bottom) {
                Ok(_) => (),
                Err(error) => println!("error {:?}", error),
            };
            sleep(Duration::from_millis(INTERVAL));
        });
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Left,
    Right,
    Bottom,
}
