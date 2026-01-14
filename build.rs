use anyhow::anyhow;
use generator::run;
use std::env;

fn main() -> Result<(), anyhow::Error> {
    let out_dir = env::var_os("OUT_DIR").ok_or_else(|| anyhow!("Couldn't get OUT_DIR"))?;

    run(&out_dir)
}
