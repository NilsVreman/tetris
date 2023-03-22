use std::ops::Add;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Coord(pub i32, pub i32);

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}
