use fbot_rust_client::fira_protos;
use fbot_vss_rust::{FIRASIM, Robot, Team, Ball, Point};

fn main() {
    let mut commands: [Option<fira_protos::Command>;3] = [None, None, None];


    let ball = Ball::new();

    let goalie = Robot::new(0, Team::Yellow);
    let neymar = Robot::new(1, Team::Yellow);
    let robot3 = Robot::new(2, Team::Yellow);

    loop {
        commands[neymar.id() as usize] = Some(neymar.go_to2(Point::new(0.7, ball.y())));
        commands[goalie.id() as usize] = Some(goalie.go_to2(Point::new(0.4, ball.y())));
        commands[robot3.id() as usize] = Some(robot3.go_to2(Point::new(0.1, ball.y())));

        let robot_commands: Vec<fira_protos::Command> = commands.iter().filter_map(|x| x.to_owned()).collect();

        FIRASIM.send_command(fira_protos::Commands {
            robot_commands
        });   
    }
}
