use crate::{
    models::{Auth, User},
    utils::sql::check_sql_component,
};
use rustybot_macros::get_connection;
use sha2::{Digest, Sha512};
use sqlx::Acquire;

impl Auth {
    /// Directly query user's authentication information by given name.
    ///
    /// # Arguments
    /// - `id`: login name of the user. Not its database ID.
    pub async fn auth(id: &str) -> Result<Option<Auth>, Box<dyn std::error::Error>> {
        if let Ok(id) = check_sql_component(id) {
            get_connection!();

            let sql_raw = format!("SELECT a.* FROM `tbl_auth` a LEFT JOIN `tbl_user` u ON a.auth_user_id = u.user_id WHERE u.user_name = '{}'", id);
            log::debug!(target: "sql", "{sql_raw}");
            Ok(sqlx::query_as(&sql_raw)
                .fetch_optional(&mut connection)
                .await
                .unwrap())
        } else {
            Err(format!("Invalid SQL component `{id}`").into())
        }
    }

    pub(in crate::models) async fn create(&self) -> Auth {
        get_connection!();

        let mut key_pairs: Vec<&str> = vec![];
        let mut value_pairs: Vec<String> = vec![];

        key_pairs.push("`auth_key`");
        value_pairs.push(format!("'{}'", self.key()));

        key_pairs.push("`auth_user_id`");
        value_pairs.push(format!("{}", self.auth_user_id.clone()));

        let query_string = format!(
            "INSERT INTO `tbl_auth` ({}) VALUES ({})",
            key_pairs.join(", "),
            value_pairs.join(", ")
        );

        log::debug!(target: "sql", "{}", query_string);

        let mut trans = connection.begin().await.unwrap();

        sqlx::query(&query_string)
            .execute(&mut trans)
            .await
            .unwrap();

        let auth: Self =
            sqlx::query_as("SELECT * FROM `tbl_auth` WHERE `tbl_auth`.`auth_id`= LAST_INSERT_ID()")
                .fetch_one(&mut trans)
                .await
                .unwrap();

        trans.commit().await.unwrap();
        auth
    }

    pub(in crate::models) async fn user(&self) -> User {
        User::find_by_id(self.auth_user_id).await.unwrap().unwrap()
    }
}

impl Auth {
    /// Create new auth info instance.
    pub fn new(user_id: i32, key: &str) -> Self {
        Self {
            auth_id: None,
            auth_user_id: user_id,
            auth_key: key.to_string(),
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.auth_id.clone()
    }

    pub fn key(&self) -> String {
        self.auth_key.clone()
    }
}

impl Auth {
    pub async fn hash(&self, salt: &str) -> String {
        let mut buf = [0u8; 1024];
        let user = self.user().await;
        let input = format!("{}{}{}", user.user_name, self.auth_key, salt);
        let mut hasher: Sha512 = Sha512::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_str(&hash, &mut buf).unwrap();
        hex_hash.to_string()
    }
}
