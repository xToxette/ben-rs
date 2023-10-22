use crate::{Context};
use poise::{
    BoxFuture
};

pub fn handler(ctx: Context<'_>) -> BoxFuture<'_, ()> {
    Box::pin(async move {
        println!("Executing command {}...", ctx.command().qualified_name)
    })
}