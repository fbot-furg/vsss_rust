
mod robot;
mod ball;
mod potential_field;
mod uvf;

pub use robot::Robot;
pub use ball::Ball;
pub use uvf::UVF;
pub use potential_field::PotentialField;

pub type WheelsSpeeds = (f64, f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Team{
    Yellow,
    Blue
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Origin{
    FIRASIM,
    SSLVISION
}

pub struct Goal {
    pub point: Point
}

impl Goal {
    pub fn new(point: Point) -> Self {
        Self {
            point: point
        }
    }
}

pub struct Obstacle {
    pub point: Point,
    pub radius: f64
}

impl Obstacle {
    pub fn new(point: Point, radius: f64) -> Self {
        Self {
            point: point,
            radius: radius
        }
    }
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

    pub fn cos(&self) -> f64 {
        self.x.cos()
    }

    pub fn sin(&self) -> f64 {
        self.y.sin()
    }

    pub fn translate(&self, point: &Point) -> Point {
        Point::new(self.x - point.x, self.y - point.y)
    }

    pub fn rotate(&self, angle: f64) -> Self {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();

        Self::new(x, y)
    }
}