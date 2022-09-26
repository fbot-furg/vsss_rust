use fbot_vss_rust::{Robot, Team, Ball, Point};
use flo_curves::{bezier::Curve, Coord2, BezierCurve};
use std::{thread, time};

fn main() {
    let goalie = Robot::new(0, Team::Yellow);
    let ball = Ball::new();

    let cp_goalie = goalie.control_point();
    let cp_ball = ball.control_point();


    let curve = Curve {
        start_point: Coord2 (goalie.x(), goalie.y()),
        end_point: Coord2 (ball.x(), ball.y()),
        control_points: (Coord2 (cp_goalie.x(), cp_goalie.y()), Coord2 (cp_ball.x(), cp_ball.y()) )
    };

    for i in 0..50 { 
        let pos: f64 = i as f64 / 20.0;
        
        let point = curve.point_at_pos(pos);
        let (x, y) = (point.0, point.1);

        //TODO async await
        goalie.go_to(Point::new(x, y));
    }

    goalie.set_speed(1000.0, 1000.0);

    thread::sleep(time::Duration::from_millis(200));

    goalie.set_speed(0.0, 0.0);
}
