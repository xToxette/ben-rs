#![warn(clippy::str_to_string)]

mod common;
mod dal;
mod event_handler;

use sqlx::{sqlite::SqlitePoolOptions, Acquire};
use poise::{Event, serenity_prelude as serenity};
use std::{collections::HashMap, env::var, sync::Mutex, time::Duration};
use sqlx::{SqliteConnection, SqlitePool};
use std::{option};

use serenity::VoiceState;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passes to all command functions
pub struct Data {
    conn_pool: SqlitePool,
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
        event_handler: |_ctx, event, _framework, _data| {
            Box::pin(event_handler::main_handler(_ctx, event, _framework, _data))
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

                let database_url = var("DATABASE_URL").expect("Missing `DATABASE_URL` env var");
                let data = Data {
                    conn_pool: SqlitePool::connect(&database_url).await?
                };
                sqlx::migrate!().run(&data.conn_pool).await?;

                Ok(data)
            })
        })
        .options(options)
        .intents(
            serenity::GatewayIntents::GUILD_MESSAGES
                | serenity::GatewayIntents::MESSAGE_CONTENT
                | serenity::GatewayIntents::GUILDS
                | serenity::GatewayIntents::GUILD_VOICE_STATES,
        )
        .run()
        .await
        .unwrap();
}
