mod math;
mod world;

use wasm_bindgen::prelude::*;
use web_sys::{
    CanvasRenderingContext2d,
};

use crate::world::{
    World,
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
}
#[wasm_bindgen]
impl GameState {
    pub fn starting_state() -> GameState {
        GameState {
            world: World::default(),
        }
    }

    pub fn update(&mut self, time_delta: f64) {
        // Convert it to seconds (from milliseconds)
        let time_delta = time_delta * 1e-3;

        if !self.world.is_enemy_attacking_player() {
            self.world.player.update_movement(time_delta);
        }

        let player_position = self.world.player.position();
        for enemy in &mut self.world.enemies {
            enemy.turn_towards(player_position);
            if enemy.is_in_range(player_position) {
                enemy.attack(time_delta, &mut self.world.player);
            } else {
                enemy.update_movement(time_delta);
            }
        }
    }

    pub fn render(&self, context: &CanvasRenderingContext2d, width: f64, height: f64) {
        context.clear_rect(0.0, 0.0, width, height);

        context.save();
        context.set_fill_style(&("black".into()));

        // Wall
        context.stroke_rect(75.0, 140.0, 150.0, 110.0);

        // Door
        context.fill_rect(130.0, 190.0, 40.0, 60.0);

        // Roof
        context.begin_path();
        context.move_to(50.0, 140.0);
        context.line_to(150.0, 60.0);
        context.line_to(250.0, 140.0);
        context.close_path();
        context.stroke();

        context.restore();
    }
}
