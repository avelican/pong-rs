use crate::ball::Ball;
use crate::paddle::Paddle;

use macroquad::window;

pub struct Collision;

impl Collision {
    pub fn collide_ball_wall(ball: &Ball) -> bool {
        ball.pos.y + ball.size.y / 2.0 >= window::screen_height()
            || ball.pos.y <= ball.size.y / 2.0
    }

    pub fn collide_ball_left_goal(ball: &Ball) -> bool {
        ball.pos.x - ball.size.x / 2.0 < 0.0
    }

    pub fn collide_ball_right_goal(ball: &Ball) -> bool {
        ball.pos.x + ball.size.x / 2.0 > window::screen_width()
    }

    pub fn collide_ball_paddle(ball: &Ball, paddle: &Paddle) -> bool {
        let first_pos = ball.pos;
        let first_size = ball.size;
        let second_pos = paddle.pos;
        let second_size = paddle.size;

        let first_width = first_size.x;
        let first_height = first_size.y;
        let second_width = second_size.x;
        let second_height = second_size.y;

        let atop = first_pos.y - first_height / 2.0;
        let abot = first_pos.y + first_height / 2.0;
        let aleft = first_pos.x - first_width / 2.0;
        let aright = first_pos.x + first_width / 2.0;

        let btop = second_pos.y - second_height / 2.0;
        let bbot = second_pos.y + second_height / 2.0;
        let bleft = second_pos.x - second_width / 2.0;
        let bright = second_pos.x + second_width / 2.0;

        atop <= bbot && abot >= btop && aleft <= bright && aright >= bleft
    }
}
