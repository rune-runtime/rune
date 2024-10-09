pub mod macos;
pub mod windows;

use std::{env, fs::{self, File, OpenOptions}, io::{self, BufWriter, Write}, os::unix::fs::OpenOptionsExt, path::PathBuf, process::{Command, Stdio}};

use current_platform::CURRENT_PLATFORM;
use semver::Version;
use toml::Table;

use crate::Result;
use crate::settings::Settings;

pub async fn bundle(target: &String, release: &bool) -> Result<()> {
    let config = std::fs::read_to_string("rune.toml").unwrap().parse::<Table>().unwrap();

    let current_dir = env::current_dir()?;
    let rune_dir = current_dir.join(".rune");
    let rune_bin_dir = rune_dir.join("bin");

    let build = if *release { "release" } else { "debug" }.to_owned();
    let target = target.to_owned();
    let target_triplet = match target.as_str() {
        "android" => "aarch64-linux-android",
        "ios" => "aarch64-apple-ios",
        "linux" => "x86_64-unknown-linux-musl",
        "macos" => "aarch64-apple-darwin",
        "windows" => "x86_64-pc-windows-msvc",
        _ => panic!("no --target provided")
    }.to_owned();

    let settings = Settings {
        current_dir: current_dir.clone(),
        rune_dir,
        rune_bin_dir,
        metadata_id: config["metadata"]["id"].as_str().unwrap().to_owned(),
        metadata_version: Version::parse(config["metadata"]["version"].as_str().unwrap()).unwrap(),
        build,
        target,
        target_triplet,
        runtime_version: Version::parse(config["runtime"]["version"].as_str().unwrap()).unwrap(),
        build_input_dir: current_dir.clone().join(config["build"]["input"].as_str().unwrap()),
        build_output_dir: current_dir.clone().join(config["build"]["output"].as_str().unwrap()),
        build_entrypoint: PathBuf::from(config["build"]["entrypoint"].as_str().unwrap()),
        bundle_name: config["bundle"]["name"].as_str().unwrap().to_owned(),
        bundle_identifier: config["bundle"]["identifier"].as_str().unwrap().to_owned()
    };

    println!("Building for target {}", settings.target_triplet);

    // TODO: Create a rust project that imports the wasm binary and runs it in the rune runtime
    // println!("Creating project...");
    init_rust_project(&settings).await?;
    
    // println!("Installing Rust...");
    install_rustup(&settings).await?;

    // println!("Installing Cross...");
    install_cross(&settings).await?;

    // TODO: Ensure docker is available (no portable installation possible)

    build_target(&settings).await?;

    copy_input_to_output(&settings).await?;

    // TODO: Copy source code from cargo bundle to build appropriate package for target (cargo bundle does not support using existing binaries built by cross)
    // ie. https://github.com/burtonageo/cargo-bundle/blob/master/src/bundle/ios_bundle.rs#L22

    match settings.target.as_str() {
        "android" => {},
        "ios" => {},
        "linux" => {},
        "macos" => macos::bundle_project(&settings)?,
        "windows" => windows::bundle_project(&settings)?,
        _ => {}
    }

    Ok(())
}


