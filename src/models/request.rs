use rust_ai::openai::types::chat_completion::ChatMessage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompletionRequest {
    pub prompts: Vec<ChatMessage>,
}