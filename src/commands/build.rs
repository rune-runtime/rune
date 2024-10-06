use std::env;
use std::path::Path;
use std::process::Command;

use toml::Table;

use crate::cli::NewSubcommand;

use crate::Result;

pub async fn build(release: &bool) -> Result<()> {
    let current_dir = env::current_dir()?;
    let config = std::fs::read_to_string("rune.toml").unwrap().parse::<Table>().unwrap();

    let entrypoint = match config["build"]["entrypoint"].as_str() {
        Some(entrypoint) => entrypoint,
        None => panic!("No build input provided in config!")
    };

    let input_path = config["build"]["input"].as_str();
    if input_path.is_none() {
        panic!("No build input provided in config!")
    }
    let input_path = Path::new(input_path.unwrap());
    let entrypoint_path = input_path.join(entrypoint);
    let binary = std::fs::read(&entrypoint_path).unwrap();

    let output_path = Path::new(config["build"]["output"].as_str().unwrap_or("bin"));

    crate::fs::copy_dir_all(input_path, output_path)?;

    let output_entrypoint_path = current_dir.join(&output_path).join(&entrypoint);

    // TODO: Concatenate rune dependencies read from config to wasm binary
    
    // TODO: Use wasm-tools library instead of CLI
    let output = Command::new("wasm-tools")
        .args(["component", "new", output_entrypoint_path.to_str().unwrap(), "-o", output_entrypoint_path.to_str().unwrap(), "--adapt", "./wasi_snapshot_preview1.reactor.wasm"])
        .current_dir(current_dir)
        .output()
        .expect("Failed to execute wasm-tools");

    Ok(())
}
