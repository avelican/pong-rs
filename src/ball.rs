use macroquad::texture::Texture2D;
use macroquad::texture::draw_texture;
use macroquad::texture::load_texture;
use macroquad::audio::Sound;
use macroquad::audio::load_sound;
use macroquad::audio::play_sound_once;
use macroquad::math::Vec2;
use macroquad::rand;
use macroquad::window;
use macroquad::color;

use crate::game_world::GameWorld;
use crate::particle_system::ParticleSystem;

// use crate::pong::Pong;
// use crate::particle_system::ParticleSystem;

const START_SPEED: f32 = 6.0; 

pub struct Ball {
    tex: Texture2D,
    
    pong_sfx: Sound,
    hit_wall_sfx: Sound,

    pub pos: Vec2,
    pub dir: Vec2,
    pub size: Vec2,

    // startSpeed: f32, // 6.0
}

impl Ball {
    
    pub async fn new() -> Self {

        // NOTE: pos & dir init in reset()
        let mut ball = Self {
            pos: Vec2::ZERO, 
            dir: Vec2::ZERO, 
            size: Vec2::ZERO,
            tex: load_texture("pong.png").await.unwrap(),
            pong_sfx: load_sound("pong-sfx.wav").await.unwrap(),
            hit_wall_sfx: load_sound("hitwall.wav").await.unwrap(),
        };      
        ball.tex.set_filter(macroquad::texture::FilterMode::Nearest);
        ball.size = Vec2{ x: ball.tex.width(), y: ball.tex.height() };
        ball.reset();
        ball
    }

    pub fn bounce_wall(&mut self, part_sys: &mut ParticleSystem) {
        self.dir *= Vec2::new(1.0, -1.0);
        play_sound_once(&self.hit_wall_sfx);

        // TODO where do we get the particlesystem? GameWorld?
        part_sys.sparkle(self.pos, 1., 0.8, 40);
        

    
        // Eject ball from wall! in case the ball gets stuck
        if self.pos.y < self.tex.height() {
            self.pos += Vec2::new(0.0, 20.0);
        }
    
        if self.pos.y > window::screen_height() + self.tex.height() {
            self.pos -= Vec2::new(0.0, 20.0);
        }
    
        // It's not a bug, it's a feature!
    }
    


    pub fn bounce_paddle(&mut self, paddle_pos: Vec2, part_sys: &mut ParticleSystem) {
        self.dir *= Vec2::new(-1., 1.); // flip x component

        //Depending on height difference, change direction vector
        let speed_before_hit: f32 = Vec2::length(self.dir);
        self.dir += (self.pos - paddle_pos) / 4.; // angle of approach changes angle of bounce
        self.dir = Vec2::normalize(self.dir) * speed_before_hit * 1.1;	// fix speed, add 10%;

        play_sound_once(&self.pong_sfx);
        // pongsfx.Play();
        part_sys.sparkle(self.pos, 2., 0.8, 40);
    }

    pub fn update(&mut self, part_sys: &mut ParticleSystem) {
        // Conditional gravity makes nice sinusoidal patterns in the particles leaving the ball,
        //   and adds a degree of difficulty to the game (and somewhat compensates for perfect AI).
        if (self.pos.y > window::screen_height()){
            self.dir -= Vec2::new(0., 0.1);
        }else{
            self.dir += Vec2::new(0., 0.);
        }
        // Ball leaves a trail of happiness
        part_sys.sparkle(self.pos, 0.3, 0.98, 2);
        
        self.pos += self.dir;			// move ball
    }

    pub fn draw(&self) {
        // spriteBatch.Draw(ball, position, null, Color.White, 0, Size/2, Vector2.One, SpriteEffects.None, 0);

        draw_texture(&self.tex, self.pos.x, self.pos.y, color::WHITE);
        // TODO: does this anchor at top left? we may need to use draw_texture_ex with pivot... IF pivot anchors.
        // otherwise we will need more math.

    }

    pub fn reset(&mut self) {
        self.pos = Vec2 { x: window::screen_width(), y: window::screen_height()} / 2.; // Center the ball

        // Random direction!
        let mut rand_x: f32 = 1.;

        if rand::gen_range(0, 2) == 0 {
            rand_x *= -1.;
        }

        // let randomY: f32 = (float)random.NextDouble() * 4 - 2;
        // let randomY: f32 = macroquad::rand::RandomRange(-2, 2);
        let rand_y = rand::gen_range(-2, 2) as f32; 
        // let dirVector = Vector2.Normalize(new Vector2(randomX, randomY)) * startSpeed;	// Ensure consistent start velocity
        self.dir = Vec2{ x: rand_x, y: rand_y }.normalize() * START_SPEED;	// Ensure consistent start velocity
        

    }

/*
		public Vector2 Position {
			get { return position; }
		}

		public Vector2 DirVector {
			get { return dirVector;  }
		}

		public Vector2 Size {
			get { return new Vector2(ball.Width, ball.Height); }
		}
*/


}