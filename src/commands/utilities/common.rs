use poise::{serenity_prelude as serenity};
use serenity::Timestamp;
use crate::{Context, Error};
use sysinfo::{System, SystemExt};

/// Informs you about the meaning of ben's existence
#[poise::command(prefix_command, slash_command, ephemeral, broadcast_typing)]
pub async fn information(
    ctx: Context<'_>,
) -> Result<(), Error> {
    ctx.send(|m| {
        m.content("Ben is currently being development")
            .embed(|e| {
                e.title("Development")
                    .description("Please contribute, otherwise I'm so lonely: https://github.com/xToxette/ben-rs")
                    .timestamp(Timestamp::now())
            })
    }).await?;
    Ok(())
}



#[derive(poise::ChoiceParameter)]
pub enum PerformanceType {
    CPU,
    RAM,
    Disk,
    Delay,
    All
}

/// Check the current performance of ben
#[poise::command(slash_command, ephemeral, broadcast_typing)]
pub async fn performance(
    ctx: Context<'_>,
    #[description = "What to check"] category: PerformanceType
) -> Result<(), Error> {
    let system = System::new_all();

    match category {
        PerformanceType::RAM => {
            ctx.send(|m| {
                m.content("Here is the performance information that you requested")
                    .embed(|e| {
                        e.title("Performance")
                            .fields(vec![
                                ("\u{200B}", &format!("```cs\n[total]\n``````py\n{} GB\n```", system.total_memory() / 1073741824), true),
                                ("\u{200B}", &format!("```cs\n[used]\n``````py\n{} GB\n```", system.used_memory() / 1073741824), true),
                                ("\u{200B}", &format!("```cs\n[free]\n``````py\n{} GB\n```", system.available_memory() / 1073741824), true),
                            ])
                    })
            }).await?;
        }
        _ => {
            ctx.send(|m| {
                m.content("The metric you requested has not been implemented yet")
                    .embed(|e| {
                        e.title("Development")
                            .description("But you can implement it yourself: https://github.com/xToxette/ben-rs")
                            .timestamp(Timestamp::now())
                })
            }).await?;
        }
    }
    Ok(())
}