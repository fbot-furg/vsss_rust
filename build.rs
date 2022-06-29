extern crate prost_build;

use std::{result::Result, io::Error};

fn main() -> Result<(), Error> {
    prost_build::compile_protos(&["src/common.proto", "src/command.proto", "src/packet.proto", "src/replacement.proto"], &["src"])?;

    Ok(())
}