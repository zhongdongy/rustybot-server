use std::collections::HashMap;

use chrono::{DateTime, Utc};
#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub msg_id: Option<i32>,
    pub msg_chat_id: i32,
    pub msg_sender: MessageSender,
    pub msg_content: String,
    pub msg_medias: Option<sqlx::types::Json<HashMap<String, sqlx::types::Json<MessageMedia>>>>,
    pub msg_created_at: DateTime<Utc>,
}

impl Message {
    /// Insert message medias into it
    pub fn message(&self) -> String {
        let updated_message = if self.msg_medias.is_none() {
            self.msg_content.clone()
        } else {
            let medias = self.msg_medias.clone().unwrap();
            let mut raw_msg = self.msg_content.clone();
            for (name, media) in medias.iter() {
                let key = format!("${{{{{}}}}}", name);
                if raw_msg.contains(&key) {
                    raw_msg = raw_msg.replace(&key, &format!("![]({})", media.url));
                }
            }
            raw_msg
        };

        updated_message.replace("\\$\\{\\{", "${{")
    }
}

#[derive(Debug, Clone)]
pub enum MessageSender {
    User,
    Assistant,
    System,
}

impl From<i8> for MessageSender {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::User,
            1 => Self::Assistant,
            2 => Self::System,
            _ => panic!("Impossible message sender value `{value}`"),
        }
    }
}

impl Into<i8> for MessageSender {
    fn into(self) -> i8 {
        match self {
            MessageSender::User => 0,
            MessageSender::Assistant => 1,
            MessageSender::System => 2,
        }
    }
}

impl sqlx::Type<sqlx::MySql> for MessageSender {
    fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
        i8::type_info()
    }
}

impl<'r> sqlx::Decode<'r, sqlx::MySql> for MessageSender {
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

impl serde::Serialize for MessageSender {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            MessageSender::User => serializer.serialize_i8(0),
            MessageSender::Assistant => serializer.serialize_i8(1),
            MessageSender::System => serializer.serialize_i8(2),
        }
    }
}

struct MessageSenderVisitor;

impl<'de> serde::de::Visitor<'de> for MessageSenderVisitor {
    type Value = MessageSender;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Acceptable values: 0, 1")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            0 => Ok(MessageSender::User),
            1 => Ok(MessageSender::Assistant),
            2 => Ok(MessageSender::System),
            _ => Err(E::custom(format!("Unsupported message sender `{v}`"))),
        }
    }
}

impl<'de> serde::Deserialize<'de> for MessageSender {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i8(MessageSenderVisitor)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct MessageMedia {
    #[serde(rename = "type")]
    pub ty: MediaType,
    pub url: String,
}

#[derive(Debug, Clone)]
pub enum MediaType {
    Image,
    Audio,
    Video,
}

impl From<i8> for MediaType {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::Image,
            1 => Self::Audio,
            2 => Self::Video,
            _ => panic!("Impossible media type value `{value}`"),
        }
    }
}

impl sqlx::Type<sqlx::MySql> for MediaType {
    fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
        i8::type_info()
    }
}

impl<'r> sqlx::Decode<'r, sqlx::MySql> for MediaType {
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

impl serde::Serialize for MediaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            MediaType::Image => serializer.serialize_i8(0),
            MediaType::Audio => serializer.serialize_i8(1),
            MediaType::Video => serializer.serialize_i8(2),
        }
    }
}

struct MediaTypeVisitor;

macro_rules! visit_int {
    ($int:ty) => {
        paste::paste! {
            fn [<visit_ $int>]<E>(self, v: $int) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match v {
                    0 => Ok(MediaType::Image),
                    1 => Ok(MediaType::Audio),
                    2 => Ok(MediaType::Video),
                    _ => Err(E::custom(format!("Unsupported media type `{}`",v))),
                }
            }
        }
    };
}

impl<'de> serde::de::Visitor<'de> for MediaTypeVisitor {
    type Value = MediaType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("values: 0, 1, 2")
    }

    visit_int!(i8);
    visit_int!(i16);
    visit_int!(i32);
    visit_int!(i64);
    visit_int!(i128);
    visit_int!(u8);
    visit_int!(u16);
    visit_int!(u32);
    visit_int!(u64);
    visit_int!(u128);
}

impl<'de> serde::Deserialize<'de> for MediaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(MediaTypeVisitor)
    }
}
