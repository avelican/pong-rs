pub mod pong;
pub mod ball;
pub mod collision;
pub mod game_world;
pub mod input_helper;
pub mod paddle;
pub mod particle;
pub mod particle_system;
pub mod util;

use macroquad::prelude::*;

#[macroquad::main("Pong")]
async fn main() {
    // seed RNG
    rand::srand(macroquad::miniquad::date::now() as _);

    loop {
        clear_background(RED);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("IT WORKS!!", 20.0, 20.0, 30.0, DARKGRAY);

        next_frame().await
    }
}

