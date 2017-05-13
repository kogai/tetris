use rand::random;
use shape::{PosRow, PosColumn, Grid, Shape};

#[derive(Debug)]
pub struct World {
    grid: Grid,
    shape_queue: Vec<Shape>,
}

impl World {
    pub fn new(num_of_columns: u8, num_of_rows: u8) -> Self {
        World {
            grid: (0..num_of_rows)
                .map(|_| (0..num_of_columns).map(|_| 0).collect())
                .collect(),
            shape_queue: vec![],
        }
    }

    fn fill_world(&self, to_fill: Vec<(PosRow, PosColumn)>) -> Grid {
        self.grid
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, _)| {
                        let is_matched_position = to_fill.iter().any(|&(r, c)| r as usize == i && c as usize == j);
                        if is_matched_position { 1 } else { 0 }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    pub fn tick(&mut self) {
        let shape = match self.shape_queue.pop() {
            Some(s) => s.tick(),
            None => random::<Shape>(),
        };

        let to_fill = shape.get_positions();
        self.shape_queue.push(shape);
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
    grid.iter()
        .map(|row| {
            row.iter().map(|x| if *x == 0 { white } else { black }).collect::<Vec<_>>().join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}
