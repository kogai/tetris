use world::{show, Block};

pub type Grid = Vec<Vec<u8>>;
pub type PosX = u8;
pub type PosY = u8;

#[derive(Debug)]
struct Inner {
  grid: Vec<Vec<u8>>,
  posX: u8,
  posY: u8,
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
    pub fn square() -> Self {
        Shape::Square(Inner {
          grid: vec![
            vec![1,1],
            vec![1,1],
          ],
          posX: 0,
          posY: 0,
        })
    }
    
    // pub fn bracket_l() -> Self {
    //     Shape::BracketL(vec![
    //       vec![1,0],
    //       vec![1,0],
    //       vec![1,1],
    //     ], 0, 0)
    // }
    
    // pub fn bracket_r() -> Self {
    //     Shape::BracketR(vec![
    //       vec![0,1],
    //       vec![0,1],
    //       vec![1,1],
    //     ], 0, 0)
    // }

    pub fn tick(&mut self) -> Self {
      use self::Shape::*;
      match self {
          &mut Square(ref inner) => Square(Inner { grid: inner.grid.clone(), posX: inner.posX, posY: inner.posY + 1 }),
          &mut BracketL(ref inner) => BracketL(Inner { grid: inner.grid.clone(), posX: inner.posX, posY: inner.posY + 1  }),
          &mut BracketR(ref inner) => BracketR(Inner { grid: inner.grid.clone(), posX: inner.posX, posY: inner.posY + 1 }),
          &mut Straight(ref inner) => Straight(Inner { grid: inner.grid.clone(), posX: inner.posX, posY: inner.posY + 1 }),
          &mut TLike(ref inner) => TLike(Inner { grid: inner.grid.clone(), posX: inner.posX, posY: inner.posY + 1 }),
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