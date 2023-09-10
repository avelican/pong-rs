// pub mod pong;
pub mod ball;
pub mod collision;
pub mod game_world;
pub mod input_helper;
pub mod paddle;
pub mod particle;
pub mod particle_system;
pub mod util;

use game_world::GameWorld;
use macroquad::prelude::*;

#[macroquad::main("Pong")]
async fn main() {
    // seed RNG
    rand::srand(macroquad::miniquad::date::now() as _);

    set_pc_assets_folder("assets");
    let mut game_world = GameWorld::new().await;

    loop {
        clear_background(RED);

        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        // draw_text("IT WORKS!!", 20.0, 20.0, 30.0, DARKGRAY);

        game_world.handle_input();
        game_world.update();
        game_world.draw();
        
        

        next_frame().await
    }
}

