mod math;
mod render;
mod world;

use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d,
};

use crate::{
    math::{
        probability::{RollTable},
    },
    render::{DrawingKit},
    world::{Enemy, EnemyType, World},
};

macro_rules! log {
    ( $( $t:tt )* ) => {
        if cfg!(debug_assertions) {
            web_sys::console::log_1(&format!( $( $t )* ).into());
        }
    }
}

#[wasm_bindgen]
pub fn first_time_setup() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    log!("First time setup complete");
}
#[wasm_bindgen]
pub fn init_context(context: &CanvasRenderingContext2d) {
    context.set_fill_style(&("white".into()));
    context.set_stroke_style(&("black".into()));
    context.set_line_width(10.0);
}

#[wasm_bindgen]
pub struct GameState {
    world: World,
    state: StateType,
    next_state: Option<StateType>,
}
#[wasm_bindgen]
impl GameState {
    // TODO Check for some save data here first
    // TODO Save the RNG seed so that people can't just restart the game to get a better seed
    pub fn starting_state() -> GameState {
        // TODO Probably make this roll table from the player's progress somehow
        let enemy_roll_table = RollTable::new(&[(1.0, EnemyType::Slime)]);
        let world = World::new(enemy_roll_table);
        GameState {
            world,
            state: StateType::default(),
            next_state: None,
        }
    }

    pub fn update(&mut self, time_delta: f64) {
        // Convert it to seconds (from milliseconds)
        let time_delta = time_delta * 1e-3;

        if let Some(state) = self.next_state.take() {
            log!("Switching to state ({:?})", state);
            self.state = state;
        }

        self.next_state = self.state.update(&mut self.world, time_delta);
    }

    pub fn render(&self, context: &CanvasRenderingContext2d, width: f64, height: f64) {
        let drawing_kit = DrawingKit::new(context, width, height);
        self.state.render(&self.worl, drawing_kit);
    }
}

#[derive(Copy, Clone, Debug)]
enum StateType {
    // TODO May want a start screen or something
    /// Where the player can buy equipment and where they go after they die
    Staging,
    /// The player is progressing through the world and enemies are spawning then attacking
    Progressing,
    /// The game is paused. The menus should still be useable but no progress is happening.
    /// The screen should have a dark film painted overtop of it
    Paused,
}
impl StateType {
    /// The current state can be changed for the next update (after the next render)
    fn update(self, world: &mut World, time_delta: f64) -> Option<StateType> {
        match self {
            Self::Staging => {
                todo!();
            },
            Self::Progressing => {
                world.try_spawn_enemy();

                // 1. Check the enemy ranges to see if they're attacking
                // 2. If an enemy is attacking, face the player towards the closest one
                //    Else move the player
                // 3. All attacking enemies can now perform their attacks
                //    If not attacking, update their movement

                // 1
                let (attacking_enemies, closest_enemy) = world.determine_attacking_enemies();

                // 2
                if let Some(enemy_index) = closest_enemy {
                    self.world.player.face_towards(self.world.enemies[enemy_index].position());
                } else {
                    self.world.player.update_movement(time_delta);
                }

                // 3
                let player_position = self.world.player.position();
                for i in 0..attacking_enemies.len() {
                    let enemy = &mut self.world.enemies[i];
                    // Since the player moved, we need all enemies to start facing them
                    enemy.turn_towards(player_position);
                    if attacking_enemies[i] {
                        enemy.attack(time_delta, &mut self.world.player);
                    } else {
                        // TODO Get the enemy's old position to make sure the enemy can't go too far. Capping at 1/2 their range is probably good
                        enemy.update_movement(time_delta);
                    }
                }
                None
            },
            Self::Paused => {
                todo!();
            },
        }
    }

    fn render<'a>(self, world: &World, drawing_kit: DrawingKit<'a>) {
        match self {
            Self::Staging => {
                todo!();
            },
            Self::Progressing => {
                drawing_kit.clear();
                drawing_kit.draw_world(world);
            },
            Self::Paused => {
                todo!();
            },
        }
    }
}
impl Default for StateType {
    // TODO We will want to start in Staging
    fn default() -> StateType { Self::Progressing }
}
