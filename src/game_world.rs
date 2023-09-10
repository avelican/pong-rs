use crate::ball::Ball;
use crate::paddle::Paddle;
use crate::particle_system::ParticleSystem;
use crate::collision::Collision;

use macroquad::audio::PlaySoundParams;
use macroquad::text::TextParams;
use macroquad::text::load_ttf_font;
use macroquad::texture::Texture2D;
use macroquad::texture::draw_texture;
use macroquad::texture::draw_texture_ex;
use macroquad::texture::load_texture;
use macroquad::texture::DrawTextureParams;
use macroquad::color::Color;
use macroquad::color;
use macroquad::audio::Sound;
use macroquad::audio::load_sound;
use macroquad::audio::play_sound;
use macroquad::audio::play_sound_once;
use macroquad::audio::stop_sound;


use macroquad::input::KeyCode;
use macroquad::input::is_key_down;
use macroquad::input::is_key_pressed;
use macroquad::window;
use macroquad::window::clear_background;
use macroquad::math::Vec2;
use macroquad::math::Rect;

use macroquad::text::Font;
use macroquad::text::draw_text;
use macroquad::text::draw_text_ex;


pub struct GameWorld {
    game_state: i32, // 0 = ready, 1 = playing, 2 = paused, 3 = finished
    ball: Ball,
    heart_tex: Texture2D,
    bg_tex: Texture2D,
    font: Font,
    left_pad: Paddle,
    right_pad: Paddle,
    pub part_sys: ParticleSystem,
    bg_color: Color,
    text_color: Color,
    hit_color: Color,
    expected_ball_position: f32,
    ball_slope: f32,
    song: Sound,
    oh_yeah: Sound,
    player_win_text: String,
    lives: i32,
    // maxParticles: i32,
    multiplayer: bool,
}

impl GameWorld {

    async fn new() -> Self {

        // game_state = 0;
        let lives = 3;

        // TODO: putting await after every thing we instantiate is stupid
        // I'd rather make an Assets / AssetLoader class and just do everything there...

        let heart = load_texture("heart.png").await.unwrap(); // ball
        let bg = load_texture("star-bg").await.unwrap(); // fancy parallax bg
        // let spritefont = Content.Load<SpriteFont>("SpriteFont1");
        let oh_yeah = load_sound("oh-yeah.wav").await.unwrap(); // disturbing easter egg
        let song = load_sound("pong_soundtrack").await.unwrap();

        let font = load_ttf_font("consolas.ttf").await.unwrap();

        let left_pad = Paddle::new(true, lives).await; // left = true
        let right_pad = Paddle::new(false, lives).await;
        let ball = Ball::new().await;

        let part_sys = ParticleSystem::new().await;
        
        let text_color = color::WHITE;
        let bg_color = color::BLACK;
        let hit_color = color::RED;

        let game_world = Self {
            game_state: 0, // 0 = ready, 1 = playing, 2 = paused, 3 = finished
            ball,
            heart_tex: heart,
            bg_tex: bg,
            font,
            left_pad,
            right_pad,
            part_sys,
            bg_color,
            text_color,
            hit_color,
            expected_ball_position: 0.,
            ball_slope: 0.,
            song,
            oh_yeah,
            player_win_text: String::from(""), // todo: not idiomatic eh
            lives,
            // maxParticles: i32,
            multiplayer: false, // TODO: technically unknown at this point, but I don't wanna use Option
        };
        game_world.play_song();
        game_world

    }

    pub fn screen() -> Vec2 {
        Vec2{
            x: window::screen_width(),
            y: window::screen_height(),
        }
    }

    fn play_song(&self) {
        stop_sound(&self.song);
        play_sound(&self.song, PlaySoundParams { looped: true, volume: 1.});
        
    }

     fn handle_input(&mut self) {

        // TODO: make game_state an enum
        // TODO: if -> match

            // ready
        if self.game_state == 0 { 
            if is_key_pressed(KeyCode::Space) {		// start single-player game
                self.game_state = 1;
                self.multiplayer = false;
            }

            if is_key_pressed(KeyCode::M) {		// start multi-player game
                self.game_state = 1;
                self.multiplayer = true;
            }

        }

        if self.game_state == 1 {

            if is_key_pressed(KeyCode::E) {		// fun bonus. E for EXPLODE
                self.part_sys.play(Vec2{ x: window::screen_width(), y: window::screen_height() } / 2., 10., 0.99, 20);
            }

            if is_key_pressed(KeyCode::P) {		// pause game
                self.game_state = 2;	
            }

            self.left_pad.handle_input();

            if self.multiplayer {
                self.right_pad.handle_input();
            }
            // paused
        } else if self.game_state == 2 {
            if is_key_pressed(KeyCode::P) {	// unpause
                self.game_state = 1;
            }
        }

        if is_key_down(KeyCode::R) || self.game_state == 3 && is_key_down(KeyCode::Space) {
            self.reset();		// R for Reset at any time
            // TODO: add confirmation
        }

        if is_key_pressed(KeyCode::O) {
            play_sound_once(&self.oh_yeah);
        }
    }

