// Command enums
pub enum ShiftCmd {
    Left,
    Right,
}

pub enum RotateCmd {
    Left,
    Right,
}

pub enum BoardStatus {
    Overflow(usize),
    Okay(usize),
}

