use fbot_vss_rust::fira_rust::{fira_protos, get_ball, get_blue_robot, send_command};

fn main() {
    loop {
        // Goalie Logic
        let ball = get_ball();

        // TODO Garantir retorno do goalie
        if let Some(goalie)  = get_blue_robot(0) {
            println!("{}", goalie.orientation)
            // let err = ball.y - goalie.y;

            // let velocidade = err * 1000.0;

            // let commands = fira_protos::Commands {
            //     robot_commands: vec![
            //         fira_protos::Command {
            //             id: 3,
            //             yellowteam: false,
            //             wheel_left: velocidade,
            //             wheel_right: velocidade
            //         },
            //     ]
            // };
        
            // send_command(commands);
        };
    }
}
