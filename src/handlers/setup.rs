use crate::{Error, Data};
use poise::{
    serenity_prelude::{
        Context as SerenityContext,
        Ready,
    },
    BoxFuture,
    Framework
};
use sqlx::sqlite::SqlitePool;
use std::env::var;
use poise::serenity_prelude::Activity;

pub fn handler<'a>(
    ctx: &'a SerenityContext,
    _ready: &'a Ready,
    framework: &'a Framework<Data, Error>,
) -> BoxFuture<'a, Result<Data, Error>> {
    Box::pin(async move {
        println!("Logged in as {}", _ready.user.name);

        poise::builtins::register_globally(ctx, &framework.options().commands).await?;

        ctx.set_activity(Activity::listening("to you")).await;

        let database_url = var("DATABASE_URL").expect("Missing `DATABASE_URL` env var");
        let data = Data {
            conn_pool: SqlitePool::connect(&database_url).await?
        };
        Ok(data)
    })
}