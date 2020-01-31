use super::{
    common::{MovementStatus},
};
use crate::{
    math::{Direction, Position},
};

// TODO Keep track of a list of health sources so we can easily re-calculate (and display) the max
// TODO Other sources for all stats as well (even movement speed)
pub struct Player {
    move_status: MovementStatus,
    max_hp: f64,
    /// Amount of HP lost so far
    lost_hp: f64,
}
impl Player {
    pub fn current_hp(&self) -> f64 { self.max_hp - self.lost_hp }

    // TODO Take a position to determine if the damage is coming from behind
    pub fn apply_damage(&mut self, damage: f64) {
        self.lost_hp += damage;
    }
}
impl Default for Player {
    fn default() -> Player {
        Player {
            max_hp: 100.0,
            lost_hp: 0.0,
            move_status: MovementStatus::new(
                Position::new(0.0, 0.0),
                0.5,
                Direction::new(0.0, 1.0)
            ),
        }
    }
}
impl Player {
    pub fn position(&self) -> Position {
        self.move_status.position()
    }
    pub fn update_movement(&mut self, time_delta: f64) {
        self.move_status.update(time_delta);
    }
    pub fn turn_towards(&mut self, target: Position) {
        self.move_status.turn_towards(target);
    }
}
