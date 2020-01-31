mod commons;
mod enemy;
mod player;
mod status_effects;
pub use self::{
    enemy::{Enemy, EnemyType},
    player::{Player},
};

use fixedbitset::{FixedBitSet};

use crate::math::{
    probablility::{RollTable},
};

pub struct World {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    enemy_spawn_table: RollTable<EnemyType>,
    max_enemies: usize,
    // TODO Maybe set up the possibly to include modifiers to the spawned enemies stats. The definition for these modifiers should probably be in the World, but should be able to be applied to the spawned enemies.
    // TODO Have a spawn timer. Maybe max allowed enemies spawned. Might want to use a real Distribution for enemy spawn timing
    // TODO Spawn enemies at a set radius from the player
}
impl World {
    pub fn new(enemy_spawn_table: RollTable<EnemyType>) -> World {
        World {
            player: Player::default(),
            enemies: Vec::new(),
            enemy_spawn_table,
            max_enemies: 2,
        }
    }

    pub fn is_enemy_attacking_player(&self) -> bool {
        let player_position = self.player.position();
        for enemy in &self.enemies {
            if enemy.is_in_range(player_position) {
                return true;
            }
        }
        false
    }

    // Returns the bitset of attacking enemies with the optional index of the closest enemy
    pub fn determine_attacking_enemies(&self) -> (FixedBitSet, Option<usize>) {
        let mut attacking_enemies = FixedBitSet::with_capacity(self.world.enemies.len());
        let mut closest_enemy: Option<(usize, f64)> = None;
        let player_position = self.world.player.position();

        for (i, enemy) in self.world.enemies.iter_mut().enumerate() {
            if enemy.is_in_range(player_position) {
                attacking_enemies.insert(i);
                let new_closest_enemy = {
                    if let Some( (closest_index, closest_range) ) = closest_enemy.take() {
                        if enemy.attack_range() < closest_range {
                            (i, enemy.attack_range())
                        } else {
                            (closest_index, closest_range)
                        }
                    } else {
                        (i, enemy.attack_range())
                    }
                };
                closest_enemy = Some(new_closest_enemy);
            }
        }
        (attacking_enemies, closest_enemy.map(|(i, _)| i))
    }

    pub fn try_spawn_enemy(&mut self) {
        if self.enemies.len() < self.max_enemies {
            // TODO Have another roll to see if we spawn an enemy (exponential distribution)
            let enemy_type = self.enemy_spawn_table.roll();

            
            // TODO Randomly make the position some radius from the player (maybe 10?)
            // TODO Find the direction to the player from that
        }
    }
}
