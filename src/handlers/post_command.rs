use crate::{Context, Error};
use poise::{
    BoxFuture
};

pub fn handler(ctx: Context<'_>) -> BoxFuture<'_, ()> {
    Box::pin(async move {
        println!("Executed command {}!", ctx.command().qualified_name)
    })
}