use fbot_rust_client::{FIRASIM, fira_protos};
use fbot_vss_rust::{Robot, Team, Ball};

fn main() {
    let mut commands: [Option<fira_protos::Command>;3] = [None, None, None];

    let ball = Ball::new();
    let goalie = Robot::new(0, Team::Yellow);

    loop {
        let goalie_cmd = Some(goalie.potential_field(ball.point()));
        
        commands[goalie.id() as usize] = goalie_cmd;
        let robot_commands: Vec<fira_protos::Command> = commands.iter().filter_map(|x| x.to_owned()).collect();

        FIRASIM.send_command(fira_protos::Commands {
            robot_commands
        }); 
    }
}
