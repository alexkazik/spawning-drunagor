use crate::monster::monster;
use crate::setup::setup;
use anyhow::Context;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

mod game;
mod monster;
mod setup;

pub fn run(out_dir: &OsStr) -> Result<(), anyhow::Error> {
    let (generated_monster, monsters) = monster().context("function monster")?;
    let generated_setup = setup(&monsters).context("function setup")?;

    fs::write(
        Path::new(out_dir).join("generated_monster.rs"),
        &generated_monster,
    )
    .context("Failed to write generated_monster")?;

    fs::write(
        Path::new(out_dir).join("generated_setup.rs"),
        &generated_setup,
    )
    .context("Failed to write generated_setup")?;

    Ok(())
}
