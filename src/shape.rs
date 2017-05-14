use std::thread::spawn;

use rand::{Rng, thread_rng};
use chan::Receiver;

use world::{show, Block};
use command::Command;
use COLUMNS;

pub type Grid = Vec<Vec<u8>>;
pub type PosColumn = u8;
pub type PosRow = u8;
pub type CommandReceiver = Receiver<Command>;

#[derive(Debug)]
pub struct Inner {
    grid: Grid,
    pos_x: PosColumn,
    pos_y: PosRow,
    rx: CommandReceiver,
}

impl Inner {
    fn new(grid: Grid, rx: CommandReceiver) -> Self {
        let mut inner = Inner {
            grid: grid,
            rx: rx,
            pos_x: 0,
            pos_y: 0,
        };
        inner.listen();
        inner
    }

    fn with_fall(&self) -> Self {
        let mut inner = Inner {
            grid: self.grid.to_owned(),
            pos_x: self.pos_x,
            pos_y: self.pos_y + 1,
            rx: self.rx.clone(),
        };
        inner.listen();
        inner
    }

    fn listen(&mut self) {
        for command in self.rx.recv() {
            use Command::*;
            match command {
                Left => {
                    println!("left");
                    if self.pos_x > 0 {
                        self.pos_x -= 1;
                    }
                }
                Right => {
                    println!("right");
                    if self.pos_x < COLUMNS - 1 {
                        self.pos_x += 1;
                    }
                }
                Bottom => {
                    println!("bottom");
                    if !self.is_on_world() {
                        self.pos_y += 1;
                    }
                }
            }
        }
    }

    fn get_positions(&self) -> Vec<(PosRow, PosColumn)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(offset_y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|&(_, c)| *c as usize > 0)
                    .map(|(offset_x, _)| {
                        (offset_y as PosRow + self.pos_y, offset_x as PosColumn + self.pos_x)
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn is_on_world(&self) -> bool {
        self.get_positions()
            .iter()
            .any(|&(pos_r, _)| pos_r as usize >= self.grid.len() - 1)
    }
}

#[derive(Debug)]
pub enum Shape {
    Square(Inner),
    BracketL(Inner),
    BracketR(Inner),
    Straight(Inner),
    TLike(Inner),
}

impl Shape {
    fn square(rx: CommandReceiver) -> Self {
        Shape::Square(Inner::new(vec![vec![1, 1], vec![1, 1]], rx))
    }

    fn bracket_l(rx: CommandReceiver) -> Self {
        Shape::BracketL(Inner::new(vec![vec![1, 0], vec![1, 0], vec![1, 1]], rx))
    }

    fn bracket_r(rx: CommandReceiver) -> Self {
        Shape::BracketR(Inner::new(vec![vec![0, 1], vec![0, 1], vec![1, 1]], rx))
    }

    fn straight(rx: CommandReceiver) -> Self {
        Shape::Straight(Inner::new(vec![vec![1], vec![1], vec![1], vec![1]], rx))
    }

    fn t_like(rx: CommandReceiver) -> Self {
        Shape::TLike(Inner::new(vec![vec![0, 1, 0], vec![1, 1, 1]], rx))
    }

    pub fn new(rx: CommandReceiver) -> Self {
        let mut rng = thread_rng();
        let rnd = rng.gen::<u8>();
        match rnd % 5 {
            4 => Shape::bracket_l(rx),
            3 => Shape::bracket_r(rx),
            2 => Shape::straight(rx),
            1 => Shape::t_like(rx),
            _ => Shape::square(rx),
        }
    }

    pub fn tick(&self) -> Self {
        use self::Shape::*;
        match self {
            &Square(ref inner) => Square(inner.with_fall()),
            &BracketL(ref inner) => BracketL(inner.with_fall()),
            &BracketR(ref inner) => BracketR(inner.with_fall()),
            &Straight(ref inner) => Straight(inner.with_fall()),
            &TLike(ref inner) => TLike(inner.with_fall()),
        }
    }

    pub fn get_positions(&self) -> Vec<(PosRow, PosColumn)> {
        use self::Shape::*;
        match self {
            &Square(ref inner) |
            &BracketL(ref inner) |
            &BracketR(ref inner) |
            &Straight(ref inner) |
            &TLike(ref inner) => inner.get_positions(),
        }
    }
}

impl Block for Shape {
    fn show(&self) -> String {
        use self::Shape::*;
        match self {
            &Square(ref inner) |
            &BracketL(ref inner) |
            &BracketR(ref inner) |
            &Straight(ref inner) |
            &TLike(ref inner) => show(&inner.grid),
        }
    }
}
