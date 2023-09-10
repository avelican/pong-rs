// todo: obviously just use MacroQuad's built in particles...

use std::f32::consts::PI;

use macroquad::color;
use macroquad::math::Vec2;
use macroquad::rand;
use macroquad::texture::DrawTextureParams;
use macroquad::texture::Texture2D;
use macroquad::texture::draw_texture_ex;
use macroquad::texture::load_texture;



use crate::particle::Particle;

const MAX_PARTICLES: usize = 300;

pub struct ParticleSystem {
	// content: ContentManager,
	part_list: Vec<Particle>,
	// max_particles: usize,
	pointer: usize, // we don't need such a big number, but Rust is whining
	tex: Texture2D,
}

impl ParticleSystem {
	pub async fn new(/*content_in: ContentManager,*/ /*max_particles_in: usize */) -> Self {
		let mut part_sys = Self {

			part_list: Vec::with_capacity(MAX_PARTICLES),
			pointer: 0,
			tex: load_texture("particle.png").await.unwrap(),
		};
		// NOTE: We do this the "long way" because our ::default() function should be called every time
		// The vec macro just calls it once and clones it, which is not what we want.
		// We want random colors!
		for _ in 0..MAX_PARTICLES {
			part_sys.part_list.push(Particle::default());
		}
		part_sys
	}

	pub fn play(&mut self, pos: Vec2, strength: f32, drag: f32, number_particles: i32) {
		let mut i = 0;
		while i < number_particles {
			let random_dir = self.random_direction(strength) * self.random_float(0.5, 1.5);
			self.part_list[self.pointer] = Particle::new(/*&self.content, */ pos, random_dir, drag);

			i += 1;
			self.pointer += 1; // we use this to keep track of where we are, when we loop through part_list, replacing old particles as we go along.
			self.pointer %= MAX_PARTICLES;
		}
	}

	pub fn update(&mut self) {
		for p in self.part_list.iter_mut() {
			p.update();
		}
	}

	pub fn draw(&self) {
		for p in self.part_list.iter() {
			self.draw_particle(p);
		}
	}

	// TODO: Inline to draw? Or does the compiler do this?
	pub fn draw_particle(&self, p: &Particle /*, sprite_batch: &mut SpriteBatch */) {
		let tex = &self.tex;
		let rotation = p.dir.y.atan2(p.dir.x) + PI / 2.0;
		let length = p.dir.length();
		let mut dest_size = Vec2::new(tex.width(), tex.height() * length); // TODO did I multiply the right axis?
		dest_size *= 2.;  // NOTE: macroquad has no "scale", so we just double the dest_size 

		// draw_texture(&p.tex, p.pos.x, p.pos.y, color::WHITE);
		let draw_tex_params = DrawTextureParams {
			rotation,
			dest_size: Some(dest_size), // this thing really wants to be a Some
			pivot: Some(Vec2 { x: tex.width() / 2., y: 0. },),
			..Default::default()
		};
		draw_texture_ex(&tex, p.pos.x, p.pos.y, color::WHITE, draw_tex_params);
		
		// NOTE: TODO FIXME
		// Original XNA code takes an origin // Vec2::new(size.x / 2.0, 0.0),
		// Does macroquad have anything like that?
		// It has "pivot" but that only appears to be used for rotation.
		// We may need to do our own math here.
	
		// // XNA CODE
		// spriteBatch.Draw(particle, 
		//     position, 
		//     null, //srcRect
		//     color, 
		//     angle, 
		//     new Vector2(Size().X/2, 0), // origin
		//     new Vector2(dirVector.Length()/2, dirVector.Length() * 2), // scale
		//     SpriteEffects.None, 
		//     0); 
	}
	


	fn random_direction(&self, length: f32) -> Vec2 {
		let rand_x: f32 = rand::gen_range(-1.0, 1.0);
		let rand_y: f32 = rand::gen_range(-1.0, 1.0);

		Vec2::normalize(Vec2 { x: rand_x, y: rand_y }) * length
	}

	fn random_float(&self, a: f32, b: f32) -> f32 {
		rand::gen_range(a, b)
	}

	////////////
	
	// pub fn sparkle(position: Vec2, strength: f32, resistance: f32, number_particles: i32, game_world: &mut GameWorld) {
	// 	game_world.part_sys.play(position, strength, resistance, number_particles); // Particle effects!
	// }

	pub fn sparkle(&mut self, position: Vec2, strength: f32, resistance: f32, number_particles: i32) {
		self.play(position, strength, resistance, number_particles); // Particle effects!
	}

	
	
}