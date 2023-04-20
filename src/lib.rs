use actix_web::{post, web, App, HttpResponse, HttpServer};
use rust_ai::openai::ChatCompletion;

#[post("/v1/chat/completions")]
async fn completions(data: web::Json<ChatCompletion>) -> HttpResponse {
    let client = reqwest::Client::new();
    let config = rust_ai::utils::config::Config::load().unwrap();
    let endpoint = "/v1/chat/completions";
    let url = format!("{}{}", config.openai.base_endpoint(), endpoint);
    let mut req = client.post(url);
    req = req.header("Authorization", format!("Bearer {}", config.openai.api_key));

    let res = req.json(&data).send().await.unwrap();

    let mut resp_builder = HttpResponse::Ok();
    for header in res.headers().iter() {
        resp_builder.append_header(header);
    }

    resp_builder.streaming(res.bytes_stream())
}

pub async fn create_server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(completions))
        .bind(("0.0.0.0", 9090))?
        .run()
        .await
}
