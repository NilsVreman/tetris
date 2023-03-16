use tetris;

pub fn main() {
    println!("Hello, World!");

    for block in tetris::blocks::BLOCKS {
        println!("{}\n", block);
    }
}
