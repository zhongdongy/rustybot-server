use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub(super) user_id: Option<i32>,
    pub(super) user_name: String,
    pub(super) user_display_name: String,
    pub(super) user_avatar: Option<String>,
    pub(super) user_role: UserRole,
    pub(super) user_state: UserState,
    pub(super) user_created_at: DateTime<Utc>,
    pub(super) user_active_at: DateTime<Utc>,

    #[serde(skip)]
    pub(crate) __content_updated: bool,
    
    #[serde(skip)]
    pub(crate) __auth_key: Option<String>,
}

impl sqlx::FromRow<'_, sqlx::mysql::MySqlRow> for User {
    fn from_row(row: &sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(Self {
            user_id: row.try_get("user_id")?,
            user_name: row.try_get("user_name")?,
            user_display_name: row.try_get("user_display_name")?,
            user_avatar: row.try_get("user_avatar")?,
            user_role: row.try_get("user_role")?,
            user_state: row.try_get("user_state")?,
            user_created_at: row.try_get("user_created_at")?,
            user_active_at: row.try_get("user_active_at")?,
            __content_updated: false,
            __auth_key: None,
        })
    }
}

#[derive(Debug, Clone)]
pub enum UserRole {
    Normal,
    Admin,
}

impl From<i8> for UserRole {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::Normal,
            1 => Self::Admin,
            _ => panic!("Impossible user role value `{value}`"),
        }
    }
}

impl Into<i8> for UserRole {
    fn into(self) -> i8 {
        match self {
            UserRole::Normal => 0,
            UserRole::Admin => 1,
        }
    }
}

impl sqlx::Type<sqlx::MySql> for UserRole {
    fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
        i8::type_info()
    }
}

impl<'r> sqlx::Decode<'r, sqlx::MySql> for UserRole {
    fn decode(
        value: <sqlx::MySql as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError>
    where
        i8: sqlx::Decode<'r, sqlx::MySql>,
    {
        let value = <i8 as sqlx::Decode<sqlx::MySql>>::decode(value).unwrap();
        Ok(value.into())
    }
}

impl serde::Serialize for UserRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            UserRole::Normal => serializer.serialize_i8(0),
            UserRole::Admin => serializer.serialize_i8(1),
        }
    }
}

struct UserRoleVisitor;

impl<'de> serde::de::Visitor<'de> for UserRoleVisitor {
    type Value = UserRole;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Acceptable values: 0, 1")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            0 => Ok(UserRole::Normal),
            1 => Ok(UserRole::Admin),
            _ => Err(E::custom(format!("Unsupported user role `{v}`"))),
        }
    }
}

impl<'de> serde::Deserialize<'de> for UserRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i8(UserRoleVisitor)
    }
}

#[derive(Debug, Clone)]
pub enum UserState {
    Active,
    Inactive,
}

impl Into<i8> for UserState {
    fn into(self) -> i8 {
        match self {
            UserState::Active => 0,
            UserState::Inactive => 1,
        }
    }
}

impl From<i8> for UserState {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::Active,
            1 => Self::Inactive,
            _ => panic!("Impossible user role value `{value}`"),
        }
    }
}

impl sqlx::Type<sqlx::MySql> for UserState {
    fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
        i8::type_info()
    }
}

impl<'r> sqlx::Decode<'r, sqlx::MySql> for UserState {
    fn decode(
        value: <sqlx::MySql as sqlx::database::HasValueRef<'r>>::ValueRef,
    ) -> Result<Self, sqlx::error::BoxDynError>
    where
        i8: sqlx::Decode<'r, sqlx::MySql>,
    {
        let value = <i8 as sqlx::Decode<sqlx::MySql>>::decode(value).unwrap();
        Ok(value.into())
    }
}

impl serde::Serialize for UserState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            UserState::Active => serializer.serialize_i8(0),
            UserState::Inactive => serializer.serialize_i8(1),
        }
    }
}

struct UserStateVisitor;

impl<'de> serde::de::Visitor<'de> for UserStateVisitor {
    type Value = UserState;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Acceptable values: 0, 1")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            0 => Ok(UserState::Active),
            1 => Ok(UserState::Inactive),
            _ => Err(E::custom(format!("Unsupported user role `{v}`"))),
        }
    }
}

impl<'de> serde::Deserialize<'de> for UserState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i8(UserStateVisitor)
    }
}
