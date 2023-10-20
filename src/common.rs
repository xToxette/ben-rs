use poise::serenity_prelude::Timestamp;
use crate::{Context, Error};

#[poise::command(prefix_command, slash_command)]
pub async fn information(
    ctx: Context<'_>,
    #[description = "Informs you about the meaning of ben's existence"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    let msg = ctx.channel_id().send_message(&ctx.http(), |m| {
        m.content("Ben is currently being development")
            .embed(|e| {
                e.title("Development")
                    .timestamp(Timestamp::now())
            })
    }).await;
    Ok(())
}
