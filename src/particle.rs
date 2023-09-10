use macroquad::rand::RandomRange;
// use macroquad::texture::DrawTextureParams;
// use macroquad::texture::Texture2D;
// use macroquad::texture::load_texture;
// use macroquad::texture::draw_texture;
// use macroquad::texture::draw_texture_ex;


use macroquad::math::Vec2;
// use macroquad::color;
use macroquad::color::Color;


// use std::f32::consts::PI;

// use crate::util::randf;

pub struct Particle {
    // tex: Texture2D, // moved to ParticleSystem
    pub color: Color,
    pub pos: Vec2,
    pub dir: Vec2,
    pub drag: f32,
}

impl Particle {
    pub fn new(/*content: &ContentManager,*/ pos: Vec2, dir: Vec2, drag: f32) -> Self {
        let pos = pos + dir * 4.0;
        let color = Color::new(RandomRange::gen_range(0.0, 1.0), gen_range(0.0, 1.0), gen_range(0.0, 1.0), 1.0);
        // let tex = load_texture("particle.png").await.unwrap();

        Particle {
            // tex,
            color,
            pos,
            dir,
            drag,
        }
    }

    pub fn update(&mut self) {
        self.pos += self.dir;
        self.pos += random_direction(0.1);
        self.dir *= self.drag;
    }

    // // NOTE: tex moved to ParticleSystem
    // pub fn size(&self) -> Vec2 {
    //     Vec2::new(self.tex.width(), self.tex.height())
    // }
}

impl Default for Particle {
    fn default() -> Self {
        Self::new(Vec2::ZERO, Vec2::ZERO, 0.)
    }
}

fn random_direction(length: f32) -> Vec2 {
    let rand_x: f32 = gen_range(-1.0, 1.0);
    let rand_y: f32 = gen_range(-1.0, 1.0);

    Vec2::normalize(Vec2::new(rand_x, rand_y)) * length
}

// cannot be imported directly my ASS
fn gen_range(a: f32, b: f32) -> f32 {
    return RandomRange::gen_range(a, b);
}