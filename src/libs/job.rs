use std::error::Error;

use atc::ChannelCommand;
use rust_ai::openai::{
    types::chat_completion::{ChatMessage, Chunk},
    ChatCompletion,
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use std::sync::mpsc::Sender;

// use crate::utils::config::Config;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct JobMessage {
    job_id: String,
    message_chunk: String,
    index: usize,
}

impl JobMessage {
    pub fn to_json(&self) -> String {
        to_string(self).unwrap()
    }
}

pub struct Job {
    pub job_id: String,
    prompts: Vec<ChatMessage>,
    sender: Sender<ChannelCommand>,
}

impl Job {
    pub async fn completion(&mut self) -> Result<Vec<Chunk>, Box<dyn Error>> {
        Ok(ChatCompletion::default()
            .messages(self.prompts.clone())
            .streamed_completion(Some(|chunk: Chunk| {
                let choice = chunk.choices.get(0).unwrap();
                if let Some(message) = choice.delta.content.clone() {
                    self.sender
                        .send(ChannelCommand::ChannelMessage((
                            self.job_id.clone(),
                            message,
                        )))
                        .unwrap();
                }
            }))
            .await?)
    }
}

pub struct JobBuilder {
    job_id: String,
    prompts: Vec<ChatMessage>,
    sender: Sender<ChannelCommand>,
}

impl JobBuilder {
    pub fn new(id: String, sender: Sender<ChannelCommand>) -> Self {
        Self {
            job_id: id,
            prompts: vec![],
            sender,
        }
    }

    pub fn set_messages(self, messages: Vec<ChatMessage>) -> Self {
        Self {
            prompts: messages,
            ..self
        }
    }

    pub fn finalize(self) -> Job {
        Job {
            job_id: self.job_id,
            prompts: self.prompts,
            sender: self.sender,
        }
    }
}
