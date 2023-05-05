use crate::models::{Chat, Message};
use chrono::Utc;
use rustybot_macros::get_connection;
use sqlx::Acquire;

impl Chat {
    /// Create a new chat entity that can be saved to database.
    pub fn new(uid: i32) -> Self {
        Self {
            chat_id: None,
            chat_user_id: uid,
            chat_created_at: Utc::now(),
            chat_date: Utc::now().date_naive(),
            chat_summary: None,
        }
    }

    /// Get message history of current chat entity.
    pub async fn history(&self) -> Vec<Message> {
        if self.chat_id.is_none() {
            return vec![];
        }
        Message::find_messages_by_chat(self.chat_id.clone().unwrap())
            .await
            .unwrap()
    }
}

/// Methods that implement SQL operations.
impl Chat {
    pub async fn save(&self) -> Result<Self, Box<dyn std::error::Error>> {
        get_connection!();

        let mut key_pairs: Vec<&str> = vec![];
        let mut value_pairs: Vec<String> = vec![];

        key_pairs.push("`chat_user_id`");
        value_pairs.push(format!("{}", self.chat_user_id));

        key_pairs.push("`chat_created_at`");
        value_pairs.push(format!("'{}'", self.chat_created_at.format("%+")));

        key_pairs.push("`chat_date`");
        value_pairs.push(format!("'{}'", self.chat_date.format("%F")));

        if self.chat_summary.is_some() {
            key_pairs.push("`chat_summary`");
            value_pairs.push(format!("'{}'", self.chat_summary.clone().unwrap()));
        }

        let query_string = format!(
            "INSERT INTO `tbl_chat` ({}) VALUES ({})",
            key_pairs.join(", "),
            value_pairs.join(", ")
        );

        log::debug!(target: "sql", "{}", query_string);

        let mut trans = connection.begin().await.unwrap();

        sqlx::query(&query_string)
            .execute(&mut trans)
            .await
            .unwrap();

        let chat: Self =
            sqlx::query_as("SELECT * FROM `tbl_chat` WHERE `tbl_chat`.`chat_id`= LAST_INSERT_ID()")
                .fetch_one(&mut trans)
                .await
                .unwrap();

        trans.commit().await.unwrap();
        Ok(chat)
    }

    pub async fn find_chats_by_user(uid: i32) -> Result<Vec<Self>, Box<dyn std::error::Error>> {
        get_connection!();

        let sql_raw = format!(
            "SELECT * FROM `tbl_chat` WHERE `tbl_chat`.`chat_user_id` = {}",
            uid
        );
        log::debug!(target: "sql", "{sql_raw}");
        Ok(sqlx::query_as(&sql_raw)
            .fetch_all(&mut connection)
            .await
            .unwrap())
    }

    pub async fn chat_by_id(cid: i32) -> Result<Option<Chat>, Box<dyn std::error::Error>> {
      get_connection!();

      let sql_raw = format!(
          "SELECT * FROM `tbl_chat` WHERE `tbl_chat`.`chat_id` = {}",
          cid
      );
      log::debug!(target: "sql", "{sql_raw}");
      Ok(sqlx::query_as(&sql_raw)
          .fetch_optional(&mut connection)
          .await
          .unwrap())
  }
}
