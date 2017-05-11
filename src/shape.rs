use world::{show, Block};

#[derive(Debug)]
pub enum Shape {
  Square(Vec<Vec<u8>>),
  BracketL(Vec<Vec<u8>>),
  BracketR(Vec<Vec<u8>>),
  Straight(Vec<Vec<u8>>),
  TLike(Vec<Vec<u8>>),
}

impl Shape {
    pub fn square() -> Self {
        Shape::Square(vec![
          vec![1,1],
          vec![1,1],
        ])
    }
}

impl Block for Shape {
    fn show(&self) -> String {
      use self::Shape::*;
      match self {
          &Square(ref grid) |
          &BracketL(ref grid) |
          &BracketR(ref grid) |
          &Straight(ref grid) |
          &TLike(ref grid) => show(grid),
      }
    }
}