use crate::{Context, Error, Data, dal};
use poise::{Event, serenity_prelude as serenity};

pub async fn main_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data
) -> Result<(), Error> {
    match event {
        Event::VoiceStateUpdate { old, new } => {
            dal::events::voice_state_update::insert(&data.conn_pool, &new).await?;
        }
        Event::Message { new_message } => {
            println!("Message: {:#?}", new_message);
        }
        _ => {}
    }
    Ok(())
}
