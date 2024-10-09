use crate::cli::NewSubcommand;

use crate::Result;

pub mod game;

pub async fn new(new: &NewSubcommand) -> Result<()> {
    match new {
        crate::cli::NewSubcommand::Game {
            identifier,
            name,
            template,
        } => game::game(identifier, name, template).await?,
    }

    Ok(())
}
