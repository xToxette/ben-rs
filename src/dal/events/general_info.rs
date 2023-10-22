use sqlx::{
    self,
    Error,
    sqlite,
    SqlitePool
};

pub async fn insert(executor: &SqlitePool, timestamp: i64, user_id: i64, guild_id: Option<i64>) -> Result<sqlite::SqliteQueryResult, Error> {
    sqlx::query!(
        "INSERT INTO events_general_info (timestamp, user_id, guild_id) VALUES (?, ?, ?)",
        timestamp, user_id, guild_id
    ).execute(executor).await
}