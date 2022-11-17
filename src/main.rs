use fbot_vss_rust::{Robot, Team, Ball, Point, Origin, Obstacle};

use std::{thread, time};

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(time::Duration::from_millis(50))
        .open().expect("Failed to open port");
    

    let ball = Ball::new(Origin::SSLVISION);
    let robot = Robot::new(Origin::SSLVISION, 0, Team::Yellow);
    let robot_blue = Robot::new(Origin::SSLVISION, 0, Team::Blue);
    loop {

        let obstacle = Obstacle::new(robot_blue.point(), 10.0);

        let speeds = robot.potential_field(ball.point(), obstacle);

        


        let speeds: (u8, u8) = ((speeds.0 + 127.0) as u8, (speeds.1 + 127.0) as u8);

        println!("Speed: {:?}", speeds);

        let command: [u8; 5] = [5, 0, speeds.0, speeds.1, 15]; 

        port.write(&command).expect("Write failed!");
        thread::sleep(time::Duration::from_millis(5));
    }
}
