use poise::{CreateReply, serenity_prelude as serenity};
use crate::{Context, dal, Error};

/// Display various statistics about your presence in this server
#[poise::command(slash_command, ephemeral)]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "A user to get statistics for"]
    user: Option<serenity::User>,
) -> Result<(), Error> {
    let target_user = user.clone().unwrap_or(ctx.author().clone());
    let voice_state_updates = dal::events::voice_state_update::get_voice_state_updates(
        &ctx.data().conn_pool,
        target_user.id.0 as i64,
        ctx.guild_id()
            .map(|i| i.0 as i64))
        .await?;

    if voice_state_updates.is_empty() {
        ctx.send(|m| m.content(format!("Not statistics were found for the requested user"))).await?;
        return Ok(())
    }



    ctx.send(|m| {
        match user {
            Some(user) =>
                m.content(format!("Lets see {}'s statistics", target_user.name)),
            None =>
                m.content("Lets see your statistics"),
        }.embed(|m| {
            m.title("Statistics")
        })
    }).await?;

    Ok(())
}