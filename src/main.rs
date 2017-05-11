static COLMUN: u8 = 10;
static ROWS: u8 = 20;

fn main() {
    let world =
        (0..ROWS).map(|_| (0..COLMUN).map(|_| 0).collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

    let shapes = vec![vec![1, 1, 1, 1], vec![]];

    for line in world.iter() {
        println!("{:?}", line);
    }
}

