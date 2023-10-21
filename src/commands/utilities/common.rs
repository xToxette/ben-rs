use poise::serenity_prelude::Timestamp;
use crate::{Context, Error};

/// Informs you about the meaning of ben's existence
#[poise::command(prefix_command, slash_command)]
pub async fn information(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let msg = ctx.channel_id().send_message(&ctx.http(), |m| {
        m.content("Ben is currently being development")
            .embed(|e| {
                e.title("Development")
                    .description("Please contribute, otherwise I'm so lonely: https://github.com/xToxette/ben-rs")
                    .timestamp(Timestamp::now())
            })
    }).await;
    Ok(())
}
