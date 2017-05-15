use rand::{Rng, thread_rng};

use world::{show, Block};
use command::Command;
use COLUMNS;

pub type Grid = Vec<Vec<u8>>;
pub type PosColumn = u8;
pub type PosRow = u8;

#[derive(Debug, Clone)]
pub struct Inner {
    grid: Grid,
    pos_x: PosColumn,
    pos_y: PosRow,
}

impl Inner {
    fn new(grid: Grid) -> Self {
        Inner {
            grid: grid,
            pos_x: 0,
            pos_y: 0,
        }
    }

    fn move_to(&self, command: Command) -> Self {
        use Command::*;
        match command {
            Left if self.pos_x > 0 => {
                Inner {
                    grid: self.grid.clone(),
                    pos_x: self.pos_x - 1,
                    pos_y: self.pos_y,
                }
            }
            Right if self.pos_x < COLUMNS - 1 => {
                Inner {
                    grid: self.grid.clone(),
                    pos_x: self.pos_x + 1,
                    pos_y: self.pos_y,
                }
            }
            Bottom => {
                Inner {
                    grid: self.grid.clone(),
                    pos_x: self.pos_x,
                    pos_y: self.pos_y + 1,
                }
            }
            _ => self.clone(), 
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
    fn square() -> Self {
        Shape::Square(Inner::new(vec![vec![1, 1], vec![1, 1]]))
    }

    fn bracket_l() -> Self {
        Shape::BracketL(Inner::new(vec![vec![1, 0], vec![1, 0], vec![1, 1]]))
    }

    fn bracket_r() -> Self {
        Shape::BracketR(Inner::new(vec![vec![0, 1], vec![0, 1], vec![1, 1]]))
    }

    fn straight() -> Self {
        Shape::Straight(Inner::new(vec![vec![1], vec![1], vec![1], vec![1]]))
    }

    fn t_like() -> Self {
        Shape::TLike(Inner::new(vec![vec![0, 1, 0], vec![1, 1, 1]]))
    }

    pub fn new() -> Self {
        let mut rng = thread_rng();
        let rnd = rng.gen::<u8>();
        match rnd % 5 {
            4 => Shape::bracket_l(),
            3 => Shape::bracket_r(),
            2 => Shape::straight(),
            1 => Shape::t_like(),
            _ => Shape::square(),
        }
    }

    pub fn move_to(&self, command: Command) -> Self {
        use self::Shape::*;
        match self {
            &Square(ref inner) => Square(inner.move_to(command)),
            &BracketL(ref inner) => BracketL(inner.move_to(command)),
            &BracketR(ref inner) => BracketR(inner.move_to(command)),
            &Straight(ref inner) => Straight(inner.move_to(command)),
            &TLike(ref inner) => TLike(inner.move_to(command)),
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
