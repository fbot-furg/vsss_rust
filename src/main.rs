use vsss_rust_client::{FIRASIM};
use fbot_vss_rust::{Origin, Robot, Ball, UVF, Obstacle, Vector, TEAM, Command, Team};

fn main() {
    let team = *TEAM;
    let ball = Ball::new(Origin::FIRASIM);
    let mut robot = Robot::new(Origin::FIRASIM, 0);

    robot.set_robot_speed(40.0)
        .set_orientation_kp(30.0);

    let mut goalie = Robot::new(Origin::FIRASIM, 1);

    goalie.set_robot_speed(20.0)
        .set_orientation_kp(5.0);
        

    //debug
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // let enemy_team = Obstacle::enemy_team_fira();

    // std::thread::sleep(std::time::Duration::from_secs(1));

    // let enemy_team = Obstacle::enemy_team_fira();

    // let mut uvf = UVF::new(ball.point(), enemy_team);

    // uvf.set_de(10.0)
    //     .set_kr(10.0)
    //     .set_d_min(15.0);

    // uvf.plot_vector_field();

    loop {
        let enemy_team = Obstacle::enemy_team_fira();

        let mut uvf = UVF::new(ball.point(), enemy_team);

        uvf.set_de(10.0)
            .set_kr(10.0)
            .set_d_min(10.0);

        let force = uvf.calculate_force(&robot.point());
        let target_point = Vector::new(robot.point().x() - force.x(), robot.point().y() - force.y());

        let speed = robot.go_to(target_point);

        let a = match *TEAM {
            Team::Yellow => 65.0,
            Team::Blue => -65.0
        };
        let goalie_target_point = Vector::new(a, ball.y());

        let goalie_speed = goalie.go_to2(goalie_target_point);
        
        let command = Command::new(0, team.to_bool(), speed.0, speed.1);
        let goalie_command = Command::new(1, team.to_bool(), goalie_speed.0, goalie_speed.1);

        FIRASIM.send_command(vec![goalie_command, command]); 
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