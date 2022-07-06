use std::io::Cursor;
use prost::Message;


// Include the `items` module, which is generated from items.proto.
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