mod world;
static COLMUN: u8 = 10;
static ROWS: u8 = 14;

fn main() {
    let game = world::World::new(COLMUN, ROWS);

    // loop
    game.tick();
    println!("{}", game.show());
}

