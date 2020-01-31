use super::{
    common::{MovementStatus},
};

pub struct Enemy {
    move_status: MovementStatus,
    damage: f64,
    attacks_left: u64,
    /// In seconds between attacks
    attack_speed: f64,
    /// In seconds
    time_since_last_attack: f64,
    /// In world units
    attack_range: f64,
}
impl Enemy {
    pub fn attack_range(&self) -> f64 { self.attack_range }

    pub fn is_in_range(&self, target: Position) -> bool {
        self.position().distance_to(target) < self.attack_range
    }

    pub fn attack(&mut self, time_delta: f64, player: &mut Player) {
        let total_time = self.time_since_last_attack + time_delta;
        let total_attacks = total_time / self.attack_speed;
        let attacks_to_do = total_attacks.trunc();
        // Keep track of any leftover time
        self.time_since_last_attack = total_time - (attacks_to_do * self.attack_speed);

        for _ in 0..(total_attacks as u64) {
            player.apply_damage(self.damage);
            self.attacks_left = self.attacks_left.saturating_sub(1);
        }
    }
}
impl Enemy {
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

#[derive(Copy, Clone)]
pub enum EnemyType {
    Slime,
}
impl EnemyType {
    pub fn spawn(self, position: Position, direction: Direction) -> Enemy {
        match self {
            Self::Slime => Enemy {
                move_status: MovementStatus::new(position, 1.5, direction),
                damage: 5.0,
                attacks_left: 3,
                attack_speed: 1.25,
                time_since_last_attack: 0.0,
                attack_range: 0.2,
            },
        }
    }
}
