pub fn main() {
    //let mut board = tetris::Board::new();

    //let bg = tetris::BlockGenerator::new();
    //for (idx, block) in bg.enumerate() {
    //    board.add_block(idx*2, &block);
    //}

    //println!("{}", board);

    //board.clear_line(1);

    //println!("\n\n{}", board);
    let mut tetris = tetris::TetrisGame::new();
    tetris.run();

    println!("{}", 2_u32);
    println!("{}", 2_u32 << 1);
    println!("{}", 2_u32 >> 1);
}
