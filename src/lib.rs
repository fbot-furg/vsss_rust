
mod robot;
mod ball;
mod potential_field;

pub use robot::Robot;
pub use ball::Ball;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Team{
    Yellow,
    Blue
}

#[derive(Debug)]
pub struct Point {
    x: f64,
    y: f64
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: x,
            y: y
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn orientation_to(&self, p: &Point) -> f64 {
        let x = p.x - self.x;
        let y = p.y - self.y;

        y.atan2(x)
    }

    pub fn distance_to(&self, p: &Point) -> f64 {
        let x = p.x - self.x;
        let y = p.y - self.y;

        (x*x + y*y).sqrt()
    }

    pub fn angle(&self) -> f64 {
        self.y.atan2(self.x)
    }
}