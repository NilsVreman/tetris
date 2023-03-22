pub fn main() {
    let mut tetris = tetris::Tetris::new(10, 20);
    tetris.print();
    while let tetris::GameStatus::Okay = tetris.tick() {}
    tetris.print();
}
