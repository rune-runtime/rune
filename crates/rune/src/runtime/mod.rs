use std::path::PathBuf;

use anyhow::Result;
use cpal::traits::HostTrait;
use libtest_mimic::{Arguments, Trial};
use pollster;

mod common;
#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(any(target_os = "macos", target_os = "windows"))]
mod native;

pub use common::*;
#[cfg(target_arch = "wasm32")]
pub use web::*;
#[cfg(any(target_os = "macos", target_os = "windows"))]
pub use native::{
    run::run,
    run::test,
    state::RuneRuntimeState
};
