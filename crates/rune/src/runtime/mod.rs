use std::path::PathBuf;

use anyhow::Result;
use cpal::traits::HostTrait;
use libtest_mimic::{Arguments, Trial};
use pollster;

mod common;
#[cfg(target_arch = "wasm32")]
mod web;
#[cfg(not(target_arch = "wasm32"))]
mod native;

pub use common::*;
#[cfg(target_arch = "wasm32")]
pub use web::{
    run::run,
    run::test,
    state::RuneRuntimeState
};
#[cfg(not(target_arch = "wasm32"))]
pub use native::{
    run::run,
    run::test,
    state::RuneRuntimeState
};
