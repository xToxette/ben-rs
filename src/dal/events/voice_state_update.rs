use poise::serenity_prelude::{GuildId, VoiceState};
use sqlx::{sqlite, SqliteExecutor, SqlitePool, Database, Error, Sqlite};
use crate::{dal};

/// Inserts a voice state update event into the table
/// Automatically handles inserting a new row into the general_info table
pub async fn insert(mut executor: &SqlitePool, voice_state: &VoiceState) -> Result<sqlite::SqliteQueryResult, Error> {
    let general_info_result = dal::events::general_info::insert(&executor,
                                      std::time::UNIX_EPOCH.elapsed().unwrap().as_secs() as i64,
                                      voice_state.user_id.0 as i64,
                                      voice_state.guild_id.unwrap_or(GuildId(0)).0 as i64).await?;

    let general_info_id = general_info_result.last_insert_rowid();
    let voice_channel_id: Option<i64> = match voice_state.channel_id {
        Some(channel_id) => Some(channel_id.0 as i64),
        None => None
    };
    let self_stream: bool = match voice_state.self_stream {
        Some(stream) => stream,
        None => false
    };

    sqlx::query!("INSERT INTO events_voice_state_update (general_event_info_id, \
                                                        voice_channel_id, \
                                                        self_muted, \
                                                        self_deafened, \
                                                        self_streaming, \
                                                        self_video, \
                                                        deafened, \
                                                        muted, \
                                                        suppressed)\
                    VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
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