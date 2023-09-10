use macroquad::audio::play_sound_once;
use macroquad::texture::Texture2D;
use macroquad::texture::draw_texture;
use macroquad::texture::load_texture;
use macroquad::audio::Sound;
use macroquad::audio::load_sound;
use macroquad::color::Color;
use macroquad::color;
use macroquad::window;
use macroquad::math::Vec2;
use macroquad::input::KeyCode;
use macroquad::input::is_key_down;

use crate::game_world::GameWorld;



pub struct Paddle {
    tex: Texture2D,
    left: bool,
    pub pos: Vec2,
    color: Color,
    acc: f32,
    move_speed: f32,
    pub max_speed: f32,
    pub lives: i32,
    start_lives: i32,
    hit_sound: Sound,
    pub size: Vec2,
}

impl Paddle {
    pub async fn new(/*content_manager: &ContentManager, */left: bool, lives: i32) -> Self {
        // let paddle_texture = content_manager.load::<Texture2D>("paddle");
        // let hit_sound = content_manager.load::<Sound>("hit");
        
        let tex = load_texture("paddle.png").await.unwrap();
        let hit_sound = load_sound("hit.wav").await.unwrap();

        let pos = if left {
            Vec2::new(tex.width() * 1.5, window::screen_height() / 2.0)
        } else {
            Vec2::new(window::screen_width() - tex.width() * 1.5, window::screen_height() / 2.0)
        };
        let color = if left { color::RED } else { Color::new(0.0, 0.3, 1.0, 1.0) };



        Self {
            size: Vec2{ x: tex.width(), y: tex.height() },
            tex,
            left,
            pos,
            color,
            acc: 2.0,
            move_speed: 0.0,
            max_speed: 11.0,
            lives,
            start_lives: lives,
            hit_sound,
        }
    }

    pub fn handle_input(&mut self) {

        if self.left {
            if is_key_down(KeyCode::W) {
                self.move_pls(false);
            }
            if is_key_down(KeyCode::S) {
                self.move_pls(true);
            } 
        } else {
            if is_key_down(KeyCode::Up) {
                self.move_pls(false);
            }
            if is_key_down(KeyCode::Down) {
                self.move_pls(true);
            }
        }
    }

    pub fn update(&mut self) {
        if (self.move_speed > 0.0 && self.pos.y < window::screen_height()) || 
            (self.move_speed < 0.0 && self.pos.y > 0.0) {
            self.pos.y += self.move_speed;
        }

        if self.move_speed < 0.0 {
            self.move_speed += 1.0;
        }
        if self.move_speed > 0.0 {
            self.move_speed -= 1.0;
        }
    }

    // NOTE: move is a reserved word in Rust...
    pub fn move_pls(&mut self, down: bool) {
        if down && self.move_speed < self.max_speed {
            self.move_speed += self.acc;
        } else if self.move_speed > -self.max_speed {
            self.move_speed -= self.acc;
        } 
    }

    pub fn move_AI(&mut self, down: bool) {

        if (down && self.pos.y < window::screen_height() - self.max_speed) {
            self.move_pls(true);
        } else if self.pos.y > self.max_speed {
            self.move_pls(false);
        }
        //same here except this is for the ai
    }

    pub fn draw(&self) {
        // sprite_batch.draw_texture(self.paddle, self.pos, Some(self.color));
        
        draw_texture(&self.tex, self.pos.x, self.pos.y, color::WHITE);
        // TODO: does this anchor at top left? we may need to use draw_texture_ex with pivot... IF pivot anchors.
        // otherwise we will need more math.
    }

    pub fn reset(&mut self) {
        self.lives = self.start_lives;
        self.move_speed = 0.0;
        let horizontal_position = if self.left { 
            self.tex.width() * 1.5 
        } else { 
            window::screen_width() - self.tex.width() * 1.5 
        };
        self.pos = Vec2::new(horizontal_position, window::screen_height() / 2.0);
    }

    pub fn lose_life(&mut self) -> bool{
        self.lives -= 1;

        // rust doesn't like this!
        // if self.lives == 0 {
        //     game_world.end_game(self.left);
        // }
        play_sound_once(&self.hit_sound);
        self.lives == 0 // return "died" status
    }

    // Getters
    // pub fn lives(&self) -> i32 {
    //     self.lives
    // }
}