     fn update(&mut self) {
        if self.game_state == 1 {
            self.ball.update();
            self.left_pad.update();
            self.right_pad.update();
            self.part_sys.update();
            self.collide_all_the_things();		// just that

            if ! self.multiplayer {
                self.follow_ball(false);
                //follow_ball(true);  // uncomment this line for an epic AI vs AI mode that was cut from the final release
            }

        }
    }


     fn draw(&self) {

        clear_background(self.bg_color);

        // This line is just the defaults, except for SamplerState.PointClamp, which gives us really nice pixely graphics
        // spriteBatch.Begin(SpriteSortMode.Deferred, BlendState.AlphaBlend, SamplerState.PointWrap, DepthStencilState.None, RasterizerState.CullCounterClockwise);

        self.silly_bgs();		// crazy disco parallax

            // start screen
        if self.game_state == 0 {
            self.write_stuff("HELLO THIS IS PONG", Vec2::new(window::screen_height(), window::screen_width()) / 2., 0.7);
            self.write_stuff("Space for Single Player \n\n\n M for Mulitplayer", Vec2::new(window::screen_height(), window::screen_width()) / 2., 0.5);
        }


            // playing
        if self.game_state == 1 {

            self.left_pad.draw();
            self.right_pad.draw();
            self.ball.draw();

            self.draw_hearts();

            // Lives left
            self.write_stuff(&self.left_pad.lives.to_string(), Vec2::new(window::screen_height()/5., window::screen_width()) / 2., 1.0);
            self.write_stuff(&format!("{}", self.right_pad.lives),  Vec2::new(window::screen_height() * (9./5.), window::screen_width()) / 2., 1.0);
            // TODO: wasn't sure which "convert the goddamn number to a string" syntax was least bad
            
            self.part_sys.draw();		// sparkles

        }

        // paused
        if self.game_state == 2 {
            self.write_stuff("PAUSED", Vec2::new(window::screen_height(), window::screen_width()) / 2., 1.0);
        }

        // win
        if self.game_state == 3 {
            self.write_stuff(&self.player_win_text,  Vec2::new(window::screen_height(), window::screen_width()) / 2., 0.7);
        }

        // spriteBatch.End();

    }


     pub fn end_game(&mut self, left: bool) {
        self.game_state = 3;
        if left {		// the losing paddle calls this method, so other player wins
            self.player_win_text = format!("Right Player Won!");
        }else{
            self.player_win_text = format!("Left Player Won!");
        }
    }


     fn collide_all_the_things(&mut self) {
        //all of the collision handles in here
        
        if Collision::collide_ball_wall(&self.ball) {
            // self.ball.bounce_wall(self);
            let ball: &mut Ball = &mut self.ball;
            ball.bounce_wall(&mut self.part_sys);
            
        }

        if Collision::collide_ball_paddle(&self.ball, &self.left_pad) {
            self.ball.bounce_paddle(self.left_pad.pos);
        }

        if Collision::collide_ball_paddle(&self.ball, &self.right_pad) {
            self.ball.bounce_paddle(self.right_pad.pos);
        }

        if Collision::collide_ball_left_goal(&self.ball) {
            self.left_pad.lose_life(self);
            self.ball.reset();
        }

        if Collision::collide_ball_right_goal(&self.ball) {
            self.right_pad.lose_life(self);
            self.ball.reset();
        }

    }


     fn follow_ball(&mut self, left: bool) {

        // There is probably a really elegant one liner to do this, but this is the best we could come up with.. :P
        self.ball_slope = self.ball.dir.y/self.ball.dir.x;


        // Calculate the "end Y" position if the ball just kept going (through the walls)
        let end_y: f32;
        if left {
            end_y = (self.left_pad.pos.x - self.left_pad.size.x - self.ball.pos.x) * self.ball_slope + self.ball.pos.y;
        } else {
            end_y = (self.right_pad.pos.x - self.right_pad.size.x - self.ball.pos.x) * self.ball_slope + self.ball.pos.y;
        }

        // Are we in an even or odd screen? ( Imagine infinite screens stacked vertically in both directions )
        let flip: bool = (end_y / window::screen_width() ) as i32 % 2  == 1;

        let mut real_end_y: f32 = end_y % window::screen_width();
        real_end_y = real_end_y.abs();

        // If the ball bounces, the Y gets inverted!
        if flip {
            real_end_y *= -1.;
        }

        if real_end_y < 0. {
            real_end_y += window::screen_width();
        }

        self.expected_ball_position = real_end_y; // Variable used for debugging, back in the day...

        // Finally, move the paddle!
        if left {
            if (self.expected_ball_position - self.left_pad.pos.y).abs() > self.left_pad.max_speed {
                self.left_pad.move_AI(self.expected_ball_position > self.left_pad.pos.y);
            }
        } else {
            if (self.expected_ball_position - self.right_pad.pos.y).abs() > self.right_pad.max_speed {
                self.right_pad.move_AI(self.expected_ball_position > self.right_pad.pos.y);
            }
        }

    }



