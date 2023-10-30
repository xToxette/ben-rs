use poise::serenity_prelude::{GuildId, VoiceState};
use sqlx::{sqlite, SqlitePool, Error, Row};
use crate::{dal, dal::models};

/// Inserts a voice state update event into the table
/// Automatically handles inserting a new row into the general_info table
pub async fn insert(executor: &SqlitePool, voice_state: &VoiceState) -> Result<sqlite::SqliteQueryResult, Error> {
    let general_info_result = dal::events::general_info::insert(&executor,
        std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() as i64,
        voice_state.user_id.0 as i64,
        voice_state.guild_id.map(|i| i.0 as i64)
    ).await?;

    let general_info_id = general_info_result.last_insert_rowid();

    let voice_channel_id = voice_state.channel_id.map(|channel_id| channel_id.0 as i64);
    let self_stream = voice_state.self_stream.unwrap_or(false);
    sqlx::query!( r#"
        INSERT INTO events_voice_state_update (
            general_event_info_id,
            voice_channel_id,
            self_muted,
            self_deafened,
            self_streaming,
            self_video,
            deafened,
            muted,
            suppressed
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        general_info_id,
        voice_channel_id,
        voice_state.self_mute,
        voice_state.self_deaf,
        self_stream,
        voice_state.self_video,
        voice_state.deaf,
        voice_state.mute,
        voice_state.suppress
    ).execute(executor).await
}

pub async fn get_voice_state_updates(executor: &SqlitePool, user_id: i64, guild_id: Option<i64>) -> Result<Vec<models::DBVoiceStateUpdate>, Error> {
    let db_voice_state_updates = sqlx::query(r#"
        SELECT
            egi.id as "egi_id",
            egi.timestamp,
            egi.user_id,
            egi.guild_id,
            events_voice_state_update.id as "events_voice_state_update_id",
            events_voice_state_update.voice_channel_id,
            events_voice_state_update.self_muted,
            events_voice_state_update.self_deafened,
            events_voice_state_update.self_streaming,
            events_voice_state_update.self_video,
            events_voice_state_update.deafened,
            events_voice_state_update.muted,
            events_voice_state_update.suppressed
        FROM events_voice_state_update
        JOIN events_general_info egi on egi.id = events_voice_state_update.general_event_info_id
        WHERE egi.user_id = ? and egi.guild_id = ?"#
    ).bind(user_id).bind(guild_id) .fetch_all(executor).await?;

    let voice_state_updates = db_voice_state_updates.iter().map(|row| models::DBVoiceStateUpdate::from_joined(row)).collect();

    Ok(voice_state_updates)
}
