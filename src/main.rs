#![warn(clippy::str_to_string)]

mod common;

use poise::serenity_prelude as serenity;
use std::{collections::HashMap, env::var, sync::Mutex, time::Duration};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passes to all command functions
pub struct Data {
    ben: Mutex<bool>,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
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
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    env_logger::init();

    let options = poise::FrameworkOptions {
        commands: vec![common::information()],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("#".into()),
            additional_prefixes: vec![
                poise::Prefix::Literal("ben, "),
            ],
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        pre_command: |ctx| {
            Box::pin(async move {
                println!("Executing command {}...", ctx.command().qualified_name)
            })
        },
        post_command: |ctx| {
            Box::pin(async move {
                println!("Executed command {}!", ctx.command().qualified_name)
            })
        },
        command_check: Some(|ctx| {
            Box::pin(async move {
                Ok(true)
            })
        }),
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(async move {
                println!("Got an event in event handler: {:?}", event.name());
                Ok(())
            })
        },
        ..Default::default()
    };
    
    poise::Framework::builder()
        .token(
            var("DISCORD_TOKEN")
                .expect("Missing `DISCORD_TOKEN` env var")
        )
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                println!("Logged in as {}", _ready.user.name);
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    ben: Mutex::new(true),
                })
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::GUILD_MESSAGES | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .run()
        .await
        .unwrap();
}