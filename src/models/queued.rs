use rust_ai::openai::types::chat_completion::ChatMessage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct QueuedCompletionJob {
    pub job_id: String,
    pub prompts: Vec<ChatMessage>,
}
