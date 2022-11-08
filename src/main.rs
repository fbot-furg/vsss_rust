use fbot_rust_client::{FIRASIM, REFEREE, fira_protos};
use fbot_vss_rust::{Robot, Team, Ball, Point, Origin};
use std::{thread, time};

fn main() {
    let mut port = serialport::new("/dev/ttyUSB0", 9600)
        .timeout(time::Duration::from_millis(50))
        .open().expect("Failed to open port");

    let ball = Ball::new(Origin::SSLVISION);
    let goalie = Robot::new(Origin::SSLVISION, 0, Team::Blue);

    loop {
        // println!("{}", goalie.x());
        let speeds = goalie.go_to2(ball.point());


        let speeds: (u8, u8) = ((speeds.0 + 127.0) as u8, (speeds.1 + 127.0) as u8);

        println!("Speed: {:?}", speeds);

        let command: [u8; 5] = [5, 0, speeds.0, speeds.1, 15]; 

        println!("Robot: {:?}", goalie.point());
        println!("Ball: {:?}", ball.point());
        println!("Command: {:?}", command);

        port.write(&command).expect("Write failed!");
        thread::sleep(time::Duration::from_millis(5));
    }
}
