extern crate prost_build;

use std::{result::Result, io::Error};

fn main() -> Result<(), Error> {
    prost_build::compile_protos(&["src/grSim_Commands.proto", "src/grSim_.proto", "src/grSim_Commands.proto"], &["src"])?;

    Ok(())
}
