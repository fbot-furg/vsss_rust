use fbot_rust_client::{ball};
use crate::Point;
pub struct Ball {} 

impl Ball {
    pub fn new() -> Self {
        Self {}
    }

    pub fn x(&self) -> f64 {
        ball().x
    }

    pub fn y(&self) -> f64 {
        ball().y
    }

    pub fn point(&self) -> Point {
        Point::new(self.x(), self.y())
    }

    pub fn control_point(&self) -> Point{
        let ball = ball();
        let ball_point = Point::new(ball.x, ball.y);
    
        let goal_point = Point::new(-0.75, 0.0);
        let cp = 0.5;
        
        let orientation_to_goal = ball_point.orientation_to(&goal_point);
        
        let cp_x = orientation_to_goal.cos() * cp;
        let cp_y = orientation_to_goal.sin() * cp;
    
        Point::new(ball.x + cp_x, ball.y + cp_y)
        
    }
}