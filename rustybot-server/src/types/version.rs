#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Version {
    /// E.g. 1.0.0
    pub version: String,

    /// Release date, "yyyy-MM-DD"
    pub date: String,

    /// Description
    pub description: String,

    /// Release (distribution) URLs
    pub releases: Option<ReleaseInfo>,

    /// Download URL
    pub url: String,
}
impl Default for Version {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            date: "1995-09-26".to_string(),
            description: String::new(),
            url: String::new(),
            releases: None,
        }
    }
}

#[derive(Default, Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ReleaseInfo {
    pub msi_x64: Option<String>,
    pub deb_arm64: Option<String>,
    pub deb_x64: Option<String>,
}

#[derive(Default, Debug, serde::Deserialize, serde::Serialize)]
pub struct VersionInfo {
    pub latest: Version,
    pub history: Vec<Version>,
}
