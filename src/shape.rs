use world::{show, Block};

pub type Grid = Vec<Vec<u8>>;
pub type PosX = u8;
pub type PosY = u8;

#[derive(Debug)]
pub struct Inner {
  grid: Grid,
  pos_x: PosX,
  pos_y: PosY,
}

impl Inner {
    fn with_fall(&self) -> Self {
        Inner { grid: self.grid.clone(), pos_x: self.pos_x, pos_y: self.pos_y + 1 }
    }

    fn get_positions(&self) -> Vec<(PosY, PosX)> {
        self.grid.iter().enumerate().flat_map(|(offset_y, row)| {
            row.iter().enumerate().map(|(offset_x, _)| {
                (offset_y as PosY + self.pos_y, offset_x as PosX + self.pos_x)
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>()
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
    pub fn square() -> Self {
        Shape::Square(Inner {
          grid: vec![
            vec![1,1],
            vec![1,1],
          ],
          pos_x: 0,
          pos_y: 0,
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
          &mut Square(ref inner) => Square(inner.with_fall()),
          &mut BracketL(ref inner) => BracketL(inner.with_fall()),
          &mut BracketR(ref inner) => BracketR(inner.with_fall()),
          &mut Straight(ref inner) => Straight(inner.with_fall()),
          &mut TLike(ref inner) => TLike(inner.with_fall()),
      }
    }

    pub fn get_positions(&self) -> Vec<(PosY, PosX)> {
        use self::Shape::*;
        match self {
            &Square(ref inner) => inner.get_positions(),
            &BracketL(ref inner) => inner.get_positions(),
            &BracketR(ref inner) => inner.get_positions(),
            &Straight(ref inner) => inner.get_positions(),
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