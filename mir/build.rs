use std::env;
use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=src/cli.yaml");
    let out_dir = env::var("OUT_DIR")?;
    fs::copy("src/cli.yaml", out_dir + "/cli.yaml")?;
    Ok(())
}
