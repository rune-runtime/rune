use std::{env, fs::{self, File, OpenOptions, Permissions}, io::{self, BufWriter, ErrorKind, Read, Write}, os::unix::fs::{OpenOptionsExt, PermissionsExt}, path::{Path, PathBuf}, process::{Command, Stdio}};

use color_eyre::eyre::Result;
use crossterm::event::KeyEvent;

use rune::input;
use ratatui::prelude::Rect;
use semver::Version;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use toml::Table;
use current_platform::CURRENT_PLATFORM;

use crate::{
    action::Action, cli::{Cli, CliCommand}, components::Component, config::Config, mode::Mode, settings::Settings, tui
};

pub struct App {
    pub config: Config,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub should_quit: bool,
    pub should_suspend: bool,
    pub mode: Mode,
    pub command: Option<CliCommand>,
    pub last_tick_key_events: Vec<KeyEvent>,
}

impl App {
    pub fn new(cli: Cli) -> Result<Self> {
        let config = Config::new()?;
        let mode = Mode::Home;
        let command = cli.command;

        Ok(Self {
            tick_rate: 4.0,
            frame_rate: 60.0,
            should_quit: false,
            should_suspend: false,
            config,
            command,
            mode,
            last_tick_key_events: Vec::new(),
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        match &self.command {
            Some(CliCommand::New(new)) => crate::commands::new::new(new).await?,
            Some(CliCommand::Test) => {
                let input_path = Path::new("../test-game/dist/.rune/input/");
                let binary = std::fs::read(input_path.join("test-game.wasm")).unwrap();
                rune::runtime::test(input_path.to_path_buf(), binary).await;
            },
            Some(CliCommand::Run { release }) => {
                crate::commands::build::build(release).await?;

                let current_dir = env::current_dir()?;
                let config = std::fs::read_to_string(current_dir.join("rune.toml")).unwrap().parse::<Table>().unwrap();

                let entrypoint_path = match config["build"]["entrypoint"].as_str() {
                    Some(entrypoint_path) => entrypoint_path,
                    None => panic!("No build input provided in config!")
                };

                match config["build"]["output"].as_str() {
                    Some(output_path) => {
                        let output_path = current_dir.join(output_path);
                        let entrypoint_path = output_path.join(entrypoint_path);
                        let binary = std::fs::read(entrypoint_path).unwrap();
                        rune::runtime::run(output_path.to_path_buf(), binary);
                    },
                    None => panic!("No build input provided in config!")
                }
            },
            Some(CliCommand::Build { release }) => {
                crate::commands::build::build(release).await?;
            },
            Some(CliCommand::Bundle { target, release }) => {
                crate::commands::build::build(release).await?;
                crate::commands::bundle::bundle(target, release).await?;
            },
            Some(CliCommand::Upgrade) => crate::commands::upgrade::upgrade().await?,
            Some(CliCommand::Docs) => crate::commands::docs::docs(&self.config, &self.mode).await?,
            None => {}
        }

        Ok(())
    }
}
