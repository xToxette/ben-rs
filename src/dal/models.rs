use sqlx::Row;

#[derive(Debug)]
pub struct DBVoiceStateUpdate {
    pub id: i64,
    pub general_info: DBGeneralInfo,
    pub voice_channel_id: Option<i64>,
    pub self_muted: bool,
    pub self_deafened: bool,
    pub self_streaming: bool,
    pub self_video: bool,
    pub deafened: bool,
    pub muted: bool,
    pub suppressed: bool
}

impl DBVoiceStateUpdate {
    /// Create a DBVoiceStateUpdate from a joined query
    /// event_voice_state_update's id must be aliased as "events_voice_state_update_id"
    /// events_general_info's id must be aliased as "egi_id"
    pub fn from_joined(row: &sqlx::sqlite::SqliteRow) -> Self {
        Self {
            id: row.get("events_voice_state_update_id"),
            general_info: DBGeneralInfo {
                id: row.get("egi_id"),
                timestamp: row.get("timestamp"),
                user_id: row.get("user_id"),
                guild_id: row.get("guild_id")
            },
            voice_channel_id: row.get("voice_channel_id"),
            self_muted: row.get("self_muted"),
            self_deafened: row.get("self_deafened"),
            self_streaming: row.get("self_streaming"),
            self_video: row.get("self_video"),
            deafened: row.get("deafened"),
            muted: row.get("muted"),
            suppressed: row.get("suppressed")
        }
    }
}

#[derive(Debug)]
pub struct DBGeneralInfo {
    pub id: i64,
    pub timestamp: i64,
    pub user_id: i64,
    pub guild_id: Option<i64>
}

#[derive(Debug)]
pub struct DBMessage {
    id: i64,
    general_event_info_id: i64,
    text_channel_id: i64,
    message_id: i64,
    content: String
}
