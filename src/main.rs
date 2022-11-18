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

        let obstacle = Obstacle::new(robot_blue.point(),  10.0);

        let mut speeds = robot.potential_field(ball.point(), obstacle);

        println!("Speed antes: {:?}", speeds);
        if speeds.0 < 0.0 {
            let a = speeds.0 * -1.0;

            speeds = (132.0 + a, speeds.1);
        } else {
            speeds = (4.0 + speeds.0, speeds.1);
        }

        if speeds.1 < 0.0 {
            let a = speeds.1 * -1.0;

            speeds = (speeds.0, 132.0 + a);
        } else {
            speeds = (speeds.0, 4.0 + speeds.1);
        }

        println!("Speed: {:?}", speeds);

        let command: [u8; 3] = [1, speeds.0 as u8, speeds.1 as u8]; 

        println!("{:?}", command);

        port.write(&command).expect("Write failed!");
        thread::sleep(time::Duration::from_millis(10));
    }
}
