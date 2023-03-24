use std::ops::{Add, Mul};

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Coord(pub i32, pub i32);

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<f32> for Coord {
    type Output = Coord;

    fn mul(self, rhs: f32) -> Self::Output {
        Coord((self.0 as f32 * rhs) as i32, (self.1 as f32 * rhs) as i32)
    }
}

impl Mul<Coord> for f32 {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        Coord((rhs.0 as f32 * self) as i32, (rhs.1 as f32 * self) as i32)
    }
}
