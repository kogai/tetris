use std::sync::Arc;
use std::sync::mpsc::Receiver;

use rand::{Rng, thread_rng};
use world::{show, Block};
use command::Command;

pub type Grid = Vec<Vec<u8>>;
pub type PosColumn = u8;
pub type PosRow = u8;

#[derive(Debug)]
pub struct Inner {
    grid: Grid,
    pos_x: PosColumn,
    pos_y: PosRow,
    rx: Arc<Receiver<Command>>,
}

impl Inner {
    fn new(grid: Grid, rx: Arc<Receiver<Command>>) -> Self {
        Inner {
            grid,
            rx,
            pos_x: 0,
            pos_y: 0,
        }
    }

    fn with_fall(&self) -> Self {
        Inner {
            grid: self.grid.to_owned(),
            pos_x: self.pos_x,
            pos_y: self.pos_y + 1,
            rx: self.rx.clone(),
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
    fn square(rx: Arc<Receiver<Command>>) -> Self {
        Shape::Square(Inner::new(vec![
                vec![1, 1], vec![1, 1]
            ], rx))
    }

    fn bracket_l(rx: Arc<Receiver<Command>>) -> Self {
        Shape::BracketL(Inner::new(vec![
                vec![1, 0],
                vec![1, 0],
                vec![1, 1]
            ], rx))
    }

    fn bracket_r(rx: Arc<Receiver<Command>>) -> Self {
        Shape::BracketR(Inner::new(vec![
                vec![0, 1],
                vec![0, 1],
                vec![1, 1]
            ], rx))
    }
    
    fn straight(rx: Arc<Receiver<Command>>) -> Self {
        Shape::Straight(Inner::new(vec![
                vec![1],
                vec![1],
                vec![1],
                vec![1],
            ], rx))
    }

    fn t_like(rx: Arc<Receiver<Command>>) -> Self {
        Shape::TLike(Inner::new(vec![
                vec![0, 1, 0],
                vec![1, 1, 1],
            ], rx))
    }

    pub fn new(rx: Arc<Receiver<Command>>) -> Self {
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
