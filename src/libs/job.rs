use std::error::Error;

use log::error;
use rust_ai::openai::{
    types::chat_completion::{ChatMessage, Chunk},
    ChatCompletion,
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::utils::config::Config;

use super::mqtt::Publisher;

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
    current_index: usize,
}

impl Job {
    fn mqtt_topic(&self) -> String {
        let config = Config::load().unwrap();
        format!(
            "{}{}",
            config.mqtt.topic_prefix.unwrap_or(String::new()),
            self.job_id
        )
    }

    pub async fn completion(&mut self) -> Result<Vec<Chunk>, Box<dyn Error>> {
        Ok(ChatCompletion::default()
            .messages(self.prompts.clone())
            .streamed_completion(Some(|chunk: Chunk| {
                let choice = chunk.choices.get(0).unwrap();
                if let Some(message) = choice.delta.content.clone() {
                    if let Err(e) = Publisher::new()
                        .unwrap()
                        .publish_message(self.produce_message(message).to_json(), self.mqtt_topic())
                    {
                        error!(target: "app", "Unable to post message chunk to MQTT: {:?}", e);
                    }
                }
            }))
            .await?)
    }

    fn produce_message(&mut self, message: String) -> JobMessage {
        self.current_index += 1;
        JobMessage {
            job_id: self.job_id.clone(),
            message_chunk: message,
            index: self.current_index - 1,
        }
    }
}

pub struct JobBuilder {
    job_id: String,
    prompts: Vec<ChatMessage>,
}

impl JobBuilder {
    pub fn new(id: String) -> Self {
        Self {
            job_id: id,
            prompts: vec![],
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
            current_index: 0,
        }
    }
}
