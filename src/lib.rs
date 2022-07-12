pub mod fira_rust {
    use std::io::Cursor;
    use prost::Message;
    use std::net::UdpSocket;

    const VISION_ADDRS: &str = "224.0.0.1:10002";
    const COMMAND_ADDRS: &str = "127.0.0.1:20011";

    pub mod fira_protos {
        include!(concat!(env!("OUT_DIR"), "/fira_message.sim_to_ref.rs"));
    }    

    pub fn serialize_packet(packet: &fira_protos::Packet) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.reserve(packet.encoded_len());
        
        // Unwrap is safe, since we have reserved sufficient capacity in the vector.
        packet.encode(&mut buf).unwrap();
        buf
    }

    pub fn deserialize_env(buf: &[u8]) -> Result<fira_protos::Environment, prost::DecodeError> {
        fira_protos::Environment::decode(&mut Cursor::new(buf))
    }

    pub fn send_command(commands: fira_protos::Commands) {
        let socket_sender = UdpSocket::bind(VISION_ADDRS).unwrap();

        let packet = fira_protos::Packet {
            cmd: Some(commands),
            replace: None        
        };
        let buf = serialize_packet(&packet); 

        match socket_sender.send_to(&buf, COMMAND_ADDRS) {
            Ok(_) => {},
            Err(e) => {
                println!("Error Send {}", e)
            }
        };
    }

    fn get_frame() -> Option<fira_protos::Frame>{
        let socket = UdpSocket::bind(VISION_ADDRS).unwrap();
        let mut buf = [0; 1024];

        let (len, addr) = socket.recv_from(&mut buf).unwrap();
        
        let env = deserialize_env(&buf[..len]).unwrap();

        env.frame
    }

    pub fn get_ball() -> fira_protos::Ball {
        let mut ret = fira_protos::Ball{
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
        };

        if let Some(frame) = get_frame() {
            if let Some(ball) = frame.ball {
                ret = ball
            }
        }

        ret
    }

    pub fn get_blue_robot(id: u32) -> Option<fira_protos::Robot> {
        if let Some(frame) = get_frame() {
            let mut ret = None;

            for robot in frame.robots_blue {
                if robot.robot_id == id {
                    ret = Some(robot)
                }
            };

            ret

        } else { None }
    }

    pub fn get_yellow_robot(id: u32) -> Option<fira_protos::Robot> {
        if let Some(frame) = get_frame() {
            let mut ret = None;

            for robot in frame.robots_yellow {
                if robot.robot_id == id {
                    ret = Some(robot)
                }
            };

            ret

        } else { None }
    }

    pub fn got_to(target_x: f64, target_y: f64) {
        // O comando está apenas funcionando com o robo do time amarelo id 0

        
    }

}

