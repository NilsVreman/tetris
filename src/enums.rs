#[derive(Debug, Copy, Clone)]
pub enum BlockID {
    I, J, L, O, S, T, Z,
}

pub enum ShiftCmd {
    Left, Right,
}

pub enum RotateCmd {
    Left, Right,
}

#[derive(Debug)]
pub enum GameStatus {
    Okay,
    GameOver,
}
