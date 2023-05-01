use crate::{
    models::{Auth, User, UserRole, UserState},
    utils::sql::check_sql_component,
};
use chrono::{DateTime, Utc};
use rustybot_macros::get_connection;
use sqlx::Acquire;

/// Helper functions for managing user.
///
/// Internally uses SQL to create/update/query users.
impl User {
    /// Query user entity by its `user_name` field.
    pub async fn find_by_name(id: &str) -> Result<Option<User>, Box<dyn std::error::Error>> {
        if let Ok(id) = check_sql_component(id) {
            get_connection!();
            Ok(
                sqlx::query_as("SELECT * FROM `tbl_user` WHERE `tbl_user`.`user_name` = ?")
                    .bind(id)
                    .fetch_optional(&mut connection)
                    .await
                    .unwrap(),
            )
        } else {
            Err(format!("Invalid SQL component `{id}`").into())
        }
    }
    /// Query user entity by its `user_id` field.
    pub async fn find_by_id(id: i32) -> Result<Option<User>, Box<dyn std::error::Error>> {
        get_connection!();
        Ok(
            sqlx::query_as("SELECT * FROM `tbl_user` WHERE `tbl_user`.`user_id` = ?")
                .bind(id)
                .fetch_optional(&mut connection)
                .await
                .unwrap(),
        )
    }

    pub async fn auth(&self) -> Auth {
        get_connection!();
        sqlx::query_as("SELECT * FROM `tbl_auth` WHERE `auth_user_id` = ?;")
            .bind(self.user_id)
            .fetch_one(&mut connection)
            .await
            .unwrap()
    }

    /// Save any updates to current user entity. Only Ok(true) indicates a
    /// successful update operation. Ok(false) means no need to update.
    pub async fn save(&self) -> Result<bool, Box<dyn std::error::Error>> {
        if self.user_id.is_none() {
            return Err("User ID not ready. Query from DB first.".into());
        }

        get_connection!();

        if self.__content_updated == false {
            return Ok(false);
        }
        let mut value_pairs: Vec<String> = vec![];

        if self.user_avatar.is_some() {
            value_pairs.push(format!(
                "`tbl_user`.`user_avatar` = '{}'",
                self.user_avatar.as_ref().unwrap()
            ));
        }

        value_pairs.push(format!(
            "`tbl_user`.`user_display_name` = '{}'",
            self.user_display_name
        ));

        value_pairs.push(format!(
            "`tbl_user`.`user_role` = {}",
            Into::<i8>::into(self.user_role.clone())
        ));

        value_pairs.push(format!(
            "`tbl_user`.`user_state` = {}",
            Into::<i8>::into(self.user_state.clone())
        ));

        let query_string = format!(
            "UPDATE `tbl_user` SET {} WHERE `tbl_user`.`user_id` = ?",
            value_pairs.join(", ")
        );

        log::debug!(target: "sql", "{}", query_string);

        sqlx::query(&query_string)
            .bind(self.user_id)
            .execute(&mut connection)
            .await
            .unwrap();

        Ok(true)
    }

