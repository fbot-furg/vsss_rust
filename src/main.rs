use fbot_rust_client::{FIRASIM, REFEREE, fira_protos};
use fbot_vss_rust::{Origin, Robot, Team, Ball, Point, UVF, Goal, Obstacle};

fn main() {
    let ball = Ball::new(Origin::FIRASIM);
    let robot = Robot::new(Origin::FIRASIM, 0, Team::Yellow);

    let mut commands: [Option<fira_protos::Command>;1] = [None];
    
    loop {
        let uvf = UVF::new(ball.point(), vec![]);

        let force = uvf.calculate_force(&robot.point());
        let target_point = Point::new(robot.point().x() - force.x(), robot.point().y() - force.y());

        let speed = robot.go_to(target_point);

        let cmd =  fira_protos::Command {
            id: 0,
            yellowteam: true,
            wheel_left: speed.0,
            wheel_right: speed.1,
        };
        
        commands[robot.id() as usize] = Some(cmd);

        let robot_commands: Vec<fira_protos::Command> = commands.iter().filter_map(|x| x.to_owned()).collect();

        FIRASIM.send_command(fira_protos::Commands {
            robot_commands
        }); 
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