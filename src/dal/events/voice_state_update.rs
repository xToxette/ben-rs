use poise::serenity_prelude::{GuildId, VoiceState};
use sqlx::{sqlite, SqliteExecutor, SqlitePool, Database, Error, Sqlite};
use crate::{dal};

/// Inserts a voice state update event into the table
/// Automatically handles inserting a new row into the general_info table
pub async fn insert(mut executor: impl SqliteExecutor<'_>, voice_state: &VoiceState) -> Result<sqlite::SqliteQueryResult, Error> {
    let general_info_result = dal::events::general_info::insert(&mut executor,
                                      std::time::UNIX_EPOCH.elapsed().unwrap().as_secs(),
                                      voice_state.user_id.0,
                                      voice_state.guild_id.unwrap_or(GuildId(0)).0).await?;

    let general_info_id = general_info_result.last_insert_rowid();
    let voice_channel_id: Option<u64> = match voice_state.channel_id {
        Some(channel_id) => Some(channel_id.0),
        None => None
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
        voice_state.self_stream.unwrap_or(false),
        voice_state.self_video,
        voice_state.deaf,
        voice_state.mute,
        voice_state.suppress
    ).execute(&mut executor).await

}