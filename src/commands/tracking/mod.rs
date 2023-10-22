mod statistics;

use crate::{Context, Error};
use statistics::stats;

#[poise::command(slash_command, ephemeral, subcommands("stats"))]
pub async fn tracking(
    _ctx: Context<'_>,
) -> Result<(), Error> {
    Ok(())
}
