
mod robot;
mod ball;
mod potential_field;
mod uvf;

use vsss_rust_client::{FIRASIM, fira_protos};

pub use robot::Robot;
pub use ball::Ball;
pub use uvf::UVF;
pub use potential_field::PotentialField;
pub use fira_protos::Command;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long, default_value = "y")]
   team: String
}

#[macro_use]
extern crate lazy_static;

lazy_static!(
    pub static ref TEAM: Team = {
        let args = Args::parse();
        Team::from_str(&args.team)
    };
);

pub type WheelsSpeeds = (f64, f64);

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Team{
    Yellow,
    Blue
}

impl Team {
    pub fn from_str(team: &str) -> Self {
        match team {
            "yellow" => Team::Yellow,
            "y" => Team::Yellow,
            "blue" => Team::Blue,
            "b" => Team::Blue,
            _ => panic!("Invalid team")
        }
    }

    pub fn to_bool(&self) -> bool {
        match self {
            Team::Yellow => true,
            Team::Blue => false
        }
    }

    pub fn enemy_goal_x(&self) -> f64 {
        match self {
            Team::Yellow => 68.0,
            Team::Blue => -68.0
        }
    }
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Origin{
    FIRASIM,
    SSLVISION
}

pub struct Goal {
    pub point: Vector
}

impl Goal {
    pub fn new(point: Vector) -> Self {
        Self {
            point: point
        }
    }
}

#[derive(Debug)]
pub struct Obstacle {
    pub point: Vector,
    pub radius: f64
}

impl Obstacle {
    pub fn new(point: Vector, radius: f64) -> Self {
        Self {
            point: point,
            radius: radius
        }
    }

    //retur enemy team as vector of Obstacle
    pub fn enemy_team_fira() -> Vec<Obstacle> {
        let enemy_team = match *TEAM {
            Team::Yellow => FIRASIM.blue_robots(),
            Team::Blue => FIRASIM.yellow_robots()
        };

        enemy_team.iter().map(|robot| {
            let point = Vector::new(robot.x * 100.0, robot.y * 100.0);
            Obstacle::new(point, 10.0)
        }).collect()
    }

    pub fn point(&self) -> Vector {
        self.point
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vector {
    x: f64,
    y: f64
}

impl Vector {
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

    pub fn orientation_to(&self, p: &Vector) -> f64 {
        let x = p.x - self.x;
        let y = p.y - self.y;

        y.atan2(x)
    }

    pub fn distance_to(&self, p: &Vector) -> f64 {
        let x = p.x - self.x;
        let y = p.y - self.y;

        (x.powi(2) + y.powi(2)).sqrt()
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

    pub fn translate(&self, point: &Vector) -> Vector {
        Vector::new(self.x - point.x, self.y - point.y)
    }

    pub fn rotate(&self, angle: f64) -> Self {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();

        Self::new(x, y)
    }

    pub fn normalize(&self) -> Self {
        let norm = (self.x * self.x + self.y * self.y).sqrt();
        
        Self::new(self.x / norm, self.y / norm)
    }

    pub fn scale(&self, scale: f64) -> Self {
        Self::new(self.x * scale, self.y * scale)
    }

    pub fn add(&self, point: &Vector) -> Self {
        Self::new(self.x + point.x, self.y + point.y)
    }

    pub fn opposite(&self) -> Self {
        Self::new(-self.x, -self.y)
    }
}