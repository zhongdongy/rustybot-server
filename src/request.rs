pub async fn post_remote_stream<T>(endpoint: impl std::fmt::Display, data: &T) -> actix_web::HttpResponse
where
    T: serde::Serialize + ?Sized,
{
    let config = rust_ai::utils::config::Config::load().unwrap();
    let url = format!("{}{}", config.openai.base_endpoint(), endpoint);
    let mut req = reqwest::Client::new().post(url);
    req = req.header("Authorization", format!("Bearer {}", config.openai.api_key));

    let res = req.json(&data).send().await.unwrap();

    let mut resp_builder = actix_web::HttpResponse::Ok();
    for header in res.headers().iter() {
        resp_builder.append_header(header);
    }

    resp_builder.streaming(res.bytes_stream())
}

pub async fn post_remote<T>(endpoint: impl std::fmt::Display, data: &T) -> actix_web::HttpResponse
where
    T: serde::Serialize + ?Sized,
{
    let config = rust_ai::utils::config::Config::load().unwrap();
    let url = format!("{}{}", config.openai.base_endpoint(), endpoint);
    let mut req = reqwest::Client::new().post(url);
    req = req.header("Authorization", format!("Bearer {}", config.openai.api_key));

    let res = req.json(&data).send().await.unwrap();

    let mut resp_builder = actix_web::HttpResponse::Ok();
    for header in res.headers().iter() {
        resp_builder.append_header(header);
    }

    let bytes = res.bytes().await.unwrap();

    resp_builder.body(bytes)
}
