use fbot_rust_client::{FIRASIM, fira_protos};
use crate::{Point, Team};

// Teste Kick
// use flo_curves::{bezier::Curve, Coord2, BezierCurve};
// use std::{thread, time};

const ORIENTATION_KP: f64 = 10.0;
const ROBOT_SPEED: f64 = 20.0;

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
            Team::Yellow => FIRASIM.yellow_robot(&self.id),
            Team::Blue => FIRASIM.blue_robot(&self.id)
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn team(&self) -> Team {
        self.team
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
    // Extrair a logica de envio de comandos para outro ponto
    pub fn set_speed(&self, wheel_left: f64, wheel_right: f64) {
        let commands = fira_protos::Commands {
            robot_commands: vec![
                fira_protos::Command {
                    id: self.id,
                    yellowteam: self.team == Team::Yellow,
                    wheel_left: wheel_left,
                    wheel_right: wheel_right,
                },
            ]
        };

        FIRASIM.send_command(commands);
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

    pub fn go_to2(&self, target_point: Point) -> fira_protos::Command {
        
        // Se o Robo estiver muito proximo do ponto, nao faz nada
        if self.point().distance_to(&target_point) < 0.05 {
            // self.set_speed(0.0, 0.0);
            return fira_protos::Command {
                id: self.id,
                yellowteam: self.team == Team::Yellow,
                wheel_left: 0.0,
                wheel_right: 0.0,
            };
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
        fira_protos::Command {
            id: self.id,
            yellowteam: self.team == Team::Yellow,
            wheel_left: wheel_left,
            wheel_right: wheel_right,
        }
        // self.set_speed(wheel_left, wheel_right);
    }

    pub fn kick() {
        // let goalie = Robot::new(0, Team::Yellow);
        // let ball = Ball::new();

        // let cp_goalie = goalie.control_point();
        // let cp_ball = ball.control_point();


        // let curve = Curve {
        //     start_point: Coord2 (goalie.x(), goalie.y()),
        //     end_point: Coord2 (ball.x(), ball.y()),
        //     control_points: (Coord2 (cp_goalie.x(), cp_goalie.y()), Coord2 (cp_ball.x(), cp_ball.y()) )
        // };

        // for i in 0..50 { 
        //     let pos: f64 = i as f64 / 20.0;
            
        //     let point = curve.point_at_pos(pos);
        //     let (x, y) = (point.0, point.1);

        //     //TODO async await
        //     goalie.go_to(Point::new(x, y));
        // }

        // goalie.set_speed(1000.0, 1000.0);

        // thread::sleep(time::Duration::from_millis(200));

        // goalie.set_speed(0.0, 0.0);

    }
}