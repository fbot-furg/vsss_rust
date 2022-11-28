use vsss_rust_client::{FIRASIM, SSLVISION, fira_protos, ssl_vision_protos};
use crate::{TEAM, Vector, Team, Goal, Obstacle, Origin, WheelsSpeeds};

enum Rotation {
    Clockwise,
    CounterClockwise
}

#[derive(Debug)]
pub struct Robot {
    origin: Origin,
    id: u32,
    team: Team,
    wheels_speeds: WheelsSpeeds,
    orientation_kp: f64,
    orientation_ki: f64,
    orientation_kd: f64,
    distance_kp: f64,
    distance_ki: f64,
    distance_kd: f64,
    distance_error_sum: f64,
    distance_error_last: f64,
    angle_error_sum: f64,
    angle_error_last: f64,
}

impl Robot {
    pub fn new(origin: Origin, id: u32) -> Self {
        Self { 
            origin, 
            id, 
            team: *TEAM, 
            wheels_speeds: (0.0, 0.0),
            orientation_kp: 10.0,
            orientation_ki: 0.0,
            orientation_kd: 0.0,
            distance_kp: 10.0,
            distance_ki: 1.0,
            distance_kd: 1.0,
            distance_error_sum: 0.0,
            distance_error_last: 0.0,
            angle_error_sum: 0.0,
            angle_error_last: 0.0,
        }
    }

    pub fn set_distance_pid(&mut self, pid: (f64, f64, f64)) -> &mut Self {
        self.distance_kp = pid.0;
        self.distance_ki = pid.1;
        self.distance_kd = pid.2;
        self
    }

    pub fn set_orientation_pid(&mut self, pid: (f64, f64, f64)) -> &mut Self {
        self.orientation_kp = pid.0;
        self.orientation_ki = pid.1;
        self.orientation_kd = pid.2;
        self
    }

    pub fn get_command(&self) -> fira_protos::Command {
        let speeds = self.wheels_speeds;

        fira_protos::Command::new(self.id, self.team.to_bool(), speeds.0, speeds.1)
    }

    fn set_wheels_speeds(&mut self, speed: WheelsSpeeds) -> &mut Self {
        self.wheels_speeds = (speed.0.clamp(-100.0, 100.0), speed.1.clamp(-100.0, 100.0));
        self
    }

    fn robot_fira(&self) -> fira_protos::Robot{
        match self.team {
            Team::Yellow => FIRASIM.yellow_robot(&self.id),
            Team::Blue => FIRASIM.blue_robot(&self.id)
        }
    }

