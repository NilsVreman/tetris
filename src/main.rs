pub fn main() {
    let bg = tetris::BlockGenerator::new();
    for block in bg {
        println!("{}\n", block);
    }

    let board = tetris::Board::new();
    println!("{}", board);
}
