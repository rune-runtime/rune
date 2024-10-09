extern crate rune;

use rune::runtime;
use std::process::Command;

#[cfg(test)]
fn main() {
    use std::{env, path::Path};

    let current_dir = env::current_dir().unwrap();

    let runtime_tests_dir = current_dir.join("tests/runtime-tests/");

    let output = Command::new("cargo")
        .arg("build")
        .current_dir(&runtime_tests_dir)
        .output()
        .expect("Failed to execute cargo build");

    if !output.status.success() {
        panic!("cargo build failed!");
    }

    // let output = Command::new("rune")
    let output = Command::new(current_dir.join("../rune-cli/target/debug/rune-cli"))
        .arg("build")
        .current_dir(&runtime_tests_dir)
        .output()
        .expect("Failed to execute Rune build");

    if !output.status.success() {
        println!("{:?}", output.stderr);
        panic!("Rune build failed!");
    }

    let input_path = runtime_tests_dir.join("bin");
    let binary = std::fs::read(input_path.join("runtime_tests.wasm")).unwrap();
    pollster::block_on(rune::runtime::test(input_path.to_path_buf(), binary));
}
