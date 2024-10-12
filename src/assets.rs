use rust_embed::Embed;

#[derive(Embed)]
#[folder = "crates/rune/wit"]
pub struct RuneWits;

#[derive(Embed)]
#[folder = "crates/rune/wit/runtime"]
pub struct RuneRuntimeWits;

#[derive(Embed)]
#[folder = "src/templates"]
pub struct Templates;
