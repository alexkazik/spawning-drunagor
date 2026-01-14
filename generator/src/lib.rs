use crate::monsters::monsters;
use crate::setup::setup;
use anyhow::Context;
use std::ffi::OsStr;
use std::fs::write;
use std::path::Path;

mod game;
mod monsters;
mod setup;

pub fn run(out_dir: &OsStr) -> Result<(), anyhow::Error> {
    let mut output = String::new();

    let monsters = monsters(&mut output)?;
    setup(&mut output, &monsters)?;

    write(Path::new(out_dir).join("generated.rs"), &output)
        .context("Failed to write generated data")?;

    Ok(())
}
