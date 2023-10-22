use crate::{Error, Data};
use poise::{
    BoxFuture
};

pub fn handler(
    error: poise::FrameworkError<'_, Data, Error>
) -> BoxFuture<'_, ()> {
    Box::pin(async move {
        match error {
            poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
            poise::FrameworkError::Command { error, ctx } => {
                println!("Error in command `{}`: {:?}", ctx.command().name, error,)
            }
            error => {
                if let Err(e) = poise::builtins::on_error(error).await {
                    println!("Error while handling error: {}", e);
                }
            }
        }
    })
}