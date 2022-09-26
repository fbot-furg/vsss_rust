use fbot_rust_client::fira_protos;
use fbot_rust_client::{ball, yellow_robot, blue_robot};

const ORIENTATION_KP: f64 = 20.0;
const ROBOT_SPEED: f64 = 20.0;

#[derive(Debug, PartialEq)]
pub enum Team{
    Yellow,
    Blue
}

// TODO
// Implement cord funcitons inside point
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
}

#[derive(Debug)]
pub struct Robot {
    id: u32,
    team: Team,
}

impl Robot {
    pub fn new(id: u32, team: Team) -> Self {
        Self {
            id: id,
            team: team,
        }
        }

    fn robot(&self) -> fira_protos::Robot{
        match self.team {
            Team::Yellow => yellow_robot(&self.id).unwrap(),
            Team::Blue => blue_robot(&self.id).unwrap()
        }
    }

    pub fn x(&self) -> f64 {
        self.robot().x
    }

    pub fn y(&self) -> f64 {
        self.robot().y
    }

    pub fn orientation(&self) -> f64 {
        self.robot().orientation
    }

    pub fn reverse_orientation(&self) -> f64 {
        self.robot().orientation + std::f64::consts::PI
    }

    pub fn point(&self) -> Point {
        Point::new(self.x(), self.y())
    }

    pub fn control_point(&self) -> Point {
        let cp = 1.0;
        let (x, y, orientation) = (self.x(), self.y(), self.orientation());

        let cp_x = orientation.cos() * cp;
        let cp_y = orientation.sin() * cp;

        Point::new(x + cp_x, y + cp_y)
    }

    // TODO
    // Extrar a logica de envio de comandos para outro ponto
    pub fn set_speed(&self, wheel_left: f64, wheel_right: f64) {
        let commands = fira_protos::Commands {
            robot_commands: vec![
                fira_protos::Command {
                    id: 0,
                    yellowteam: self.team == Team::Yellow,
                    wheel_left: wheel_left,
                    wheel_right: wheel_right,
                },
            ]
        };

        fbot_rust_client::send_command(commands);
    }

    pub fn go_to(&self, target_point: Point) {
        
        // Se o Robo estiver muito proximo do ponto, nao faz nada
        if self.point().distance_to(&target_point) < 0.1 {
            self.set_speed(0.0, 0.0);
            return;
        }

        let target_angle = self.point().orientation_to(&target_point);
        let robot_angle = self.orientation();
        
        let mut angle_error = target_angle - robot_angle;

        // Normaliza o angulo
        if angle_error > std::f64::consts::PI {
            angle_error -= 2.0 * std::f64::consts::PI;
        } else if angle_error < -std::f64::consts::PI {
            angle_error += 2.0 * std::f64::consts::PI;
        }

        // Calcula a velocidade angular
        let angular_speed = angle_error * ORIENTATION_KP;

        // Calculata velocidade das rodas
        let wheel_left = ROBOT_SPEED - angular_speed;
        let wheel_right = ROBOT_SPEED + angular_speed;

        // Send command
        self.set_speed(wheel_left, wheel_right);
    }

    pub fn go_to2(&self, target_point: Point) {
        
        // Se o Robo estiver muito proximo do ponto, nao faz nada
        if self.point().distance_to(&target_point) < 0.05 {
            self.set_speed(0.0, 0.0);
            return;
        }

        let target_angle = self.point().orientation_to(&target_point);
        let robot_angle = self.orientation();
        let robot_angle_reverse = self.reverse_orientation();
        
        let angle_error = target_angle - robot_angle;
        let angle_error_reverse = target_angle - robot_angle_reverse;

        let (mut smallest_angle_error, speed) = if angle_error.abs() < angle_error_reverse.abs() {
            (angle_error, ROBOT_SPEED)
        } else {
            (angle_error_reverse, -ROBOT_SPEED)
        };

        // Normaliza o angulo
        if smallest_angle_error > std::f64::consts::PI {
            smallest_angle_error -= 2.0 * std::f64::consts::PI;
        } else if smallest_angle_error < -std::f64::consts::PI {
            smallest_angle_error += 2.0 * std::f64::consts::PI;
        }
        
        let angular_speed = smallest_angle_error * ORIENTATION_KP;

        // Calcular velocidade das rodas
        let wheel_left = speed - angular_speed;
        let wheel_right = speed + angular_speed;

        // Envia Comando
        self.set_speed(wheel_left, wheel_right);
    }
}

// TODO 
// create struct Ball
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

