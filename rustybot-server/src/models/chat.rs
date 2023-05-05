use chrono::{DateTime, NaiveDate, Utc};
#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Chat {
    pub chat_id: Option<i32>,
    pub chat_user_id: i32,
    pub chat_created_at: DateTime<Utc>,
    pub chat_date: NaiveDate,
    pub chat_summary: Option<String>,
}