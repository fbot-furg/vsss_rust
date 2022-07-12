use fbot_vss_rust::fira_rust::{fira_protos, get_ball, get_yellow_robot, send_command};

fn main() {
    loop {
        // Goalie Logic
        let ball = get_ball();

        // TODO Garantir retorno do goalie
        if let Some(goalie)  = get_yellow_robot(0) {
            let err = ball.y - goalie.y;

            let velocidade = err * 1000.0;

            let commands = fira_protos::Commands {
                robot_commands: vec![
                    fira_protos::Command {
                        id: 0,
                        yellowteam: true,
                        wheel_left: velocidade,
                        wheel_right: velocidade
                    },
                ]
            };
        
            send_command(commands);
        };
    }
}