async fn init_rust_project(settings: &Settings) -> Result<()> {
    let metadata_id = &settings.metadata_id;
    let runtime_version = settings.runtime_version.to_string();

    let project_dir = settings.rune_dir.join("project");
    let src_dir = project_dir.join("src");
    if fs::metadata(&project_dir).is_ok() {
        return Ok(());
    }
    fs::create_dir_all(&project_dir)?;

    let cargotoml_path = project_dir.join("Cargo.toml");
    let mut cargo_toml = File::create(&cargotoml_path)?;
    cargo_toml.write_all(format!(r#"
    [package]
    name = "{}"
    version = "0.1.0"
    edition = "2021"
    publish = false

    [dependencies]
    rune = {{ path = "../../../rune", version = "{runtime_version}" }}

    [[bin]]
    name = "{}"
    path = "src/main.rs"
    "#, metadata_id, metadata_id).as_bytes())?;

    fs::create_dir_all(&project_dir.join("src"))?;

    let entrypoint_path_str = settings.build_entrypoint.to_str().unwrap();
    let main_path = src_dir.join("main.rs");
    let mut main = File::create(&main_path)?;
    main.write_all(format!(r#"
    use std::env;
    use std::fs;
    use rune::runtime;

    fn main() {{
        let input_path = env::current_exe().unwrap().parent().unwrap().join(".rune/input/");
        let binary = fs::read(input_path.join("{entrypoint_path_str}")).expect("Failed to read the WASM file");
        runtime::run(input_path, binary);
    }}
    "#).as_bytes())?;
    
    Ok(())
}

async fn install_rustup(settings: &Settings) -> Result<()> {
    let rustup_dir = settings.rune_bin_dir.join("rustup");
    if fs::metadata(&rustup_dir).is_ok() {
        return Ok(());
    }

    let os = env::consts::OS;
    let filename = match os {
        "windows" => "rustup-init.exe",
        _ => "rustup-init",
    };
    let url = format!(
        "https://static.rust-lang.org/rustup/dist/{}/{}",
        CURRENT_PLATFORM, filename
    );
    let resp = reqwest::get(url).await?;
    let content = resp.bytes().await?;

    let out_path = rustup_dir.join(filename);
    println!("{}", out_path.to_str().unwrap());
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .mode(0o755) // set the permissions at creation
        .open(&out_path)?;

    let mut out = BufWriter::new(file);
    out.write_all(&content.as_ref())?;

    let mut cmd = Command::new(out_path);

    cmd.env("CARGO_HOME", settings.rune_bin_dir.join("cargo"))
        .env("RUSTUP_HOME", settings.rune_bin_dir.join("rustup"));

    cmd.arg("-y");

    let output = cmd
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()?;

    println!("Status: {}", output.status);
    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    println!("Error: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}

async fn install_cross(settings: &Settings) -> Result<()> {
    let cross_path = settings.rune_bin_dir.join("cargo/bin/cross");
    if fs::metadata(cross_path).is_ok() {
        return Ok(());
    }

    let cargo_path = settings.rune_bin_dir.join("cargo/bin/cargo");

    let mut cmd = Command::new(cargo_path.clone());

    cmd.env("CARGO_HOME", settings.rune_bin_dir.join("cargo"))
        .env("RUSTUP_HOME", settings.rune_bin_dir.join("rustup"));

    cmd.args(["install", "cross"]);

    let output = cmd
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()?;

    println!("Output: {}", String::from_utf8_lossy(&output.stdout));
    println!("Error: {}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}

async fn build_target(settings: &Settings) -> Result<()> {
    let rust_project_path = settings.rune_dir.join("project/.");

    let cargo_bin_path = settings.rune_bin_dir.join("cargo/bin");

    let current_path = env::var("PATH").unwrap_or_else(|_| String::from(""));
    let new_path = format!("{}:{}", cargo_bin_path.to_str().unwrap(), current_path);
    env::set_var("PATH", new_path);

    let mut cmd = Command::new("cross");
    cmd.current_dir(rust_project_path);

    cmd.env("CARGO_HOME", settings.rune_bin_dir.join("cargo"))
        .env("RUSTUP_HOME", settings.rune_bin_dir.join("rustup"));

    cmd.args(["build", "--target", &settings.target_triplet]);

    let output = cmd
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .output()?;

    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("{}", String::from_utf8_lossy(&output.stderr));

    Ok(())
}

async fn copy_input_to_output(settings: &Settings) -> Result<()> {
    crate::fs::copy_dir_all(
        &settings.build_input_dir,
        settings.build_output_dir.join(".rune/input")
    )?;
    Ok(())
}
