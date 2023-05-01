#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Auth {
    pub(in crate::models) auth_id: Option<i32>,
    pub(in crate::models) auth_user_id: i32,
    pub(in crate::models) auth_key: String,
}
