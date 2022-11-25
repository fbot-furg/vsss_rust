use vsss_rust_client::{FIRASIM};
use fbot_vss_rust::{Origin, Robot, Team, Ball, Vector, UVF, Command};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long, default_value = "y")]
   team: String
}


fn main() {
    let args = Args::parse();
    
    //Get team from args, default is yellow
    let team = Team::from_str(&args.team);

    let ball = Ball::new(Origin::FIRASIM);
    let robot = Robot::new(Origin::FIRASIM, 0, team);
    
    loop {
        let uvf = UVF::new(ball.point(), vec![], team);

        let force = uvf.calculate_force(&robot.point());
        let target_point = Vector::new(robot.point().x() - force.x(), robot.point().y() - force.y());

        let speed = robot.go_to(target_point);

        let command = Command::new(0, team.to_bool(), speed.0, speed.1);

        FIRASIM.send_command(vec![command]); 
    }
}


// let mut port = serialport::new("/dev/ttyUSB0", 9600)
//     .timeout(time::Duration::from_millis(50))
//     .open().expect("Failed to open port");


// let ball = Ball::new(Origin::SSLVISION);
// let robot = Robot::new(Origin::SSLVISION, 0, Team::Yellow);
// let robot_blue = Robot::new(Origin::SSLVISION, 0, Team::Blue);
// loop {

//     let obstacle = Obstacle::new(robot_blue.point(),  10.0);

//     let mut speeds = robot.potential_field(ball.point(), obstacle);

//     println!("Speed antes: {:?}", speeds);
//     if speeds.0 < 0.0 {
//         let a = speeds.0 * -1.0;

//         speeds = (132.0 + a, speeds.1);
//     } else {
//         speeds = (4.0 + speeds.0, speeds.1);
//     }

//     if speeds.1 < 0.0 {
//         let a = speeds.1 * -1.0;

//         speeds = (speeds.0, 132.0 + a);
//     } else {
//         speeds = (speeds.0, 4.0 + speeds.1);
//     }

//     println!("Speed: {:?}", speeds);

//     let command: [u8; 3] = [1, speeds.0 as u8, speeds.1 as u8]; 

//     println!("{:?}", command);

//     port.write(&command).expect("Write failed!");
//     thread::sleep(time::Duration::from_millis(10));
// }