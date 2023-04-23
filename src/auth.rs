use sha2::{Digest, Sha512};
use std::path::PathBuf;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct AuthUser {
    pub id: String,
    pub key: String,
    pub note: Option<String>,
}

impl AuthUser {
    pub fn hash(&self, salt: &str) -> String {
        let mut buf = [0u8; 1024];
        let input = format!("{}{}{}", self.id, self.key, salt);
        let mut hasher: Sha512 = Sha512::new();
        hasher.update(input.as_bytes());
        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_str(&hash, &mut buf).unwrap();
        hex_hash.to_string()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct AuthConfig {
    pub users: Vec<AuthUser>,
}

impl AuthConfig {
    pub fn find(&self, id: &str) -> Option<AuthUser> {
        for user in self.users.iter() {
            if user.id == id {
                return Some(user.clone());
            }
        }
        None
    }
}

pub fn auth_with_file(id: &str, hash: &str, salt: &str) -> bool {
    let auth_path = PathBuf::from("auth.yml");
    if auth_path.exists() {
        let auth_contents = std::fs::read_to_string(auth_path.as_path()).unwrap();
        if let Ok(auth) = serde_yaml::from_str::<AuthConfig>(&auth_contents) {
            if let Some(user) = auth.find(id) {
                if user.hash(salt) == hash {
                    log::debug!(target: "app", "Authentication passed");
                    return true;
                }
            }
        }
    }
    log::warn!(target: "app", "Authentication failed");
    return false;
}
