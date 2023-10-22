use crate::{Error, Data, dal};
use poise::{BoxFuture, Event, serenity_prelude as serenity};

pub fn main_handler<'a>(
    _ctx: &'a serenity::Context,
     event: &'a Event<'a>,
    _framework: poise::FrameworkContext<'a, Data, Error>,
     data: &'a Data
) -> BoxFuture<'a, Result<(), Error>> {
    Box::pin(async move {
        match event {
            Event::VoiceStateUpdate { old: _, new } => {
                dal::events::voice_state_update::insert(&data.conn_pool, &new).await?;
            }
            Event::Message { new_message : _} => {
            }
            _ => {}
        }
        Ok(())
    })
}
