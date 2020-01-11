use crate::math::{
    Direction, Position,
};

#[derive(Default)]
pub struct World {
    pub player: Player,
    pub enemies: Vec<Enemy>,
}
impl World {
    pub fn is_enemy_attacking_player(&self) -> bool {
        let player_position = self.player.position();
        for enemy in &self.enemies {
            if enemy.is_in_range(player_position) {
                return true;
            }
        }
        false
    }
}

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

struct MovementStatus {
    /// The x and y position
    position: Position,
    /// The number of units they will move in a single second
    speed: f64,
    /// The x and y direction
    direction: Direction,
}
impl MovementStatus {
    fn new(position: Position, speed: f64, direction: Direction) -> MovementStatus {
        MovementStatus { position, speed, direction }
    }
    fn position(&self) -> Position { self.position }

    fn turn_towards(&mut self, target: Position) {
        self.direction = self.position.direction_to(target);
    }

    fn update(&mut self, time_delta: f64) {
        let space_moved = self.direction * (self.speed * time_delta);
        self.position += space_moved;
    }
}
