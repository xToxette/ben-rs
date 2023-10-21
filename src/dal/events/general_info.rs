use sqlx::{
    self,
    Database,
    Error,
    sqlite,
    SqliteExecutor,
    SqlitePool
};

pub async fn insert(mut executor: impl SqliteExecutor<'_>, timestamp: u64, user_id: u64, guild_id: u64) -> Result<sqlite::SqliteQueryResult, Error> {
    sqlx::query!(
        "INSERT INTO events_general_info (timestamp, user_id, guild_id) VALUES (?, ?, ?)",
        timestamp, user_id, guild_id
    ).execute(&mut executor).await
}