    fn robot_vision(&self) -> ssl_vision_protos::SslDetectionRobot{
        match self.team {
            Team::Yellow => SSLVISION.yellow_robot(&self.id),
            Team::Blue => SSLVISION.blue_robot(&self.id)
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn team(&self) -> Team {
        self.team
    }

    pub fn x(&self) -> f64 {
        match self.origin {
            //FIRASIM returns values in m, converting to cm
            Origin::FIRASIM => self.robot_fira().x * 100.0,
            Origin::SSLVISION => self.robot_vision().x.into()
        }
    }

    pub fn y(&self) -> f64 {
        match self.origin {
            //FIRASIM returns values in m, converting to cm
            Origin::FIRASIM => self.robot_fira().y * 100.0,
            Origin::SSLVISION => self.robot_vision().y.into()
        }
    }

    pub fn orientation(&self) -> f64 {
        match self.origin {
            Origin::FIRASIM => self.robot_fira().orientation,
            Origin::SSLVISION => {
                if let Some(orientation) = self.robot_vision().orientation {
                    orientation.into()
                } else {
                    0.0
                }
            }
        }
    }

    pub fn reverse_orientation(&self) -> f64 {
        self.orientation() + std::f64::consts::PI
    }

    pub fn point(&self) -> Vector {
        Vector::new(self.x(), self.y())
    }

    pub fn control_point(&self) -> Vector {
        let cp = 1.0;
        let (x, y, orientation) = (self.x(), self.y(), self.orientation());

        let cp_x = orientation.cos() * cp;
        let cp_y = orientation.sin() * cp;

        Vector::new(x + cp_x, y + cp_y)
    }

    // DEPRECATED
    // pub fn go_to(&self, target_point: Vector) -> WheelsSpeeds{
        
    //     // Se o Robo estiver muito proximo do ponto, nao faz nada
    //     // if self.point().distance_to(&target_point) < 0.1 {
    //     //     self.set_speed(0.0, 0.0);
    //     //     return;
    //     // }

    //     let target_angle = self.point().orientation_to(&target_point);
    //     let robot_angle = self.orientation();
        
    //     let mut angle_error = target_angle - robot_angle;

    //     // Normaliza o angulo
    //     if angle_error > std::f64::consts::PI {
    //         angle_error -= 2.0 * std::f64::consts::PI;
    //     } else if angle_error < -std::f64::consts::PI {
    //         angle_error += 2.0 * std::f64::consts::PI;
    //     }

    //     // Calcula a velocidade angular
    //     let angular_speed = angle_error * self.orientation_kp;

    //     // Calculata velocidade das rodas
    //     let wheel_left = self.robot_speed - angular_speed;
    //     let wheel_right = self.robot_speed + angular_speed;

    //     (wheel_left, wheel_right)
    // }

    
    pub fn go_to2(&mut self, target_point: Vector, ball_point: Vector) -> &mut Self {

        // Se o Robo estiver muito proximo do ponto, nao faz nada
        // if self.point().distance_to(&target_point) < 1.0 {
        //     self.distance_error_sum = 0.0;
        //     self.angle_error_sum = 0.0;
        //     return self.set_wheels_speeds((0.0, 0.0))
        // }

        let target_angle = self.point().orientation_to(&target_point);
        
        let robot_angle = self.orientation();
        let robot_angle_reverse = self.reverse_orientation();

        let distance_error = self.point().distance_to(&ball_point);
        self.distance_error_sum += distance_error / 100.0;
        
        let angle_error = target_angle - robot_angle;
        let angle_error_reverse = target_angle - robot_angle_reverse;

        //Controle de qual lado ele vai escolher para ir
        let (mut smallest_angle_error, rotation) = if angle_error.abs() < angle_error_reverse.abs() {
            (angle_error, Rotation::Clockwise)
        } else {
            (angle_error, Rotation::Clockwise)
        };

        if smallest_angle_error > std::f64::consts::PI {
            smallest_angle_error -= 2.0 * std::f64::consts::PI;
        } else if smallest_angle_error < -std::f64::consts::PI {
            smallest_angle_error += 2.0 * std::f64::consts::PI;
        }

        println!("Angle Error: {}", smallest_angle_error);

        if smallest_angle_error.abs() < 0.05 {
            self.angle_error_sum = 0.0;
        }

        self.angle_error_sum += smallest_angle_error / 100.0;

        //calcula velocidade de rotacao usando PID
        let mut rotation_speed = smallest_angle_error * self.orientation_kp; 
        rotation_speed += self.angle_error_sum * self.orientation_ki;
        rotation_speed += (smallest_angle_error - self.angle_error_last) * self.orientation_kd;

        let rotation_speed = rotation_speed.clamp(-100.0, 100.0);
        
        let mut speed = distance_error * self.distance_kp;
        speed += self.distance_error_sum * self.distance_ki;
        speed += (distance_error - self.distance_error_last) * self.distance_kd;

        speed *= match rotation {
            Rotation::Clockwise => 1.0,
            Rotation::CounterClockwise => -1.0
        };

        let speed = speed.clamp(-100.0, 100.0);


        self.distance_error_last = distance_error;
        self.angle_error_last = smallest_angle_error;

        println!("speed: {}, rotation_speed: {}", speed, rotation_speed);

    
        let wheels_speeds = match rotation {
            Rotation::Clockwise => (speed - rotation_speed, speed + rotation_speed),
            Rotation::CounterClockwise => (speed - rotation_speed, speed + rotation_speed)
        };
        
        return self.set_wheels_speeds(wheels_speeds)
    }
}