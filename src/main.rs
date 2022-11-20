use fbot_rust_client::{FIRASIM, REFEREE, fira_protos, UVF};
use fbot_vss_rust::{Robot, Team, Ball, Point};

fn main() {
    let a = UVF::new(Point::new(0.0, 0.0), vec![]);

    a.plot_vector_field();

    // let mut commands: [Option<fira_protos::Command>;3] = [None, None, None];

    // let ball = Ball::new();
    // let goalie = Robot::new(0, Team::Yellow);
    // // let neymar = Robot::new(1, Team::Yellow);
    // // let robot3 = Robot::new(2, Team::Yellow);

    // loop {
    //     println!("Foul: {:?}", REFEREE.foul());

    //     // if REFEREE.foul() == ref_protos::Foul::FreeBall {
    //         commands[goalie.id() as usize] = Some(goalie.go_to2(Point::new(0.6, ball.y())));
    //         // commands[neymar.id() as usize] = Some(neymar.go_to2(Point::new(0.7, ball.y())));
    //         // commands[robot3.id() as usize] = Some(robot3.go_to2(Point::new(0.1, ball.y())));

    //         let robot_commands: Vec<fira_protos::Command> = commands.iter().filter_map(|x| x.to_owned()).collect();

    //         FIRASIM.send_command(fira_protos::Commands {
    //             robot_commands
    //         });      
    //     // }
    // }
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