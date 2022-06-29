use std::io::Cursor;

use prost::Message;

// Include the `items` module, which is generated from items.proto.
pub mod fira {
    include!(concat!(env!("OUT_DIR"), "/fira_message.sim_to_ref.rs"));
}

pub fn serialize(shirt: &fira::Environment) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(shirt.encoded_len());
    // Unwrap is safe, since we have reserved sufficient capacity in the vector.
    shirt.encode(&mut buf).unwrap();
    buf
}

pub fn deserialize(buf: &[u8]) -> Result<fira::Environment, prost::DecodeError> {
    fira::Environment::decode(&mut Cursor::new(buf))
}