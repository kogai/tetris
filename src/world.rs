use shape;

#[derive(Debug)]
pub struct World {
    grid: Vec<Vec<u8>>
}

impl World {
    pub fn new(x_width: u8, y_width: u8) -> Self {
        World {
          grid: (0..y_width)
            .map(
              |_| (0..x_width).map(|_| 0).collect()
            ).collect()
        }
    }

    pub fn tick(&self) {
        let square = shape::Shape::square();
        println!("{}", square.show());
    }
}

impl Block for World {
    fn show(&self) -> String {
        show(&self.grid)
    }
}

pub trait Block {
  fn show(&self) -> String;
}

pub fn show(grid: &Vec<Vec<u8>>) -> String {
    let white = " 0"; // "\u{26aa}"
    let black = " 1"; // "\u{26ab}"
    grid.iter().map(
      |row| row.iter().map(|x| if *x == 0 { white } else { black } ).collect::<Vec<_>>().join("")
    ).collect::<Vec<_>>().join("\n")
}