use std::sync::mpsc::Receiver;
use shape::{PosRow, PosColumn, Grid, Shape};
use command::Command;

type CommandReceiver = Receiver<Command>;

#[derive(Debug)]
pub struct World {
    grid: Grid,
    shape_queue: Vec<Shape>,
    fixed: Vec<(PosRow, PosColumn)>,
    rx: CommandReceiver,
}

impl World {
    pub fn new(num_of_columns: u8, num_of_rows: u8, rx: CommandReceiver) -> Self {
        World {
            grid: (0..num_of_rows)
                .map(|_| (0..num_of_columns).map(|_| 0).collect())
                .collect(),
            shape_queue: vec![],
            fixed: vec![],
            rx: rx,
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
                        let is_matched_position = to_fill.iter()
                            .any(|&(r, c)| r as usize == i && c as usize == j);
                        if is_matched_position { 1 } else { 0 }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    pub fn tick(&mut self) {
        let shape = match self.shape_queue.pop() {
            Some(shape) => shape,
            None => Shape::new(),
        };

        let mut to_fill = shape.get_positions();
        let is_landed = to_fill.iter().any(|&(pos_r, _)| {
            let on_world = pos_r as usize >= self.grid.len() - 1;
            let on_exist = self.fixed.iter().any(|&(exist_pos_r, _)| pos_r + 1 == exist_pos_r);
            on_world || on_exist
        });

        if is_landed {
            self.fixed.append(&mut to_fill);
        } else {
            let shape = match self.rx.try_recv() {
                Ok(cmd) => shape.move_to(cmd),
                Err(_) => shape,
            };
            self.shape_queue.push(shape);
        };

        let fill = [to_fill.as_slice(), self.fixed.as_slice()].concat();
        self.grid = self.fill_world(fill);
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
