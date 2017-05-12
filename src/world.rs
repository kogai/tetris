use shape;

#[derive(Debug)]
pub struct World {
    grid: Vec<Vec<u8>>
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
    
    pub fn tick(&mut self) {
        let square = shape::Shape::square();
        
        //  (nth_of_row, nth_of_column)
        let to_fill = vec![(0, 0), (1, 0), (2, 0), (3, 0)];
        // let to_fill = vec![(0, 0), (0, 1), (1, 0), (1, 1)];
        
        // fill world grid
        for (nth_of_row, nth_of_column) in to_fill {
            self.grid = self.grid.iter().enumerate().map(
                |(i, row)| {
                    if i == nth_of_row {
                        let result = row.iter().enumerate().map(|(j, col)| {
                            if j == nth_of_column {
                                1
                            } else {
                                *col
                            }
                        }).collect::<Vec<_>>();

                        result
                    } else {
                        row.clone()
                    }
                }
            ).collect::<Vec<_>>();
        }
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