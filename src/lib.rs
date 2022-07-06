use std::io::Cursor;
use prost::Message;

use std::net::UdpSocket;

const VISION_ADDRS: &str = "224.0.0.1:10002";
const COMMAND_ADDRS: &str = "127.0.0.1:10011";


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

pub fn get_ball() -> Option<fira_protos::Ball> {
    let socket = UdpSocket::bind(VISION_ADDRS).unwrap();
    let mut buf = [0; 1024];

    let (len, addr) = socket.recv_from(&mut buf).unwrap();
    

    let env = deserialize_env(&buf[..len]).unwrap();

    if let Some(frame) = env.frame {
        frame.ball   
    } else {
        None
    }
}
