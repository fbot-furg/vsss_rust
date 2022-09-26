use fbot_vss_rust::{Robot, Team, Ball, Point};

fn main() {
    let goalie = Robot::new(0, Team::Yellow);
    let ball = Ball::new();

    // Goalie Logic
    loop {
        let goal_point = Point::new(0.7, ball.y());
        goalie.go_to2(goal_point);
    }   
}