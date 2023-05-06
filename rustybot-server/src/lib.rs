use std::path::PathBuf;

use crate::{
    request::{post_remote, post_remote_stream},
    types::version::VersionInfo,
};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use bytes::Bytes;
use middleware::AuthenticateMiddlewareFactory;
use models::{Chat, Message, User};
use rust_ai::openai::{types::chat_completion::Chunk, ChatCompletion};
use tokio::sync::mpsc::channel;

pub mod auth;
pub mod libs;
pub mod middleware;
pub mod models;
pub mod request;
pub mod types;
pub mod utils;

pub use utils::DB_POOL;

async fn assign_chat_id(req: HttpRequest) -> HttpResponse {
    let user = User::find_by_name(
        req.headers()
            .get("x-rustybot-id")
            .unwrap()
            .to_str()
            .unwrap(),
    )
    .await
    .unwrap()
    .unwrap();

    let chat = Chat::new(user.id().unwrap()).save().await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("{{\"chat_id\": {}}}", chat.chat_id.unwrap()))
}

async fn completions(req: HttpRequest, data: web::Json<ChatCompletion>) -> HttpResponse {
    let endpoint = "/v1/chat/completions";

    let chat_id = if let Some(chat_id_header_val) = req.headers().get("x-rustybot-chat-id") {
        chat_id_header_val.to_str().unwrap()
    } else {
        return HttpResponse::BadRequest().finish();
    };

    // Always save last message to given chat.
    let chat_id = chat_id.parse().unwrap();
    let current_model: models::MessageModel = data.model.clone().into();
    let _new_prompt = Message::new(
        chat_id,
        current_model,
        models::MessageSender::User,
        data.messages.last().unwrap().content.clone(),
        None,
    )
    .save()
    .await
    .unwrap();
    log::debug!(target: "app", "User message ID: `{}` of chat ID `{}` saved to database", _new_prompt.msg_id.unwrap(), chat_id);

    if data.stream.is_none() || data.stream == Some(false) {
        // No stream mode
        post_remote(endpoint, &data, None).await
    } else {
        // Stream mode
        let (sender, mut receiver) = channel::<Bytes>(1024);
        tokio::spawn(async move {
            let mut completion_message = String::new();
            while let Some(bytes) = receiver.recv().await {
                let chunk_data_raw = String::from_utf8(bytes.to_vec()).unwrap();

                if chunk_data_raw == "EOS__EOS" {
                    break;
                }

                log::debug!(
                    target: "openai",
                    "BYTES FROM STREAM AFTER MESSAGE ID `{}`: {}",
                    _new_prompt.msg_id.unwrap(),
                    chunk_data_raw
                );

                for chunk_data in chunk_data_raw.split("\n") {
                    let chunk_data = chunk_data.trim().to_string();
                    if &chunk_data == "data: [DONE]" {
                        log::debug!(target: "openai", "Last chunk received.");
                        break;
                    }
                    if chunk_data.starts_with("data: ") {
                        // Strip response content:
                        let stripped_chunk = &chunk_data.trim()[6..];
                        if let Ok(message_chunk) = serde_json::from_str::<Chunk>(stripped_chunk) {
                            completion_message.push_str(
                                &message_chunk
                                    .choices
                                    .last()
                                    .unwrap()
                                    .delta
                                    .content
                                    .clone()
                                    .unwrap_or(String::new()),
                            );
                        }
                    }
                }
            }
            // Save response to database.
            let _new_prompt = Message::new(
                chat_id,
                current_model,
                models::MessageSender::Assistant,
                completion_message,
                None,
            )
            .save()
            .await
            .unwrap();
            log::debug!(target: "app", "Assistant message ID: `{}` of chat ID `{}` saved to database", _new_prompt.msg_id.unwrap(), chat_id);
        });

        post_remote_stream(endpoint, &data, Some(sender)).await
    }
}

async fn version_info() -> HttpResponse {
    if PathBuf::from("version.yml").exists() {
        let contents = std::fs::read_to_string(PathBuf::from("version.yml")).unwrap();
        let version_info: VersionInfo = serde_yaml::from_str(&contents).unwrap();

        HttpResponse::Ok().json(version_info)
    } else {
        HttpResponse::InternalServerError().finish()
    }
}

async fn verify_authentication() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"result": "pass"}"#)
}

pub async fn create_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/info").route("/version", web::get().to(version_info)))
            .service(
                web::scope("/v1")
                    .wrap(AuthenticateMiddlewareFactory::new())
                    .route("/chat/completions", web::post().to(completions))
                    .route("/chat/new", web::post().to(assign_chat_id)),
            )
            .service(
                web::scope("/auth")
                    .wrap(AuthenticateMiddlewareFactory::new())
                    .route("/verify", web::get().to(verify_authentication)),
            )
    })
    .bind(("0.0.0.0", 9090))?
    .run()
    .await
}
