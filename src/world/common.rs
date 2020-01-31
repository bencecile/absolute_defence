use crate::{
    math::{Direction, Position},
};

pub struct MovementStatus {
    /// The x and y position
    position: Position,
    /// The number of units they will move in a single second
    speed: f64,
    /// The x and y direction
    direction: Direction,
}
impl MovementStatus {
    pub fn new(position: Position, speed: f64, direction: Direction) -> MovementStatus {
        MovementStatus { position, speed, direction }
    }
    pub fn position(&self) -> Position { self.position }

    pub fn turn_towards(&mut self, target: Position) {
        self.direction = self.position.direction_to(target);
    }

    pub fn update(&mut self, time_delta: f64) {
        let space_moved = self.direction * (self.speed * time_delta);
        self.position += space_moved;
    }
}
