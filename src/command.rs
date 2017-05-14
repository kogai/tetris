use std::thread::{spawn, sleep};
use std::time::{Duration};
use chan::Sender;
use INTERVAL;

#[derive(Debug, Clone)]
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
        self.tx.send(command);
    }

    pub fn fall_with_interval(&self) {
        let tx = self.tx.clone();
        spawn(move || {
            loop {
                tx.send(Command::Bottom);
                sleep(Duration::from_millis(INTERVAL));
            }
        });
    }
}

#[derive(Debug, Clone)]
pub enum Command {
    Left,
    Right,
    Bottom,
}
