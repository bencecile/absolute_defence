pub mod probability;

use std::{
    ops::{
        Add, AddAssign, Mul,
    },
};

pub fn f64_equal(num1: f64, num2: f64) -> bool {
    (num1 - num2).abs() < 1e-10
}

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
        Direction::normalize_vector(other.0 - self.0, other.1 - self.1)
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
    /// Takes an angle from the horizontal axis, counter-clockwise.
    /// Radian / 2 is 90 degrees pointing up (+y)
    pub fn from_angle(radians: f64) -> Direction {
        // TODO Do this properly
    }

    /// Reverses the direction to point opposite.
    pub fn reverse(&mut self) {
        self.0 = -self.0;
        self.1 = -self.1;
    }
}
impl Direction {
    fn normalize_vector(x: f64, y: f64) -> Direction {
        let total = x + y;
        Direction(x / total, y / total)
    }
}
/// Since we want a direction to stay normalized, this must turn into a Position
impl Mul<f64> for Direction {
    type Output = Position;
    fn mul(self, rhs: f64) -> Position { Position::new(self.0 * rhs, self.1 * rhs) }
}