    /// Create a new user (without ID) in database.
    ///
    /// A new [`User`] instance embedded in the return value contains the latest
    /// `user_id`. So you should always replace existing one.
    pub async fn create(&mut self) -> Result<User, Box<dyn std::error::Error>> {
        if self.user_id.is_some() {
            return Err("User already exists in database, do NOT create again!".into());
        }

        get_connection!();

        let mut key_pairs: Vec<&str> = vec![];
        let mut value_pairs: Vec<String> = vec![];

        if self.user_avatar.is_some() {
            key_pairs.push("`user_avatar`");
            value_pairs.push(format!("'{}'", self.user_avatar.as_ref().unwrap()));
        }

        key_pairs.push("`user_name`");
        value_pairs.push(format!("'{}'", self.user_name));

        key_pairs.push("`user_display_name`");
        value_pairs.push(format!("'{}'", self.user_display_name));

        key_pairs.push("`user_role`");
        value_pairs.push(format!("{}", Into::<i8>::into(self.user_role.clone())));

        key_pairs.push("`user_state`");
        value_pairs.push(format!("{}", Into::<i8>::into(self.user_state.clone())));

        let query_string = format!(
            "INSERT INTO `tbl_user` ({}) VALUES ({})",
            key_pairs.join(", "),
            value_pairs.join(", ")
        );

        log::debug!(target: "sql", "{}", query_string);

        let mut trans = connection.begin().await.unwrap();

        sqlx::query(&query_string)
            .execute(&mut trans)
            .await
            .unwrap();

        let user: Self =
            sqlx::query_as("SELECT * FROM `tbl_user` WHERE `tbl_user`.`user_id`= LAST_INSERT_ID()")
                .fetch_one(&mut trans)
                .await
                .unwrap();

        trans.commit().await.unwrap();

        Auth::new(
            user.user_id.clone().unwrap(),
            &uuid::Uuid::new_v4().to_string(),
        )
        .create()
        .await;

        Ok(user)
    }
}

impl User {
    /// Create a local (not available in database yet) [`User`] instance.
    ///
    /// Note: If you would like to save the user to database, call
    /// [`create()`][`User::create()`] method.
    ///
    /// # Arguments
    /// - `name`: login name of the user
    /// - `display_name`: readable name, customizable by user him(her)-self.
    /// - `role`: role of the user, affect permission level.
    pub fn new(name: &str, display_name: &str, role: &UserRole) -> Self {
        if check_sql_component(name).is_err() || check_sql_component(display_name).is_err() {
            panic!("Invalid parameters");
        }
        Self {
            user_id: None,
            user_name: name.to_string(),
            user_display_name: display_name.to_string(),
            user_avatar: None,
            user_role: role.clone(),
            user_state: UserState::Active,
            user_created_at: Utc::now(),
            user_active_at: Utc::now(),
            __content_updated: true,
            __auth_key: None,
        }
    }

    pub fn id(&self) -> Option<i32> {
        self.user_id.clone()
    }

    pub fn name(&self) -> String {
        self.user_name.clone()
    }

    pub fn display_name(&self) -> String {
        self.user_display_name.clone()
    }

    pub fn avatar(&self) -> Option<String> {
        self.user_avatar.clone()
    }

    pub fn role(&self) -> UserRole {
        self.user_role.clone()
    }

    pub fn state(&self) -> UserState {
        self.user_state.clone()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.user_created_at.clone()
    }

    pub fn active_at(&self) -> DateTime<Utc> {
        self.user_active_at.clone()
    }

    pub async fn auth_key(&self) -> String {
        if self.__auth_key.is_some() {
            return self.__auth_key.clone().unwrap();
        } else if self.__auth_key.is_none() && self.user_id.is_some() {
            // Query from database.
            return self.auth().await.auth_key;
        } else {
            panic!("Auth key unavailable to new user not saved to database")
        }
    }

    pub fn set_avatar(self, avatar: &str) -> Self {
        if check_sql_component(avatar).is_err() {
            panic!("Invalid avatar: `{avatar}`");
        }
        Self {
            user_avatar: Some(avatar.to_string()),
            __content_updated: true,
            ..self
        }
    }

    pub fn set_enable(self) -> Self {
        Self {
            user_state: UserState::Active,
            __content_updated: true,
            ..self
        }
    }

    pub fn set_disable(self) -> Self {
        Self {
            user_state: UserState::Inactive,
            __content_updated: true,
            ..self
        }
    }

    pub fn set_display_name(self, display_name: &str) -> Self {
        if check_sql_component(display_name).is_err() {
            panic!("Invalid display name: `{display_name}`");
        }
        Self {
            user_display_name: display_name.to_string(),
            __content_updated: true,
            ..self
        }
    }
}