     fn reset(&mut self) {
        self.play_song();	// aww yeah
        self.left_pad.reset();
        self.right_pad.reset();
        self.ball.reset();
        self.game_state = 0;
    }

    // Method to make code prettier in other places
     fn write_stuff(&self, stuff: &str, pos: Vec2, scale: f32) {
        // let textSize: Vec2 = spritefont.MeasureString(stuff);
        // ^ ???
        // .drawString(spritefont, stuff, pos, text_color, 0, textSize/2, scale, SpriteEffects.None, 0);

        // draw_text(text: &str, x: f32, y: f32, font_size: f32, color: Color);
        // we must use _ex because we want to use our own font!

        //  draw_text_ex(text: &str, x: f32, y: f32, params: TextParams<'_>)
        draw_text_ex(
            stuff,
            pos.x,
            pos.y,
            TextParams {
                font_size: scale as u16,
                font: Some(&self.font),
                ..Default::default()
            },
        );

    }
    

     fn draw_hearts(&self) {

        // for (int i = 0; i < left_pad.Lives; i++)
        //     .draw(heart, Self::screen / 6 + new Vec2(i * (heart.Width * 6), 0), null, color::RED, 0, Vec2.One * heart.Width/ 2, Vec2.One * 4, SpriteEffects.None, 0);

        // for (int i = 0; i < right_pad.Lives; i++)
        //     .draw(heart, Self::screen * 5/ 6 - new Vec2(i * (heart.Width * 6), 0), null, new Color(0,0.3f,1), 0, Vec2.One * heart.Width / 2, Vec2.One * 4, SpriteEffects.None, 0);
        
        for i in 0..self.left_pad.lives {
            // draw(
            //     heart, 
            //     Self::screen() / 6. + Vec2::new(i as f32 * (self.heart_tex.width() * 6.), 0.), 
            //     null, 
            //     color::RED, 
            //     0, 
            //     Vec2::ONE * self.heart_tex.width()/ 2., 
            //     Vec2::ONE * 4., SpriteEffects.None, 
            //     0
            // );

            let pos = Self::screen() / 6. + Vec2::new(i as f32 * (self.heart_tex.width() * 6.), 0.);

            draw_texture(&self.heart_tex, pos.x, pos.y, color::RED);

        }
        for i in 0..self.right_pad.lives {
            // draw(
            //     heart, 
            //     Self::screen() * 5./6. - Vec2::new(i as f32 * (self.heart_tex.width() * 6.), 0.),
            //     null,
            //     new Color(0,0.3f,1), 0, 
            //     Vec2.One * heart.Width / 2, 
            //     Vec2.One * 4, 
            //     SpriteEffects.None, 
            //     0
            // );

            let pos = Self::screen() * 5./6. - Vec2::new(i as f32 * (self.heart_tex.width() * 6.), 0.);

            draw_texture(&self.heart_tex, pos.x, pos.y, Color::new(0.,0.3,1., 1.));

        }
    }


     fn silly_bgs(&self) {
        // TODO

        // NOTE: macroquad does not have a way to draw repeating textures. (WTF?)
        // Well, it does, but it looks like this https://github.com/not-fl3/macroquad/issues/191
        // So we have to use for loops...

        // // let whole_screen: Rectangle = Rectangle::new(0, 0, window::screen_height() as i32 * 3, window::screen_width() as i32 * 3);
        // let scr: Vec2 = Vec2{x: window::screen_width(), y: window::screen_height()};

        // .draw(bg, -scr - ball.pos / 16, whole_screen, color::RED, 0, Vec2.One, 8, SpriteEffects.None, 0);
        // .draw(bg, -scr - ball.pos / 8, whole_screen, color::GREEN, 0, Vec2.One, 16, SpriteEffects.None, 0);
        // .draw(bg, -scr - ball.pos / 4, whole_screen, color::BLUE, 0, Vec2.One, 32, SpriteEffects.None, 0);
        // position depends on some factor of the opposite of the ball's position... which correlates with scale...  
        // In other words: crazy parallax effect!
        self.silly_bg(16., color::RED);
        self.silly_bg(8., color::GREEN);
        self.silly_bg(4., color::BLUE);
        

    }

    fn silly_bg(&self, scale: f32, color: Color) {
        
        // forgive me for this atrocity
        // it was like 1 line of code in XNA ...

        let tex = &self.bg_tex;
        // let scr = Self::screen();
        let pos = - Self::screen() - self.ball.pos / scale;
        let size = Vec2{ x: tex.width() * scale, y: tex.height() * scale};
        for x in (0..window::screen_width() as usize).step_by(tex.width() as usize) {
            for y in (0..window::screen_height() as usize).step_by(tex.height() as usize) {
                draw_texture_ex(&tex, pos.x + x as f32, pos.y + y as f32, color::WHITE, DrawTextureParams {
                    // source: None,
                    dest_size: Some(size),
                    // rotation: 0.0,
                    // flip_x: false,
                    // flip_y: false,
                    ..Default::default()
                });

                // draw_texture(&tex, x as f32, y as f32, color::WHITE);
            }
        }
    }

}