use sqlx::{
    self,
    Database,
    Error,
    sqlite,
    SqliteExecutor,
    SqlitePool
};

pub async fn insert(mut executor: &SqlitePool, timestamp: i64, user_id: i64, guild_id: i64) -> Result<sqlite::SqliteQueryResult, Error> {
    sqlx::query!(
        "INSERT INTO events_general_info (timestamp, user_id, guild_id) VALUES (?, ?, ?)",
        timestamp, user_id, guild_id
    ).execute(executor).await
}