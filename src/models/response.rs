use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompletionResponse {
    pub job_id: String,
    pub error: Option<String>,
}
