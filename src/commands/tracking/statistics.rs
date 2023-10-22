use poise::{serenity_prelude as serenity};
use crate::{Context, dal, Error};

/// Display various statistics about your presence in this server
#[poise::command(slash_command, ephemeral)]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "A user to get statistics for"]
    user: Option<serenity::Member>,
) -> Result<(), Error> {
    ctx.send(|m| {
        match user {
            Some(user) =>
                m.content(format!("Lets see {}'s statistics", user.display_name())),
            None =>
                m.content("Lets see your statistics")
        }
    }).await?;
    println!(
        "Requested data: {:#?}",
        dal::events::voice_state_update::get_voice_state_updates(&ctx.data().conn_pool, ctx.author().id.0 as i64, ctx.guild_id().map(|i| i.0 as i64)).await.unwrap()
    );
    Ok(())
}