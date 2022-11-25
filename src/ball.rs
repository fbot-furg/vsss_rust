use crate::{Vector, Origin};
use vsss_rust_client::{FIRASIM, SSLVISION};
pub struct Ball {
    origin: Origin
} 

impl Ball {
    pub fn new(origin: Origin) -> Self {
        Self { origin }
    }

    pub fn x(&self) -> f64 {
        match self.origin {
            Origin::FIRASIM => FIRASIM.ball().x * 100.0,
            Origin::SSLVISION => SSLVISION.ball().x.into()
        }
    }

    pub fn y(&self) -> f64 {
        match self.origin {
            Origin::FIRASIM => FIRASIM.ball().y * 100.0,
            Origin::SSLVISION => SSLVISION.ball().y.into()
        }
    }

    pub fn point(&self) -> Vector {
        Vector::new(self.x(), self.y())
    }

    pub fn control_point(&self) -> Vector{
        let ball = FIRASIM.ball();
        let ball_point = Vector::new(ball.x, ball.y);
    
        let goal_point = Vector::new(-0.75, 0.0);
        let cp = 0.5;
        
        let orientation_to_goal = ball_point.orientation_to(&goal_point);
        
        let cp_x = orientation_to_goal.cos() * cp;
        let cp_y = orientation_to_goal.sin() * cp;
    
        Vector::new(ball.x + cp_x, ball.y + cp_y)
        
    }
}