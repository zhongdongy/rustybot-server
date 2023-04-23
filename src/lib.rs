use std::path::PathBuf;

use actix_web::{web, App, HttpResponse, HttpServer};
use middleware::AuthenticateMiddlewareFactory;
use rust_ai::openai::ChatCompletion;

use crate::{
    request::{post_remote, post_remote_stream},
    types::version::VersionInfo,
};

pub mod auth;
pub mod middleware;
pub mod request;
pub mod types;

async fn completions(data: web::Json<ChatCompletion>) -> HttpResponse {
    let endpoint = "/v1/chat/completions";
    if data.stream.is_none() || data.stream == Some(false) {
        // No stream mode
        post_remote(endpoint, &data).await
    } else {
        // Stream mode
        post_remote_stream(endpoint, &data).await
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
                    .route("/chat/completions", web::post().to(completions)),
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
