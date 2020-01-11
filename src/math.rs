use std::{
    ops::{
        Add, AddAssign, Mul,
    },
};

#[derive(Copy, Clone)]
pub struct Position(f64, f64);
impl Position {
    pub fn new(x: f64, y: f64) -> Position { Position(x, y) }

    pub fn distance_to(self, other: Position) -> f64 {
        let diff_x = other.0 - self.0;
        let diff_y = other.1 - self.1;
        diff_x.hypot(diff_y)
    }
    pub fn direction_to(self, other: Position) -> Direction {
        Direction::new(other.0 - self.0, other.1 - self.1)
    }
}
impl Add<Position> for Position {
    type Output = Position;
    fn add(self, rhs: Position) -> Position { Position::new(self.0 + rhs.0, self.1 + rhs.1) }
}
impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[derive(Copy, Clone)]
pub struct Direction(f64, f64);
impl Direction {
    /// Normalizes the direction first
    pub fn new(x: f64, y: f64) -> Direction {
        let total = x + y;
        Direction(x / total, y / total)
    }
}
/// Since we want a direction to stay normalized, this must turn into a Position
impl Mul<f64> for Direction {
    type Output = Position;
    fn mul(self, rhs: f64) -> Position { Position::new(self.0 * rhs, self.1 * rhs) }
}
