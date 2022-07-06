extern crate prost_build;

use std::{result::Result, io::Error};

fn main() -> Result<(), Error> {
    prost_build::compile_protos(
        &["src/protos/common.proto", "src/protos/command.proto", "src/protos/packet.proto", "src/protos/replacement.proto"], 
        &["src/protos"]
    )?;

    Ok(())
}