#[derive(sqlx::FromRow, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Quota {
    pub quota_id: i32,
    pub quota_user_id: i32,
    pub quota_type: QuotaType,
    pub quota_total: i32,
    pub quota_used: i32,
}

#[derive(Debug, Clone)]
pub enum QuotaType {
    ChatCompletion,
    ImageGeneration,
    TextToSpeech,
}

impl From<i8> for QuotaType {
    fn from(value: i8) -> Self {
        match value {
            0 => Self::ChatCompletion,
            1 => Self::ImageGeneration,
            2 => Self::TextToSpeech,
            _ => panic!("Impossible quota type value `{value}`"),
        }
    }
}

impl sqlx::Type<sqlx::MySql> for QuotaType {
    fn type_info() -> <sqlx::MySql as sqlx::Database>::TypeInfo {
        i8::type_info()
    }
}

impl<'r> sqlx::Decode<'r, sqlx::MySql> for QuotaType {
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

impl serde::Serialize for QuotaType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            QuotaType::ChatCompletion => serializer.serialize_i8(0),
            QuotaType::ImageGeneration => serializer.serialize_i8(1),
            QuotaType::TextToSpeech => serializer.serialize_i8(2),
        }
    }
}

struct QuotaTypeVisitor;

impl<'de> serde::de::Visitor<'de> for QuotaTypeVisitor {
    type Value = QuotaType;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("Acceptable values: 0, 1, 2")
    }

    fn visit_i8<E>(self, v: i8) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            0 => Ok(QuotaType::ChatCompletion),
            1 => Ok(QuotaType::ImageGeneration),
            2 => Ok(QuotaType::TextToSpeech),
            _ => Err(E::custom(format!("Unsupported quota type `{v}`"))),
        }
    }
}

impl<'de> serde::Deserialize<'de> for QuotaType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i8(QuotaTypeVisitor)
    }
}
