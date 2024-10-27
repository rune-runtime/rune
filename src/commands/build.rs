use color_eyre::eyre;
use rust_embed::Embed;
use subprocess::{Exec, ExitStatus, Redirection};
use wasmparser::{Encoding, Payload};
use std::env;
use std::path::{Path, PathBuf};

use toml::Table;

use wit_component::{
    ComponentEncoder, DecodedWasm, Linker, StringEncoding, WitPrinter,
};

use crate::cli::NewSubcommand;

use crate::Result;

#[derive(Embed)]
#[folder = "wasi"]
struct WasiWasm;

pub async fn build(release: &bool) -> Result<()> {
    let current_dir = env::current_dir()?;
    let config = std::fs::read_to_string("rune.toml")
        .unwrap()
        .parse::<Table>()
        .unwrap();

    let build = config.get("build").unwrap();

    let pre_command = build.get("pre");
    if let Some(command) = pre_command {
        let result = Exec::shell(command.as_str().unwrap())
            .stdout(Redirection::Pipe) 
            .stderr(Redirection::Merge)
            .capture()
            .expect("pre command execution failed");

        let stdout = result.stdout_str();
        println!("{}", stdout);

        if !result.success() {
            return Err(eyre::eyre!("pre command execution failed"));
        }
    }

    let entrypoint = match config["build"]["entrypoint"].as_str() {
        Some(entrypoint) => entrypoint,
        None => panic!("No build input provided in config!"),
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
    
    componentize_wasm(output_entrypoint_path);

    Ok(())
}

fn componentize_wasm(output_entrypoint_path: PathBuf) {
    let parser = wat::Parser::new();
    let wasm = parser.parse_file(&output_entrypoint_path).expect("Unable to read game wasm");
    let mut encoder = ComponentEncoder::default()
        .validate(true)
        .reject_legacy_names(false);

    let bytes: Vec<u8>;
    let mut is_component = false;
    for payload in wasmparser::Parser::new(0).parse_all(&wasm) {
        let payload = payload.expect("No wasm payload");
        match payload {
            wasmparser::Payload::Version { encoding, .. } if encoding != Encoding::Module => {
                is_component = true;
            }
            _ => { }
        }
    }

    if is_component {
        bytes = wasm;
    } else {
        // encoder = encoder.merge_imports_based_on_semver(merge); // TODO: Needed?
        encoder = encoder.module(&wasm).expect("Unable to read game as a wasm module");

        let adapter = WasiWasm::get("wasi_snapshot_preview1.reactor.wasm").unwrap();
        let adapter = wat::parse_bytes(&adapter.data).unwrap();
        encoder = encoder.adapter("wasi_snapshot_preview1", &adapter).expect("Unable to read adapter");

        bytes = encoder
            .encode()
            .expect("Failed to encode a component from provided module");
    }

    std::fs::write(&output_entrypoint_path, bytes).expect("Unable to write wasm");
}
