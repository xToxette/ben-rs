#![warn(clippy::str_to_string)]

mod dal;
mod event_handler;
mod commands;
mod handlers;

use sqlx::{sqlite::SqlitePoolOptions, Acquire};
use poise::{Event, serenity_prelude as serenity};
use std::{collections::HashMap, env::var, sync::Mutex, time::Duration};
use sqlx::{SqliteConnection, SqlitePool};
use songbird::serenity::SerenityInit;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

// Custom user data passes to all command functions
pub struct Data {
    conn_pool: SqlitePool,
}

fn prefix_options() -> poise::PrefixFrameworkOptions<Data, Error> {
    poise::PrefixFrameworkOptions {
        prefix: Some("#".into()),
        additional_prefixes: vec![
            poise::Prefix::Literal("ben, "),
        ],
        ..Default::default()
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    env_logger::init();

    let options = poise::FrameworkOptions {
        commands: vec![
            commands::utilities::common::information()
        ],
        prefix_options: prefix_options(),
        on_error: handlers::error::handler,
        pre_command: handlers::pre_command::handler,
        post_command: handlers::post_command::handler,
        event_handler: event_handler::main_handler,
        ..Default::default()
    };
    
    poise::Framework::builder()
        .token(var("DISCORD_TOKEN").expect("Missing `DISCORD_TOKEN` env var") )
        .setup(handlers::setup::handler)
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
