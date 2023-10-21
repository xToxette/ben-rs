use poise::{CreateReply, serenity_prelude as serenity};
use serenity::Timestamp;
use crate::{Context, Error};
use sysinfo::{CpuExt, System, SystemExt};

use serenity::builder::{CreateEmbed, CreateMessage};

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
    let mut system = System::new_all();

    match category {
        PerformanceType::CPU => {
            ctx.send(|m| {
                m.content(format!("{}", system.global_cpu_info().cpu_usage()))
            }).await?;
        }
        PerformanceType::RAM => {
            ctx.send(|m| {
                m.content(format!("RAM: {}gb used /{}gb total", system.available_memory() / 1073741824, system.total_memory() / 1073741824))
            }).await?;
        }
        _ => {}
    }
    Ok(())
}