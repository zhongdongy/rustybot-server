use actix_web::{post, web, App, HttpResponse, HttpServer};
use middleware::AuthenticateMiddlewareFactory;
use rust_ai::openai::ChatCompletion;

use crate::request::{post_remote, post_remote_stream};

pub mod middleware;
pub mod request;
pub mod auth;

#[post("/v1/chat/completions")]
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

pub async fn create_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(AuthenticateMiddlewareFactory::new())
            .service(completions)
    })
    .bind(("0.0.0.0", 9090))?
    .run()
    .await
}
