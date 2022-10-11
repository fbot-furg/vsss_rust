use fbot_vss_rust::{Robot, Team, Ball, Point};


fn main() {
    let ball = Ball::new();

    let goalie = Robot::new(0, Team::Yellow);
    let neymar = Robot::new(1, Team::Yellow);
    
    //main loop
    loop {
        goalie.go_to2(Point::new(-0.7, ball.y()));

        neymar.go_to2(Point::new(0.7, ball.y()));
    }   
}
