use shape::{self, PosRow, PosColumn, Grid};

#[derive(Debug)]
pub struct World {
    grid: Grid
}

impl World {
    pub fn new(num_of_columns: u8, num_of_rows: u8) -> Self {
        World {
          grid: (0..num_of_rows)
            .map(
              |_| (0..num_of_columns).map(|_| 0).collect()
            ).collect()
        }
    }

    fn fill_world(&self, to_fill: Vec<(PosRow, PosColumn)>) -> Grid {
        let to_fill_rows = &to_fill.iter().map(|&(row, _)| row).collect::<Vec<_>>();
        let to_fill_columns = &to_fill.iter().map(|&(_, column)| column).collect::<Vec<_>>();

        self.grid.iter().enumerate().map(
            |(i, row)| {
                let should_update_row = to_fill_rows.iter().any(|r| *r as usize == i);
                if should_update_row {
                    row.iter().enumerate().map(|(j, col)| {
                        let should_update_column = to_fill_columns.iter().any(|c| *c as usize == j);
                        if should_update_column { 1 } else { 0 }
                    }).collect::<Vec<_>>()
                } else {
                    row.clone()
                }
            }
        ).collect::<Vec<_>>()
    }

    pub fn tick(&mut self) {
        let square = shape::Shape::square();
        let to_fill = square.get_positions();
        self.grid = self.fill_world(to_fill); 
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

pub fn show(grid: &Grid) -> String {
    let white = " 0"; // "\u{26aa}"
    let black = " 1"; // "\u{26ab}"
    grid.iter().map(
      |row| row.iter().map(|x| if *x == 0 { white } else { black } ).collect::<Vec<_>>().join("")
    ).collect::<Vec<_>>().join("\n")
}