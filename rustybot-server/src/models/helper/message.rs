use std::collections::HashMap;

use crate::models::{Message, MessageMedia, MessageSender, MessageModel};
use chrono::Utc;
use rustybot_macros::get_connection;
use sqlx::{types::Json, Acquire};

impl Message {
    /// Create a new chat entity that can be saved to database.
    ///
    /// # Arguments
    /// - `cid`: chat ID
    pub fn new(
        cid: i32,
        model: MessageModel,
        sender: MessageSender,
        content: String,
        media: Option<HashMap<String, MessageMedia>>,
    ) -> Self {
        let formed_media: Option<Json<HashMap<String, Json<MessageMedia>>>> = if media.is_none() {
            None
        } else {
            let map = media.unwrap();
            let mut updated_map: HashMap<String, Json<MessageMedia>> = HashMap::new();
            map.iter().for_each(|(key, med)| {
                updated_map.insert(key.clone(), Json::<_>(med.clone()));
            });
            Some(Json::<_>(updated_map))
        };
        Self {
            msg_id: None,
            msg_chat_id: cid,
            msg_model: model,
            msg_sender: sender,
            msg_content: content,
            msg_medias: formed_media,
            msg_created_at: Utc::now(),
        }
    }
}

/// Methods that implement SQL operations.
impl Message {
    /// Save NEW message into database.
    pub async fn save(&self) -> Result<Self, Box<dyn std::error::Error>> {
        get_connection!();

        let mut key_pairs: Vec<&str> = vec![];
        let mut value_pairs: Vec<String> = vec![];

        key_pairs.push("`msg_chat_id`");
        value_pairs.push(format!("{}", self.msg_chat_id));

        key_pairs.push("`msg_model`");
        value_pairs.push(format!("{}", Into::<i8>::into(self.msg_model.clone())));

        key_pairs.push("`msg_sender`");
        value_pairs.push(format!("{}", Into::<i8>::into(self.msg_sender.clone())));

        key_pairs.push("`msg_content`");
        value_pairs.push(format!("'{}'", self.msg_content.clone().replace("'", "''")));

        if self.msg_medias.is_some() {
            key_pairs.push("`msg_medias`");
            value_pairs.push(format!(
                "'{}'",
                serde_json::to_string(&self.msg_medias.clone().unwrap())
                    .unwrap()
                    .replace("'", "''")
            ));
        }

        key_pairs.push("`msg_created_at`");
        value_pairs.push(format!("'{}'", self.msg_created_at.format("%+")));

        let query_string = format!(
            "INSERT INTO `tbl_msg` ({}) VALUES ({})",
            key_pairs.join(", "),
            value_pairs.join(", ")
        );

        log::debug!(target: "sql", "{}", query_string);

        let mut trans = connection.begin().await.unwrap();

        sqlx::query(&query_string)
            .execute(&mut trans)
            .await
            .unwrap();

        let msg: Self =
            sqlx::query_as("SELECT * FROM `tbl_msg` WHERE `tbl_msg`.`msg_id`= LAST_INSERT_ID()")
                .fetch_one(&mut trans)
                .await
                .unwrap();

        trans.commit().await.unwrap();
        Ok(msg)
    }

    pub async fn find_messages_by_chat(cid: i32) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        get_connection!();

        let sql_raw = format!(
            "SELECT * FROM `tbl_msg` WHERE `tbl_msg`.`msg_chat_id` = {}",
            cid
        );
        log::debug!(target: "sql", "{sql_raw}");
        Ok(sqlx::query_as(&sql_raw)
            .fetch_all(&mut connection)
            .await
            .unwrap())
    }
}
