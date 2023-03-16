use tetris;

pub fn main() {
    let bg = tetris::blocks::BlockGenerator::new();
    for block in bg {
        println!("{}\n", block);
    }

    let board = tetris::board::Board::new();
    println!("{}", board);
}
